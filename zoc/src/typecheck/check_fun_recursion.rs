use super::*;

use std::ops::BitOr;

#[derive(Clone, Copy)]
pub enum RecursionCheckingContext<'a, A: AuxDataFamily> {
    Base(&'a [UnshiftedEntry<'a, A>]),
    Snoc(
        &'a RecursionCheckingContext<'a, A>,
        &'a [UnshiftedEntry<'a, A>],
    ),
}

impl<A: AuxDataFamily> RecursionCheckingContext<'static, A> {
    pub fn empty() -> Self {
        RecursionCheckingContext::Base(&[])
    }
}

#[derive(Clone)]
pub struct UnshiftedEntry<'a, A: AuxDataFamily>(pub Entry<'a, A>);

#[derive(Clone)]
pub enum Entry<'a, A: AuxDataFamily> {
    Top(Option<&'a ast::Fun<A>>),
    Substruct(SizeBound, Strict),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Strict(pub bool);

#[derive(Clone, Copy, Debug)]
pub enum SizeBound {
    Deb(Deb),
    CaselessMatch,
}

enum CallRequirement<'a, A: AuxDataFamily> {
    Recursive(RecursiveCallRequirement<'a, A>),
    AccessForbidden(&'a ast::Fun<A>),
}

#[derive(Clone)]
struct RecursiveCallRequirement<'a, A: AuxDataFamily> {
    arg_index: usize,
    strict_superstruct: Deb,
    definition_src: &'a ast::Fun<A>,
}

impl TypeChecker {
    pub(crate) fn check_recursion<A: AuxDataFamily>(
        &mut self,
        expr: ast::Expr<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        match expr {
            ast::Expr::Ind(e) => self.check_recursion_in_ind(&e.hashee, rcon),
            ast::Expr::Vcon(e) => self.check_recursion_in_vcon(&e.hashee, rcon),
            ast::Expr::Match(e) => self.check_recursion_in_match(&e.hashee, rcon),
            ast::Expr::Fun(e) => self.check_recursion_in_fun(&e.hashee, None, rcon),
            ast::Expr::App(e) => self.check_recursion_in_app(&e.hashee, rcon),
            ast::Expr::For(e) => self.check_recursion_in_for(&e.hashee, rcon),
            ast::Expr::Deb(e) => self.check_recursion_in_deb(&e.hashee, rcon),
            ast::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_recursion_in_ind<A: AuxDataFamily>(
        &mut self,
        ind: &ast::Ind<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion_in_dependent_exprs(&ind.index_types.hashee, rcon)?;

        let singleton = vec![UnshiftedEntry(Entry::Top(None))];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &singleton);
        self.check_recursion_in_vcon_defs(&ind.vcon_defs.hashee, extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_vcon_defs<A: AuxDataFamily>(
        &mut self,
        defs: &[ast::VconDef<A>],
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        for def in defs {
            self.check_recursion_in_vcon_def(def, rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_vcon_def<A: AuxDataFamily>(
        &mut self,
        def: &ast::VconDef<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion_in_dependent_exprs(&def.param_types.hashee, rcon)?;

        let extension = vec![UnshiftedEntry(Entry::Top(None)); def.param_types.hashee.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion_in_independent_exprs(&def.index_args.hashee, extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_vcon<A: AuxDataFamily>(
        &mut self,
        vcon: &ast::Vcon<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion_in_ind(&vcon.ind.hashee, rcon)
    }

    fn check_recursion_in_match<A: AuxDataFamily>(
        &mut self,
        match_: &ast::Match<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion(match_.matchee.clone(), rcon)?;

        let matchee_bound = self.get_size_bound(match_.matchee.clone(), rcon);
        self.check_recursion_in_match_cases(&match_.cases.hashee, matchee_bound, rcon)?;

        Ok(())
    }

    fn check_recursion_in_match_cases<A: AuxDataFamily>(
        &mut self,
        cases: &[ast::MatchCase<A>],
        matchee_bound: Option<SizeBound>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        for case in cases {
            self.check_recursion_in_match_case(case, matchee_bound, rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_match_case<A: AuxDataFamily>(
        &mut self,
        case: &ast::MatchCase<A>,
        matchee_bound: Option<SizeBound>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        let extension = get_rcon_extension_for_match_case_params(matchee_bound, case.arity);
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(case.return_val.clone(), extended_rcon)?;
        Ok(())
    }

    fn check_recursion_in_fun<A: AuxDataFamily>(
        &mut self,
        fun: &ast::Fun<A>,
        app_arg_status: Option<Vec<UnshiftedEntry<A>>>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion_in_dependent_exprs(&fun.param_types.hashee, rcon)?;
        self.check_recursion_in_fun_return_type(fun, rcon)?;

        let extension = self.get_fun_rcon_extension(fun, app_arg_status)?;
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);

        self.check_recursion(fun.return_val.clone(), extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_fun_return_type<A: AuxDataFamily>(
        &mut self,
        fun: &ast::Fun<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        let extension = vec![UnshiftedEntry(Entry::Top(None)); fun.param_types.hashee.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(fun.return_type.clone(), extended_rcon)?;
        Ok(())
    }

    fn get_fun_rcon_extension<'a, A: AuxDataFamily>(
        &mut self,
        fun: &'a ast::Fun<A>,
        app_arg_status: Option<Vec<UnshiftedEntry<'a, A>>>,
    ) -> Result<Vec<UnshiftedEntry<'a, A>>, TypeError<A>> {
        self.assert_decreasing_index_is_valid(fun)?;
        let fun_entry = UnshiftedEntry(Entry::Top(Some(fun)));
        let param_entries = self.get_fun_param_entries(fun, app_arg_status);

        let mut out = param_entries;
        out.push(fun_entry);
        Ok(out)
    }

    fn assert_decreasing_index_is_valid<A: AuxDataFamily>(
        &mut self,
        fun: &ast::Fun<A>,
    ) -> Result<(), TypeError<A>> {
        match &fun.decreasing_index {
            Some(decreasing_arg_index) => {
                if *decreasing_arg_index >= fun.param_types.hashee.len() {
                    return Err(TypeError::DecreasingArgIndexTooBig { fun: fun.clone() });
                }

                Ok(())
            }

            None => Ok(()),
        }
    }

    fn get_fun_param_entries<'a, A: AuxDataFamily>(
        &mut self,
        fun: &'a ast::Fun<A>,
        app_arg_status: Option<Vec<UnshiftedEntry<'a, A>>>,
    ) -> Vec<UnshiftedEntry<'a, A>> {
        app_arg_status
            .unwrap_or_else(|| vec![UnshiftedEntry(Entry::Top(None)); fun.param_types.hashee.len()])
    }

    fn check_recursion_in_app<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion_in_app_callee(app, rcon)?;
        self.check_recursion_in_independent_exprs(&app.args.hashee, rcon)?;
        Ok(())
    }

    fn check_recursion_in_app_callee<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        match &app.callee {
            ast::Expr::Deb(callee) => {
                self.check_recursion_in_app_callee_deb(app, &callee.hashee, rcon)
            }

            ast::Expr::Fun(callee) => {
                self.check_recursion_in_app_callee_fun(app, &callee.hashee, rcon)
            }

            _ => self.check_recursion(app.callee.clone(), rcon),
        }
    }

    fn check_recursion_in_app_callee_deb<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        callee: &ast::DebNode<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        let requirement = rcon.get_call_requirement(callee.deb);

        match requirement {
            Some(CallRequirement::Recursive(requirement)) => {
                self.assert_arg_satisfies_recursive_call_requirement(app, requirement, rcon)
            }

            Some(CallRequirement::AccessForbidden(definition_src)) => {
                Err(TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                    deb: callee.clone(),
                    definition_src: definition_src.clone(),
                })
            }

            None => self.check_recursion(app.callee.clone(), rcon),
        }
    }

    fn check_recursion_in_app_callee_fun<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        callee: &ast::Fun<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        let arg_status = self.get_app_callee_fun_arg_status(app, callee, rcon);
        self.check_recursion_in_fun(&callee, Some(arg_status), rcon)
    }

    fn get_app_callee_fun_arg_status<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        callee: &ast::Fun<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Vec<UnshiftedEntry<'static, A>> {
        match &callee.decreasing_index {
            None => self.get_app_callee_nonrecursive_fun_arg_status(app, rcon),

            Some(decreasing_index) => {
                self.get_app_callee_recursive_fun_arg_status(app, *decreasing_index, rcon)
            }
        }
    }

    fn get_app_callee_nonrecursive_fun_arg_status<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Vec<UnshiftedEntry<'static, A>> {
        app.args
            .hashee
            .iter()
            .enumerate()
            .map(|(arg_index, arg)| {
                let bound = self.get_size_bound(arg.clone(), rcon);
                match bound {
                    None => UnshiftedEntry(Entry::Top(None)),

                    Some(SizeBound::CaselessMatch) => {
                        UnshiftedEntry(Entry::Substruct(SizeBound::CaselessMatch, Strict(true)))
                    }

                    Some(SizeBound::Deb(superstruct)) => UnshiftedEntry(Entry::Substruct(
                        SizeBound::Deb(Deb(superstruct.0 + arg_index)),
                        Strict(false),
                    )),
                }
            })
            .collect()
    }

    fn get_app_callee_recursive_fun_arg_status<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        decreasing_index: usize,
        rcon: RecursionCheckingContext<A>,
    ) -> Vec<UnshiftedEntry<'static, A>> {
        app.args
            .hashee
            .iter()
            .enumerate()
            .map(|(arg_index, arg)| {
                if arg_index == decreasing_index {
                    let bound = self.get_size_bound(arg.clone(), rcon);
                    if let Some(bound) = bound {
                        let bound = bound.upshift(arg_index);
                        return UnshiftedEntry(Entry::Substruct(bound, Strict(false)));
                    }
                }

                UnshiftedEntry(Entry::Top(None))
            })
            .collect()
    }

    fn assert_arg_satisfies_recursive_call_requirement<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        requirement: RecursiveCallRequirement<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        if requirement.arg_index >= app.args.hashee.len() {
            // Do nothing.
            //
            // The user-provided decreasing index is either invalid
            // or the number of arguments is illegal.
            // In either case, this is a type error that will be
            // caught elsewhere in the typechecking process.
            // We don't want to return an error _here_ because
            // that would complicated this code.
            // Such complication is not necessary because
            // the other typechecking code will catch the error.
            return Ok(());
        }

        let arg = &app.args.hashee[requirement.arg_index];
        if !self.is_strict_substruct(arg.clone(), requirement.strict_superstruct, rcon) {
            return Err(TypeError::IllegalRecursiveCall {
                app: app.clone(),
                callee_deb_definition_src: requirement.definition_src.clone(),
                required_decreasing_arg_index: requirement.arg_index,
                required_strict_superstruct: requirement.strict_superstruct,
            });
        }

        Ok(())
    }

    fn check_recursion_in_for<A: AuxDataFamily>(
        &mut self,
        for_: &ast::For<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        self.check_recursion_in_dependent_exprs(&for_.param_types.hashee, rcon)?;

        let extension = vec![UnshiftedEntry(Entry::Top(None)); for_.param_types.hashee.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(for_.return_type.clone(), extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_deb<A: AuxDataFamily>(
        &mut self,
        deb: &ast::DebNode<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        if let Some(requirement) = rcon.get_call_requirement(deb.deb) {
            let err = match requirement {
                CallRequirement::Recursive(requirement) => {
                    TypeError::RecursiveFunParamInNonCalleePosition {
                        deb: deb.clone(),
                        definition_src: requirement.definition_src.clone(),
                    }
                }

                CallRequirement::AccessForbidden(definition_src) => {
                    TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                        deb: deb.clone(),
                        definition_src: definition_src.clone(),
                    }
                }
            };
            return Err(err);
        }

        Ok(())
    }
}

impl TypeChecker {
    fn check_recursion_in_dependent_exprs<A: AuxDataFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        if exprs.is_empty() {
            return Ok(());
        }

        let rcon_extension = vec![UnshiftedEntry(Entry::Top(None)); exprs.len() - 1];

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &rcon_extension[..i]);
            self.check_recursion(expr, extended_rcon)?;
        }

        Ok(())
    }

    fn check_recursion_in_independent_exprs<A: AuxDataFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        rcon: RecursionCheckingContext<A>,
    ) -> Result<(), TypeError<A>> {
        for expr in exprs {
            self.check_recursion(expr.clone(), rcon)?;
        }
        Ok(())
    }
}

