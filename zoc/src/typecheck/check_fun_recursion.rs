use super::*;

use std::ops::{BitAnd, BitOr};

#[derive(Clone, Copy)]
pub enum RecursionCheckingContext<'a> {
    Base(&'a [UnshiftedEntry<'a>]),
    Snoc(&'a RecursionCheckingContext<'a>, &'a [UnshiftedEntry<'a>]),
}

impl RecursionCheckingContext<'static> {
    pub fn empty() -> Self {
        RecursionCheckingContext::Base(&[])
    }
}

impl RecursionCheckingContext<'_> {
    fn get_call_requirement(&self, deb: Deb) -> Option<CallRequirement> {
        let entry = self.get(deb)?;
        match entry {
            Entry::RecursiveFun {
                valid_decreasing_arg_index: decreasing_arg_index,
                definition_src,
            } => Some(CallRequirement::Recursive(RecursiveCallRequirement {
                arg_index: decreasing_arg_index,
                strict_superstruct: Deb(
                    deb.0 + definition_src.param_types.len() - decreasing_arg_index
                ),
                definition_src,
            })),

            Entry::NonrecursiveFun { definition_src } => {
                Some(CallRequirement::AccessForbidden(definition_src))
            }

            Entry::Irrelevant
            | Entry::DecreasingParam { .. }
            | Entry::DecreasingParamStrictSubstruct { .. } => None,
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

    /// If `deb` is a descendant of `possible_ancestor`,
    /// this function returns `Some(strictness)`.
    /// Otherwise, it returns `None`.
    fn is_descendant(&self, deb: Deb, possible_ancestor: Deb) -> Option<Strict> {
        if deb == possible_ancestor {
            return Some(Strict(false));
        }

        let Some(entry) = self.get(deb) else {
            return None;
        };

        match entry {
            Entry::DecreasingParam { parent: None } => None,

            Entry::DecreasingParam {
                parent: Some((parent, strict)),
            } => self
                .is_descendant(parent, possible_ancestor)
                .map(|parent_strict| parent_strict | strict),

            Entry::DecreasingParamStrictSubstruct { parent_param } => self
                .is_descendant(parent_param, possible_ancestor)
                .map(|_| Strict(true)),

            Entry::Irrelevant | Entry::RecursiveFun { .. } | Entry::NonrecursiveFun { .. } => None,
        }
    }
}

#[derive(Clone)]
pub struct UnshiftedEntry<'a>(pub Entry<'a>);

impl UnshiftedEntry<'static> {
    fn irrelevant() -> Self {
        Self(Entry::Irrelevant)
    }
}

#[derive(Clone)]
pub enum Entry<'a> {
    Irrelevant,
    RecursiveFun {
        valid_decreasing_arg_index: usize,
        definition_src: &'a cst::Fun,
    },
    NonrecursiveFun {
        definition_src: &'a cst::Fun,
    },
    DecreasingParam {
        parent: Option<(Deb, Strict)>,
    },
    DecreasingParamStrictSubstruct {
        parent_param: Deb,
    },
}

