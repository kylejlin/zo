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
            Entry::FunWithValidDecreasingIndex(fun) => match fun.decreasing_index {
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

            Entry::Irrelevant | Entry::RelevantDecreasing { .. } => None,
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

    /// If `deb` is a substruct `possible_superstruct`,
    /// this function returns `Some(strictness)`.
    /// Otherwise, it returns `None`.
    fn is_substruct(&self, deb: Deb, possible_superstruct: Deb) -> Option<Strict> {
        if deb == possible_superstruct {
            return Some(Strict(false));
        }

        let Some(entry) = self.get(deb) else {
            return None;
        };

        match entry {
            Entry::RelevantDecreasing {
                lineage: Lineage::CaselessMatch,
                ..
            } => Some(Strict(true)),

            Entry::RelevantDecreasing {
                lineage: Lineage::Unattached,
                ..
            } => None,

            Entry::RelevantDecreasing {
                lineage: Lineage::SubstructOf(direct_superstruct, strict),
                ..
            } => self
                .is_substruct(direct_superstruct, possible_superstruct)
                .map(|direct_strict| direct_strict | strict),

            Entry::Irrelevant | Entry::FunWithValidDecreasingIndex(_) => None,
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
    RelevantDecreasing { is_param: bool, lineage: Lineage },
    FunWithValidDecreasingIndex(&'a cst::Fun),
}

impl Entry<'_> {
    fn upshift(self, upshift_amount: usize) -> Self {
        match self {
            Entry::Irrelevant | Entry::FunWithValidDecreasingIndex(_) => self,

            Entry::RelevantDecreasing { is_param, lineage } => Entry::RelevantDecreasing {
                is_param,
                lineage: lineage.upshift(upshift_amount),
            },
        }
    }
}

#[derive(Clone)]
pub enum Lineage {
    Unattached,
    SubstructOf(Deb, Strict),
    CaselessMatch,
}

impl Lineage {
    fn upshift(self, upshift_amount: usize) -> Self {
        match self {
            Lineage::Unattached | Lineage::CaselessMatch => self,

            Lineage::SubstructOf(superstruct, strict) => {
                Lineage::SubstructOf(Deb(superstruct.0 + upshift_amount), strict)
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

        let matchee_lineage = self.get_lineage(match_.matchee.clone(), rcon);
        self.check_recursion_in_match_cases(&match_.cases, matchee_lineage, rcon)?;

        Ok(())
    }

    fn check_recursion_in_match_cases(
        &mut self,
        cases: &[cst::MatchCase],
        matchee_lineage: Lineage,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for case in cases {
            self.check_recursion_in_match_case(case, matchee_lineage.clone(), rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_match_case(
        &mut self,
        case: &cst::MatchCase,
        matchee_lineage: Lineage,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let extension = get_rcon_extension_for_match_case_params(matchee_lineage, case.arity.value);
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
        self.assert_decreasing_index_is_valid(fun)?;
        let fun_entry = UnshiftedEntry(Entry::FunWithValidDecreasingIndex(fun));
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
        app_arg_status.unwrap_or_else(|| match &fun.decreasing_index {
            cst::NumberOrNonrecKw::Number(decreasing_index_literal) => (0..fun.param_types.len())
                .map(|param_index| {
                    if param_index == decreasing_index_literal.value {
                        UnshiftedEntry(Entry::RelevantDecreasing {
                            is_param: true,
                            lineage: Lineage::Unattached,
                        })
                    } else {
                        UnshiftedEntry::irrelevant()
                    }
                })
                .collect(),

            cst::NumberOrNonrecKw::NonrecKw(_) => {
                vec![UnshiftedEntry::irrelevant(); fun.param_types.len()]
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
                            let lineage = self.get_lineage(arg.clone(), rcon);
                            match lineage {
                                Lineage::CaselessMatch => {
                                    UnshiftedEntry(Entry::RelevantDecreasing {
                                        is_param: true,
                                        lineage: Lineage::CaselessMatch,
                                    })
                                }

                                Lineage::Unattached => UnshiftedEntry::irrelevant(),

                                Lineage::SubstructOf(superstruct, strict) => {
                                    UnshiftedEntry(Entry::RelevantDecreasing {
                                        is_param: true,
                                        lineage: Lineage::SubstructOf(
                                            Deb(superstruct.0 + arg_index),
                                            strict,
                                        ),
                                    })
                                }
                            }
                        })
                        .collect(),

                    cst::NumberOrNonrecKw::Number(decreasing_index_literal) => app
                        .args
                        .iter()
                        .enumerate()
                        .map(|(arg_index, arg)| {
                            if arg_index == decreasing_index_literal.value {
                                let lineage =
                                    self.get_lineage(arg.clone(), rcon).upshift(arg_index);
                                UnshiftedEntry(Entry::RelevantDecreasing {
                                    is_param: true,
                                    lineage,
                                })
                            } else {
                                UnshiftedEntry::irrelevant()
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
        let lineage = self.get_lineage(expr, rcon);
        match lineage {
            Lineage::Unattached => false,

            Lineage::CaselessMatch => true,

            Lineage::SubstructOf(direct_superstruct, strict) => {
                if direct_superstruct == possible_superstruct {
                    strict.0
                } else if let Some(direct_strict) =
                    rcon.is_substruct(direct_superstruct, possible_superstruct)
                {
                    strict.0 || direct_strict.0
                } else {
                    false
                }
            }
        }
    }
}

impl TypeChecker {
    fn get_lineage(&mut self, expr: cst::Expr, rcon: RecursionCheckingContext) -> Lineage {
        match expr {
            cst::Expr::Ind(_)
            | cst::Expr::Vcon(_)
            | cst::Expr::Fun(_)
            | cst::Expr::App(_)
            | cst::Expr::For(_)
            | cst::Expr::Universe(_) => Lineage::Unattached,

            cst::Expr::Match(e) => self.get_lineage_of_match(&e.hashee, rcon),

            cst::Expr::Deb(e) => self.get_lineage_of_deb(&e.hashee, rcon),
        }
    }

    fn get_lineage_of_match(
        &mut self,
        expr: &cst::Match,
        rcon: RecursionCheckingContext,
    ) -> Lineage {
        if expr.cases.is_empty() {
            return Lineage::CaselessMatch;
        }

        let matchee_lineage = self.get_lineage(expr.matchee.clone(), rcon);

        let mut lowest_common =
            self.get_lineage_of_match_case(&expr.cases[0], matchee_lineage.clone(), rcon);

        for case in &expr.cases[1..] {
            let case_superstruct =
                self.get_lineage_of_match_case(case, matchee_lineage.clone(), rcon);

            lowest_common = get_common_lineage(lowest_common, case_superstruct, rcon);
        }

        lowest_common
    }

    fn get_lineage_of_match_case(
        &mut self,
        expr: &cst::MatchCase,
        matchee_lineage: Lineage,
        rcon: RecursionCheckingContext,
    ) -> Lineage {
        let extension = get_rcon_extension_for_match_case_params(matchee_lineage, expr.arity.value);
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.get_lineage(expr.return_val.clone(), extended_rcon)
    }

    fn get_lineage_of_deb(
        &mut self,
        expr: &cst::NumberLiteral,
        rcon: RecursionCheckingContext,
    ) -> Lineage {
        let expr_deb = Deb(expr.value);
        let Some(entry) = rcon.get(expr_deb) else {
            return Lineage::Unattached;
        };
        match entry {
            Entry::RelevantDecreasing {
                is_param: false,
                lineage,
            } => lineage,

            Entry::RelevantDecreasing {
                is_param: true,
                lineage: Lineage::CaselessMatch,
            } => Lineage::CaselessMatch,

            Entry::RelevantDecreasing {
                is_param: true,
                lineage: Lineage::SubstructOf(_, _) | Lineage::Unattached,
            } => Lineage::SubstructOf(expr_deb, Strict(false)),

            Entry::Irrelevant | Entry::FunWithValidDecreasingIndex(_) => Lineage::Unattached,
        }
    }
}

fn get_rcon_extension_for_match_case_params(
    matchee_lineage: Lineage,
    case_arity: usize,
) -> Vec<UnshiftedEntry<'static>> {
    match matchee_lineage {
        Lineage::Unattached => vec![UnshiftedEntry::irrelevant(); case_arity],

        Lineage::CaselessMatch => vec![
            UnshiftedEntry(Entry::RelevantDecreasing {
                is_param: false,
                lineage: Lineage::CaselessMatch,
            });
            case_arity
        ],

        Lineage::SubstructOf(matchee_superstruct, _) => (0..case_arity)
            .map(|case_param_index| {
                UnshiftedEntry(Entry::RelevantDecreasing {
                    is_param: false,
                    lineage: Lineage::SubstructOf(
                        Deb(matchee_superstruct.0 + case_param_index),
                        Strict(true),
                    ),
                })
            })
            .collect(),
    }
}

fn get_common_lineage(a: Lineage, b: Lineage, rcon: RecursionCheckingContext) -> Lineage {
    // TODO: This is wrong.
    // if let Some(a_strict_b) = rcon.is_substruct(a.0, b.0) {
    //     return Some((b.0, b.1 & (a.1 | a_strict_b)));
    // }

    // if let Some(b_strict_a) = rcon.is_substruct(b.0, a.0) {
    //     return Some((a.0, a.1 & (b.1 | b_strict_a)));
    // }

    // None

    let (a, b) = match (a, b) {
        (Lineage::Unattached, _) | (_, Lineage::Unattached) => return Lineage::Unattached,

        (Lineage::CaselessMatch, b) => return b,
        (a, Lineage::CaselessMatch) => return a,

        (
            Lineage::SubstructOf(a_superstruct, a_strict),
            Lineage::SubstructOf(b_superstruct, b_strict),
        ) => ((a_superstruct, a_strict), (b_superstruct, b_strict)),
    };

    todo!()
}