impl TypeChecker {
    fn is_strict_substruct<A: AuxDataFamily>(
        &mut self,
        expr: ast::Expr<A>,
        possible_superstruct: Deb,
        rcon: RecursionCheckingContext<A>,
    ) -> bool {
        let bound = self.get_size_bound(expr, rcon);
        match bound {
            None => false,

            Some(SizeBound::CaselessMatch) => true,

            Some(SizeBound::Deb(bound_deb)) => {
                if let Some(strict) = is_deb_substruct(bound_deb, possible_superstruct, rcon) {
                    strict.0
                } else {
                    false
                }
            }
        }
    }
}

impl TypeChecker {
    fn get_size_bound<A: AuxDataFamily>(
        &mut self,
        expr: ast::Expr<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Option<SizeBound> {
        match expr {
            ast::Expr::Ind(_)
            | ast::Expr::Vcon(_)
            | ast::Expr::Fun(_)
            | ast::Expr::App(_)
            | ast::Expr::For(_)
            | ast::Expr::Universe(_) => None,

            ast::Expr::Match(e) => self.get_size_bound_of_match(&e.hashee, rcon),

            ast::Expr::Deb(e) => self.get_size_bound_of_deb(&e.hashee, rcon),
        }
    }

    fn get_size_bound_of_match<A: AuxDataFamily>(
        &mut self,
        expr: &ast::Match<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Option<SizeBound> {
        if expr.cases.hashee.is_empty() {
            return Some(SizeBound::CaselessMatch);
        }

        let matchee_bound = self.get_size_bound(expr.matchee.clone(), rcon);

        let mut lowest_common_bound =
            self.get_size_bound_of_match_case(&expr.cases.hashee[0], matchee_bound, rcon)?;

        for case in &expr.cases.hashee[1..] {
            let case_bound =
                self.get_size_bound_of_match_case(case, matchee_bound.clone(), rcon)?;

            lowest_common_bound = get_min_size_bound(case_bound, lowest_common_bound, rcon)?;
        }

        Some(lowest_common_bound)
    }

    fn get_size_bound_of_match_case<A: AuxDataFamily>(
        &mut self,
        expr: &ast::MatchCase<A>,
        matchee_bound: Option<SizeBound>,
        rcon: RecursionCheckingContext<A>,
    ) -> Option<SizeBound> {
        let extension = get_rcon_extension_for_match_case_params(matchee_bound, expr.arity);
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.get_size_bound(expr.return_val.clone(), extended_rcon)
    }

    fn get_size_bound_of_deb<A: AuxDataFamily>(
        &mut self,
        expr: &ast::DebNode<A>,
        rcon: RecursionCheckingContext<A>,
    ) -> Option<SizeBound> {
        let entry = rcon.get(expr.deb)?;
        match entry {
            Entry::Substruct(SizeBound::CaselessMatch, _) => Some(SizeBound::CaselessMatch),

            Entry::Substruct(SizeBound::Deb(_), _) | Entry::Top(_) => {
                Some(SizeBound::Deb(expr.deb))
            }
        }
    }
}

fn get_rcon_extension_for_match_case_params<A: AuxDataFamily>(
    matchee_bound: Option<SizeBound>,
    case_arity: usize,
) -> Vec<UnshiftedEntry<'static, A>> {
    let Some(matchee_bound) = matchee_bound else {
        return vec![UnshiftedEntry(Entry::Top(None)); case_arity];
    };

    (0..case_arity)
        .map(|case_param_index| {
            UnshiftedEntry(Entry::Substruct(
                matchee_bound.upshift(case_param_index),
                Strict(true),
            ))
        })
        .collect()
}

fn get_min_size_bound<A: AuxDataFamily>(
    a: SizeBound,
    b: SizeBound,
    rcon: RecursionCheckingContext<A>,
) -> Option<SizeBound> {
    match (a, b) {
        (SizeBound::CaselessMatch, b) => Some(b),

        (a, SizeBound::CaselessMatch) => Some(a),

        (SizeBound::Deb(a), SizeBound::Deb(b)) => get_min_size_bound_of_debs(a, b, rcon),
    }
}

fn get_min_size_bound_of_debs<A: AuxDataFamily>(
    a: Deb,
    b: Deb,
    rcon: RecursionCheckingContext<A>,
) -> Option<SizeBound> {
    get_smallest_superstruct_of_a_that_is_also_a_superstruct_of_b(a, b, rcon)
        .or_else(|| get_smallest_superstruct_of_a_that_is_also_a_superstruct_of_b(b, a, rcon))
}

fn get_smallest_superstruct_of_a_that_is_also_a_superstruct_of_b<A: AuxDataFamily>(
    a: Deb,
    b: Deb,
    rcon: RecursionCheckingContext<A>,
) -> Option<SizeBound> {
    let mut a_superstruct = a;

    let bounding_deb = loop {
        if is_deb_substruct(b, a_superstruct, rcon).is_some() {
            break a_superstruct;
        }

        let entry = rcon.get(a_superstruct)?;
        match entry {
            Entry::Top(_) => return None,

            Entry::Substruct(SizeBound::CaselessMatch, _) => break b,

            Entry::Substruct(SizeBound::Deb(a_superstruct_superstruct), _) => {
                a_superstruct = a_superstruct_superstruct;
            }
        }
    };

    Some(SizeBound::Deb(bounding_deb))
}

/// If `deb` is a substruct `possible_superstruct`,
/// this function returns `Some(strictness)`.
/// Otherwise, it returns `None`.
fn is_deb_substruct<A: AuxDataFamily>(
    deb: Deb,
    possible_superstruct: Deb,
    rcon: RecursionCheckingContext<A>,
) -> Option<Strict> {
    if deb == possible_superstruct {
        return Some(Strict(false));
    }

    let entry = rcon.get(deb)?;

    match entry {
        Entry::Top(_) => None,

        Entry::Substruct(SizeBound::CaselessMatch, _) => Some(Strict(true)),

        Entry::Substruct(SizeBound::Deb(direct_superstruct), strict) => {
            is_deb_substruct(direct_superstruct, possible_superstruct, rcon)
                .map(|direct_strict| strict | direct_strict)
        }
    }
}

impl<A: AuxDataFamily> RecursionCheckingContext<'_, A> {
    fn get_call_requirement(&self, deb: Deb) -> Option<CallRequirement<A>> {
        let entry = self.get(deb)?;
        match entry {
            Entry::Top(Some(fun)) => match fun.decreasing_index {
                Some(decreasing_index) => {
                    Some(CallRequirement::Recursive(RecursiveCallRequirement {
                        arg_index: decreasing_index,
                        strict_superstruct: Deb(
                            deb.0 + fun.param_types.hashee.len() - decreasing_index
                        ),
                        definition_src: fun,
                    }))
                }

                None => Some(CallRequirement::AccessForbidden(fun)),
            },

            Entry::Top(None) | Entry::Substruct(_, _) => None,
        }
    }