impl Entry<'_> {
    fn upshift(self, upshift_amount: usize) -> Self {
        match self {
            Entry::Irrelevant
            | Entry::RecursiveFun {
                valid_decreasing_arg_index: _,
                definition_src: _,
            }
            | Entry::NonrecursiveFun { definition_src: _ } => self,

            Entry::DecreasingParam { parent } => Entry::DecreasingParam {
                parent: parent
                    .map(|(parent_param, strict)| (Deb(parent_param.0 + upshift_amount), strict)),
            },
            Entry::DecreasingParamStrictSubstruct { parent_param } => {
                Entry::DecreasingParamStrictSubstruct {
                    parent_param: Deb(parent_param.0 + upshift_amount),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Strict(pub bool);

impl BitOr for Strict {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Strict(self.0 | rhs.0)
    }
}

impl BitAnd for Strict {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Strict(self.0 & rhs.0)
    }
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

        let singleton = vec![UnshiftedEntry::irrelevant()];
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

        let extension = vec![UnshiftedEntry::irrelevant(); def.param_types.len()];
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

        let param_deb = self
            .get_lowest_superstruct_param(match_.matchee.clone(), rcon)
            .map(|(param_deb, _)| param_deb);
        self.check_recursion_in_match_cases(&match_.cases, param_deb, rcon)?;

        Ok(())
    }

    fn check_recursion_in_match_cases(
        &mut self,
        cases: &[cst::MatchCase],
        param_deb: Option<Deb>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for case in cases {
            self.check_recursion_in_match_case(case, param_deb, rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_match_case(
        &mut self,
        case: &cst::MatchCase,
        param_deb: Option<Deb>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let extension = get_rcon_extension_of_irrelevant_entries_or_strict_substruct_entries(
            param_deb,
            case.arity.value,
        );
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
        let extension = vec![UnshiftedEntry::irrelevant(); fun.param_types.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(fun.return_type.clone(), extended_rcon)?;
        Ok(())
    }

    fn get_fun_rcon_extension<'a>(
        &mut self,
        fun: &'a cst::Fun,
        app_arg_status: Option<Vec<UnshiftedEntry<'a>>>,
    ) -> Result<Vec<UnshiftedEntry<'a>>, TypeError> {
        let fun_entry = self.get_fun_entry_and_assert_decreasing_index_is_valid(fun)?;
        let param_entries = self.get_fun_param_entries(fun, app_arg_status);

        let mut out = param_entries;
        out.push(fun_entry);
        Ok(out)
    }

    fn get_fun_entry_and_assert_decreasing_index_is_valid<'a>(
        &mut self,
        fun: &'a cst::Fun,
    ) -> Result<UnshiftedEntry<'a>, TypeError> {
        match &fun.decreasing_index {
            cst::NumberOrNonrecKw::Number(decreasing_index_literal) => {
                let decreasing_arg_index = decreasing_index_literal.value;
                if decreasing_arg_index >= fun.param_types.len() {
                    return Err(TypeError::DecreasingArgIndexTooBig { fun: fun.clone() });
                }

                Ok(UnshiftedEntry(Entry::RecursiveFun {
                    valid_decreasing_arg_index: decreasing_arg_index,
                    definition_src: fun,
                }))
            }

            cst::NumberOrNonrecKw::NonrecKw(_) => Ok(UnshiftedEntry(Entry::NonrecursiveFun {
                definition_src: fun,
            })),
        }
    }

    fn get_fun_param_entries<'a>(
        &mut self,
        fun: &'a cst::Fun,
        app_arg_status: Option<Vec<UnshiftedEntry<'a>>>,
    ) -> Vec<UnshiftedEntry<'a>> {
        app_arg_status.unwrap_or_else(|| {
            match &fun.decreasing_index {
                cst::NumberOrNonrecKw::Number(decreasing_index_literal) => {
                    (0..fun.param_types.len())
                        .map(|param_index| {
                            if param_index == decreasing_index_literal.value {
                                UnshiftedEntry(Entry::DecreasingParam { parent: None })
                            } else {
                                UnshiftedEntry::irrelevant()
                            }
                        })
                        .collect()
                }

                cst::NumberOrNonrecKw::NonrecKw(_) => {
                    // If the function is non-recursive, then all params are vacuously decreasing.
                    vec![
                        UnshiftedEntry(Entry::DecreasingParam { parent: None });
                        fun.param_types.len()
                    ]
                }
            }
        })
    }

    fn check_recursion_in_app(
        &mut self,
        app: &cst::App,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let skip_callee_check = match &app.callee {
            cst::Expr::Deb(callee) => {
                let callee_deb = Deb(callee.hashee.value);
                if let Some(requirement) = rcon.get_call_requirement(callee_deb) {
                    match requirement {
                        CallRequirement::Recursive(requirement) => self
                            .assert_arg_satisfies_recursive_call_requirement(
                                app,
                                requirement,
                                rcon,
                            )?,

                        CallRequirement::AccessForbidden(definition_src) => {
                            return Err(
                                TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                                    deb: callee.hashee.clone(),
                                    definition_src: definition_src.clone(),
                                },
                            )
                        }
                    }

                    true
                } else {
                    false
                }
            }

            cst::Expr::Fun(callee) => {
                let arg_status: Vec<UnshiftedEntry> = match &callee.hashee.decreasing_index {
                    cst::NumberOrNonrecKw::NonrecKw(_) => app
                        .args
                        .iter()
                        .enumerate()
                        .map(|(arg_index, arg)| {
                            // If the function is non-recursive,
                            // then all params are vacuously decreasing
                            // (i.e., they are decreasing in all zero recursive calls).
                            if let Some((param_deb, strict)) =
                                self.get_lowest_superstruct_param(arg.clone(), rcon)
                            {
                                UnshiftedEntry(Entry::DecreasingParam {
                                    parent: Some((Deb(param_deb.0 + arg_index), strict)),
                                })
                            } else {
                                // TODO: Consider if Entry::DecreasingParam.parent should be non-`Option`al.
                                // In this case, `DecreasingParam { parent: None }` becomes `Irrelevant`.
                                UnshiftedEntry(Entry::DecreasingParam { parent: None })
                            }
                        })
                        .collect(),

                    cst::NumberOrNonrecKw::Number(decreasing_index_literal) => app
                        .args
                        .iter()
                        .enumerate()
                        .map(|(arg_index, arg)| {
                            if arg_index == decreasing_index_literal.value {
                                let parent = self
                                    .get_lowest_superstruct_param(arg.clone(), rcon)
                                    .map(|(param_deb, strict)| {
                                        (Deb(param_deb.0 + arg_index), strict)
                                    });

                                UnshiftedEntry(Entry::DecreasingParam { parent })
                            } else {
                                UnshiftedEntry(Entry::Irrelevant)
                            }
                        })
                        .collect(),
                };

                self.check_recursion_in_fun(&callee.hashee, Some(arg_status), rcon)?;

                true
            }

            _ => false,
        };

        if !skip_callee_check {
            self.check_recursion(app.callee.clone(), rcon)?;
        }

        self.check_recursion_in_independent_exprs(&app.args, rcon)?;

        Ok(())
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

        let extension = vec![UnshiftedEntry::irrelevant(); for_.param_types.len()];
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
        let rcon_extension = vec![UnshiftedEntry::irrelevant(); exprs.len()];

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
        let Some((lowest_superstruct, strict)) = self.get_lowest_superstruct_param(expr, rcon) else {
            return false;
        };

        if lowest_superstruct == possible_superstruct {
            return strict.0;
        }

        if let Some(descendant_strict) =
            rcon.is_descendant(lowest_superstruct, possible_superstruct)
        {
            return strict.0 || descendant_strict.0;
        }

        false
    }
}

