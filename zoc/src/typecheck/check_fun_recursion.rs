use super::*;

use std::ops::BitOr;

#[derive(Clone, Copy, Debug)]
pub enum RecursionCheckingContext<'a> {
    Base(&'a [UnshiftedEntry<'a>]),
    Snoc(&'a RecursionCheckingContext<'a>, &'a [UnshiftedEntry<'a>]),
}

impl RecursionCheckingContext<'static> {
    pub fn empty() -> Self {
        RecursionCheckingContext::Base(&[])
    }
}

#[derive(Clone, Debug)]
pub struct UnshiftedEntry<'a>(pub Entry<'a>);

#[derive(Clone, Debug)]
pub enum Entry<'a> {
    Top(Option<&'a cst::Fun>),
    Substruct(SizeBound, Strict),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Strict(pub bool);

#[derive(Clone, Copy, Debug)]
pub enum SizeBound {
    Deb(Deb),
    CaselessMatch,
}

enum CallRequirement<'a> {
    Recursive(RecursiveCallRequirement<'a>),
    AccessForbidden(&'a cst::Fun),
}

#[derive(Clone)]
struct RecursiveCallRequirement<'a> {
    arg_index: usize,
    strict_superstruct: Deb,
    definition_src: &'a cst::Fun,
}

impl TypeChecker {
    pub(crate) fn check_recursion(
        &mut self,
        expr: cst::Expr,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        match expr {
            cst::Expr::Ind(e) => self.check_recursion_in_ind(&e.hashee, rcon),
            cst::Expr::Vcon(e) => self.check_recursion_in_vcon(&e.hashee, rcon),
            cst::Expr::Match(e) => self.check_recursion_in_match(&e.hashee, rcon),
            cst::Expr::Fun(e) => self.check_recursion_in_fun(&e.hashee, None, rcon),
            cst::Expr::App(e) => self.check_recursion_in_app(&e.hashee, rcon),
            cst::Expr::For(e) => self.check_recursion_in_for(&e.hashee, rcon),
            cst::Expr::Deb(e) => self.check_recursion_in_deb(&e.hashee, rcon),
            cst::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_recursion_in_ind(
        &mut self,
        ind: &cst::Ind,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&ind.index_types, rcon)?;

        let singleton = vec![UnshiftedEntry(Entry::Top(None))];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &singleton);
        self.check_recursion_in_vcon_defs(&ind.vcon_defs, extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_vcon_defs(
        &mut self,
        defs: &[cst::VconDef],
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for def in defs {
            self.check_recursion_in_vcon_def(def, rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_vcon_def(
        &mut self,
        def: &cst::VconDef,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&def.param_types, rcon)?;

        let extension = vec![UnshiftedEntry(Entry::Top(None)); def.param_types.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion_in_independent_exprs(&def.index_args, extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_vcon(
        &mut self,
        vcon: &cst::Vcon,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_ind(&vcon.ind.hashee, rcon)
    }

    fn check_recursion_in_match(
        &mut self,
        match_: &cst::Match,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion(match_.matchee.clone(), rcon)?;

        let matchee_bound = self.get_size_bound(match_.matchee.clone(), rcon);
        self.check_recursion_in_match_cases(&match_.cases, matchee_bound, rcon)?;

        Ok(())
    }

    fn check_recursion_in_match_cases(
        &mut self,
        cases: &[cst::MatchCase],
        matchee_bound: Option<SizeBound>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for case in cases {
            self.check_recursion_in_match_case(case, matchee_bound, rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_match_case(
        &mut self,
        case: &cst::MatchCase,
        matchee_bound: Option<SizeBound>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let extension = get_rcon_extension_for_match_case_params(matchee_bound, case.arity.value);
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(case.return_val.clone(), extended_rcon)?;
        Ok(())
    }

    fn check_recursion_in_fun(
        &mut self,
        fun: &cst::Fun,
        app_arg_status: Option<Vec<UnshiftedEntry>>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&fun.param_types, rcon)?;
        self.check_recursion_in_fun_return_type(fun, rcon)?;

        let extension = self.get_fun_rcon_extension(fun, app_arg_status)?;
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);

        self.check_recursion(fun.return_val.clone(), extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_fun_return_type(
        &mut self,
        fun: &cst::Fun,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let extension = vec![UnshiftedEntry(Entry::Top(None)); fun.param_types.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(fun.return_type.clone(), extended_rcon)?;
        Ok(())
    }

    fn get_fun_rcon_extension<'a>(
        &mut self,
        fun: &'a cst::Fun,
        app_arg_status: Option<Vec<UnshiftedEntry<'a>>>,
    ) -> Result<Vec<UnshiftedEntry<'a>>, TypeError> {
        self.assert_decreasing_index_is_valid(fun)?;
        let fun_entry = UnshiftedEntry(Entry::Top(Some(fun)));
        let param_entries = self.get_fun_param_entries(fun, app_arg_status);

        let mut out = param_entries;
        out.push(fun_entry);
        Ok(out)
    }

    fn assert_decreasing_index_is_valid(&mut self, fun: &cst::Fun) -> Result<(), TypeError> {
        match &fun.decreasing_index {
            cst::NumberOrNonrecKw::Number(decreasing_index_literal) => {
                let decreasing_arg_index = decreasing_index_literal.value;
                if decreasing_arg_index >= fun.param_types.len() {
                    return Err(TypeError::DecreasingArgIndexTooBig { fun: fun.clone() });
                }

                Ok(())
            }

            cst::NumberOrNonrecKw::NonrecKw(_) => Ok(()),
        }
    }

    fn get_fun_param_entries<'a>(
        &mut self,
        fun: &'a cst::Fun,
        app_arg_status: Option<Vec<UnshiftedEntry<'a>>>,
    ) -> Vec<UnshiftedEntry<'a>> {
        app_arg_status
            .unwrap_or_else(|| vec![UnshiftedEntry(Entry::Top(None)); fun.param_types.len()])
    }

    fn check_recursion_in_app(
        &mut self,
        app: &cst::App,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_app_callee(app, rcon)?;
        self.check_recursion_in_independent_exprs(&app.args, rcon)?;
        Ok(())
    }

    fn check_recursion_in_app_callee(
        &mut self,
        app: &cst::App,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        match &app.callee {
            cst::Expr::Deb(callee) => {
                self.check_recursion_in_app_callee_deb(app, &callee.hashee, rcon)
            }

            cst::Expr::Fun(callee) => {
                self.check_recursion_in_app_callee_fun(app, &callee.hashee, rcon)
            }

            _ => self.check_recursion(app.callee.clone(), rcon),
        }
    }

    fn check_recursion_in_app_callee_deb(
        &mut self,
        app: &cst::App,
        callee: &cst::NumberLiteral,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let callee_deb = Deb(callee.value);
        let requirement = rcon.get_call_requirement(callee_deb);

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

    fn check_recursion_in_app_callee_fun(
        &mut self,
        app: &cst::App,
        callee: &cst::Fun,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let arg_status = self.get_app_callee_fun_arg_status(app, callee, rcon);
        self.check_recursion_in_fun(&callee, Some(arg_status), rcon)
    }

    fn get_app_callee_fun_arg_status(
        &mut self,
        app: &cst::App,
        callee: &cst::Fun,
        rcon: RecursionCheckingContext,
    ) -> Vec<UnshiftedEntry<'static>> {
        match &callee.decreasing_index {
            cst::NumberOrNonrecKw::NonrecKw(_) => {
                self.get_app_callee_nonrecursive_fun_arg_status(app, rcon)
            }

            cst::NumberOrNonrecKw::Number(decreasing_index_literal) => self
                .get_app_callee_recursive_fun_arg_status(app, decreasing_index_literal.value, rcon),
        }
    }

    fn get_app_callee_nonrecursive_fun_arg_status(
        &mut self,
        app: &cst::App,
        rcon: RecursionCheckingContext,
    ) -> Vec<UnshiftedEntry<'static>> {
        app.args
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

    fn get_app_callee_recursive_fun_arg_status(
        &mut self,
        app: &cst::App,
        decreasing_index: usize,
        rcon: RecursionCheckingContext,
    ) -> Vec<UnshiftedEntry<'static>> {
        app.args
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

    fn assert_arg_satisfies_recursive_call_requirement(
        &mut self,
        app: &cst::App,
        requirement: RecursiveCallRequirement,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        if requirement.arg_index >= app.args.len() {
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

        let arg = &app.args[requirement.arg_index];
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

    fn check_recursion_in_for(
        &mut self,
        for_: &cst::For,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&for_.param_types, rcon)?;

        let extension = vec![UnshiftedEntry(Entry::Top(None)); for_.param_types.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(for_.return_type.clone(), extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_deb(
        &mut self,
        deb: &cst::NumberLiteral,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        if let Some(requirement) = rcon.get_call_requirement(Deb(deb.value)) {
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
    fn check_recursion_in_dependent_exprs(
        &mut self,
        exprs: &[cst::Expr],
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let rcon_extension = vec![UnshiftedEntry(Entry::Top(None)); exprs.len()];

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &rcon_extension[..i]);
            self.check_recursion(expr, extended_rcon)?;
        }

        Ok(())
    }

    fn check_recursion_in_independent_exprs(
        &mut self,
        exprs: &[cst::Expr],
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for expr in exprs {
            self.check_recursion(expr.clone(), rcon)?;
        }
        Ok(())
    }
}

impl TypeChecker {
    fn is_strict_substruct(
        &mut self,
        expr: cst::Expr,
        possible_superstruct: Deb,
        rcon: RecursionCheckingContext,
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
    fn get_size_bound(
        &mut self,
        expr: cst::Expr,
        rcon: RecursionCheckingContext,
    ) -> Option<SizeBound> {
        match expr {
            cst::Expr::Ind(_)
            | cst::Expr::Vcon(_)
            | cst::Expr::Fun(_)
            | cst::Expr::App(_)
            | cst::Expr::For(_)
            | cst::Expr::Universe(_) => None,

            cst::Expr::Match(e) => self.get_size_bound_of_match(&e.hashee, rcon),

            cst::Expr::Deb(e) => self.get_size_bound_of_deb(&e.hashee, rcon),
        }
    }

    fn get_size_bound_of_match(
        &mut self,
        expr: &cst::Match,
        rcon: RecursionCheckingContext,
    ) -> Option<SizeBound> {
        if expr.cases.is_empty() {
            return Some(SizeBound::CaselessMatch);
        }

        let matchee_bound = self.get_size_bound(expr.matchee.clone(), rcon);

        let mut lowest_common_bound =
            self.get_size_bound_of_match_case(&expr.cases[0], matchee_bound, rcon)?;

        for case in &expr.cases[1..] {
            let case_bound =
                self.get_size_bound_of_match_case(case, matchee_bound.clone(), rcon)?;

            lowest_common_bound = get_min_size_bound(case_bound, lowest_common_bound, rcon)?;
        }

        Some(lowest_common_bound)
    }

    fn get_size_bound_of_match_case(
        &mut self,
        expr: &cst::MatchCase,
        matchee_bound: Option<SizeBound>,
        rcon: RecursionCheckingContext,
    ) -> Option<SizeBound> {
        let extension = get_rcon_extension_for_match_case_params(matchee_bound, expr.arity.value);
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.get_size_bound(expr.return_val.clone(), extended_rcon)
    }

    fn get_size_bound_of_deb(
        &mut self,
        expr: &cst::NumberLiteral,
        rcon: RecursionCheckingContext,
    ) -> Option<SizeBound> {
        let expr_deb = Deb(expr.value);
        let entry = rcon.get(expr_deb)?;
        match entry {
            Entry::Substruct(SizeBound::CaselessMatch, _) => Some(SizeBound::CaselessMatch),

            Entry::Substruct(SizeBound::Deb(_), _) | Entry::Top(_) => {
                Some(SizeBound::Deb(expr_deb))
            }
        }
    }
}

fn get_rcon_extension_for_match_case_params(
    matchee_bound: Option<SizeBound>,
    case_arity: usize,
) -> Vec<UnshiftedEntry<'static>> {
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

fn get_min_size_bound(
    a: SizeBound,
    b: SizeBound,
    rcon: RecursionCheckingContext,
) -> Option<SizeBound> {
    match (a, b) {
        (SizeBound::CaselessMatch, b) => Some(b),

        (a, SizeBound::CaselessMatch) => Some(a),

        (SizeBound::Deb(a), SizeBound::Deb(b)) => get_min_size_bound_of_debs(a, b, rcon),
    }
}

fn get_min_size_bound_of_debs(a: Deb, b: Deb, rcon: RecursionCheckingContext) -> Option<SizeBound> {
    get_smallest_superstruct_of_a_that_is_also_a_superstruct_of_b(a, b, rcon)
        .or_else(|| get_smallest_superstruct_of_a_that_is_also_a_superstruct_of_b(b, a, rcon))
}

fn get_smallest_superstruct_of_a_that_is_also_a_superstruct_of_b(
    a: Deb,
    b: Deb,
    rcon: RecursionCheckingContext,
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
fn is_deb_substruct(
    deb: Deb,
    possible_superstruct: Deb,
    rcon: RecursionCheckingContext,
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

impl RecursionCheckingContext<'_> {
    fn get_call_requirement(&self, deb: Deb) -> Option<CallRequirement> {
        let entry = self.get(deb)?;
        match entry {
            Entry::Top(Some(fun)) => match fun.decreasing_index {
                cst::NumberOrNonrecKw::Number(decreasing_index_literal) => {
                    let decreasing_index = decreasing_index_literal.value;
                    Some(CallRequirement::Recursive(RecursiveCallRequirement {
                        arg_index: decreasing_index,
                        strict_superstruct: Deb(deb.0 + fun.param_types.len() - decreasing_index),
                        definition_src: fun,
                    }))
                }

                cst::NumberOrNonrecKw::NonrecKw(_) => Some(CallRequirement::AccessForbidden(fun)),
            },

            Entry::Top(None) | Entry::Substruct(_, _) => None,
        }
    }

    fn get(&self, deb: Deb) -> Option<Entry> {
        let unshifted = self.get_unshifted(deb)?;
        Some(unshifted.0.upshift(deb.0 + 1))
    }

    fn get_unshifted(&self, deb: Deb) -> Option<UnshiftedEntry> {
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

impl Entry<'_> {
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