    fn get(&self, deb: Deb) -> Option<Entry<A>> {
        let unshifted = self.get_unshifted(deb)?;
        Some(unshifted.0.upshift(deb.0 + 1))
    }

    fn get_unshifted(&self, deb: Deb) -> Option<UnshiftedEntry<A>> {
        match self {
            RecursionCheckingContext::Base(entries) => {
                let index = (entries.len()).checked_sub(1 + deb.0)?;
                Some(entries.get(index)?.clone())
            }

            RecursionCheckingContext::Snoc(subcontext, types) => {
                if let Some(index) = (types.len()).checked_sub(1 + deb.0) {
                    Some(types.get(index)?.clone())
                } else {
                    subcontext.get_unshifted(Deb(deb.0 - types.len()))
                }
            }
        }
    }
}

impl<A: AuxDataFamily> Entry<'_, A> {
    fn upshift(self, upshift_amount: usize) -> Self {
        match self {
            Entry::Top(_) => self,

            Entry::Substruct(bound, strict) => {
                Entry::Substruct(bound.upshift(upshift_amount), strict)
            }
        }
    }
}

impl SizeBound {
    fn upshift(self, upshift_amount: usize) -> Self {
        match self {
            SizeBound::CaselessMatch => SizeBound::CaselessMatch,

            SizeBound::Deb(deb) => SizeBound::Deb(Deb(deb.0 + upshift_amount)),
        }
    }
}

impl BitOr for Strict {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Strict(self.0 | rhs.0)
    }
}