impl TypeChecker {
    /// If `expr` is a strict substruct of some param at deb `d`,
    /// then `Some((d, Strict(true)))` is returned.
    /// If `expr` is a nonstrict substruct of some param at deb `d`,
    /// then `Some((d, Strict(false)))` is returned.
    /// If there are multiple possible values for `d`,
    /// we choose return the lowest in the param tree.
    /// Otherwise, `None` is returned.
    fn get_lowest_superstruct_param(
        &mut self,
        expr: cst::Expr,
        rcon: RecursionCheckingContext,
    ) -> Option<(Deb, Strict)> {
        match expr {
            cst::Expr::Ind(_)
            | cst::Expr::Vcon(_)
            | cst::Expr::Fun(_)
            | cst::Expr::App(_)
            | cst::Expr::For(_)
            | cst::Expr::Universe(_) => None,

            cst::Expr::Match(e) => self.get_lowest_superstruct_param_of_match(&e.hashee, rcon),

            cst::Expr::Deb(e) => self.get_lowest_superstruct_param_of_deb(&e.hashee, rcon),
        }
    }

    fn get_lowest_superstruct_param_of_match(
        &mut self,
        expr: &cst::Match,
        rcon: RecursionCheckingContext,
    ) -> Option<(Deb, Strict)> {
        if expr.cases.is_empty() {
            // TODO: This is blatantly WRONG.
            // A zero-cased match vacuously is a strict substruct
            // of _any_ param.
            // We will need to redesign the whole system to handle this.
            return None;
        }

        let matchee_superstruct = self.get_lowest_superstruct_param(expr.matchee.clone(), rcon);

        let mut lowest_common = self.get_lowest_superstruct_param_of_match_case(
            &expr.cases[0],
            matchee_superstruct,
            rcon,
        )?;

        for case in &expr.cases[1..] {
            let case_superstruct =
                self.get_lowest_superstruct_param_of_match_case(case, matchee_superstruct, rcon)?;

            lowest_common =
                get_lowest_common_ancestor_param(lowest_common, case_superstruct, rcon)?;
        }

        Some(lowest_common)
    }

    fn get_lowest_superstruct_param_of_match_case(
        &mut self,
        expr: &cst::MatchCase,
        matchee_superstruct: Option<(Deb, Strict)>,
        rcon: RecursionCheckingContext,
    ) -> Option<(Deb, Strict)> {
        todo!()
    }

    fn get_lowest_superstruct_param_of_deb(
        &mut self,
        expr: &cst::NumberLiteral,
        rcon: RecursionCheckingContext,
    ) -> Option<(Deb, Strict)> {
        let expr_deb = Deb(expr.value);
        let entry = rcon.get(expr_deb)?;
        match entry {
            Entry::DecreasingParamStrictSubstruct { parent_param } => {
                Some((parent_param, Strict(true)))
            }

            Entry::DecreasingParam { .. } => Some((expr_deb, Strict(false))),

            Entry::Irrelevant | Entry::RecursiveFun { .. } | Entry::NonrecursiveFun { .. } => None,
        }
    }
}

fn get_rcon_extension_of_irrelevant_entries_or_strict_substruct_entries(
    deb: Option<Deb>,
    len: usize,
) -> Vec<UnshiftedEntry<'static>> {
    let Some(deb) = deb else {
        return vec![UnshiftedEntry::irrelevant(); len];
    };

    (0..len)
        .map(|i| {
            UnshiftedEntry(Entry::DecreasingParamStrictSubstruct {
                parent_param: Deb(deb.0 + i),
            })
        })
        .collect()
}

fn get_lowest_common_ancestor_param(
    a: (Deb, Strict),
    b: (Deb, Strict),
    rcon: RecursionCheckingContext,
) -> Option<(Deb, Strict)> {
    if let Some(a_strict_b) = rcon.is_descendant(a.0, b.0) {
        return Some((b.0, b.1 & (a.1 | a_strict_b)));
    }

    if let Some(b_strict_a) = rcon.is_descendant(b.0, a.0) {
        return Some((a.0, a.1 & (b.1 | b_strict_a)));
    }

    None
}
