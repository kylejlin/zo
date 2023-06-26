use crate::{
    ast::*,
    eval::{Evaluator, NormalForm, Normalized},
    replace_debs::*,
};

use std::{ops::BitOrAssign, rc::Rc};

type RcHashed<T> = Rc<SemanticHashed<T>>;

mod concrete_substitution;
use concrete_substitution::*;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: RcHashed<DebNode>,
        tcon_len: usize,
    },
    InvalidVconIndex(RcHashed<Vcon>),
    UnexpectedNonTypeExpression {
        expr: Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        expr: Expr,
        level: UniverseLevel,
        max_permitted_level: UniverseLevel,
    },
    WrongNumberOfIndexArguments {
        def: VconDef,
        expected: usize,
        actual: usize,
    },
    NonInductiveMatcheeType {
        expr: Expr,
        type_: NormalForm,
    },
    WrongNumberOfMatchCases {
        match_: RcHashed<Match>,
        matchee_type_ind: Normalized<RcHashed<Ind>>,
    },
    TypeMismatch {
        expr: Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
        subbed_expected: NormalForm,
        subbed_actual: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: RcHashed<App>,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: RcHashed<App>,
        callee_type: Normalized<RcHashed<For>>,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum LazyTypeContext<'a> {
    Base(Normalized<&'a [Expr]>),
    Snoc(&'a LazyTypeContext<'a>, Normalized<&'a [Expr]>),
}

impl LazyTypeContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazyTypeContext::Base(types) => types.raw().len(),
            LazyTypeContext::Snoc(subcontext, types) => subcontext.len() + types.raw().len(),
        }
    }

    pub fn get(&self, deb: Deb) -> Option<NormalForm> {
        match self {
            LazyTypeContext::Base(types) => {
                let index = (types.raw().len() - 1).checked_sub(deb.0)?;
                types.get(index).map(Normalized::cloned)
            }
            LazyTypeContext::Snoc(subcontext, types) => {
                if let Some(index) = (types.raw().len() - 1).checked_sub(deb.0) {
                    types.get(index).map(Normalized::cloned)
                } else {
                    subcontext.get(Deb(deb.0 - types.raw().len()))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LazySubstitutionContext<'a> {
    Base(&'a [LazySubstitution]),
    Snoc(&'a LazySubstitutionContext<'a>, &'a [LazySubstitution]),
}

#[derive(Debug, Clone)]
pub struct LazySubstitution {
    pub tcon_len: usize,
    pub from: NormalForm,
    pub to: NormalForm,
}

impl LazySubstitutionContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazySubstitutionContext::Base(subs) => subs.len(),
            LazySubstitutionContext::Snoc(first, subs) => first.len() + subs.len(),
        }
    }

    pub fn into_concrete_noncompounded_substitutions(
        self,
        current_tcon_len: usize,
    ) -> Vec<ConcreteSubstitution> {
        match self {
            LazySubstitutionContext::Base(subs) => {
                lazy_substitution_slice_to_concrete_noncompounded_substitutions(
                    subs,
                    current_tcon_len,
                )
                .collect()
            }

            LazySubstitutionContext::Snoc(first, subs) => {
                let mut first = first.into_concrete_noncompounded_substitutions(current_tcon_len);
                let rest = lazy_substitution_slice_to_concrete_noncompounded_substitutions(
                    subs,
                    current_tcon_len,
                );
                first.extend(rest);
                first
            }
        }
    }
}

fn lazy_substitution_slice_to_concrete_noncompounded_substitutions(
    subs: &[LazySubstitution],
    current_tcon_len: usize,
) -> impl Iterator<Item = ConcreteSubstitution> + '_ {
    subs.iter().map(move |sub| {
        let upshift_amount = current_tcon_len - sub.tcon_len;
        let from = sub.from.clone().upshift(upshift_amount);
        let to = sub.to.clone().upshift(upshift_amount);
        ConcreteSubstitution { from, to }
    })
}

#[derive(Clone, Debug, Default)]
pub struct TypeChecker {
    pub evaluator: Evaluator,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TypeChecker {
    pub fn get_type(
        &mut self,
        expr: Expr,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        match expr {
            Expr::Ind(e) => self.get_type_of_ind(e, tcon, scon),
            Expr::Vcon(e) => self.get_type_of_vcon(e, tcon, scon),
            Expr::Match(e) => self.get_type_of_match(e, tcon, scon),
            Expr::Fun(e) => self.get_type_of_fun(e, tcon, scon),
            Expr::App(e) => self.get_type_of_app(e, tcon, scon),
            Expr::For(e) => self.get_type_of_for(e, tcon, scon),
            Expr::Deb(e) => self.get_type_of_deb(e, tcon),
            Expr::Universe(e) => self.get_type_of_universe(e),
        }
    }

    fn get_type_of_ind(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_ind_precheck(ind.clone(), tcon, scon)?;
        Ok(self.get_ind_type_assuming_ind_is_well_typed(ind))
    }

    fn perform_ind_precheck(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let index_type_types =
            self.get_types_of_dependent_expressions(ind.value.index_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(&index_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: ind.value.index_types.value[offender_index].clone(),
                type_: index_type_types.index(offender_index).cloned(),
            }
        })?;

        // Once we verify that the index types are all well-typed,
        // it is safe to construct a predicted type for the ind type.
        let predicted_ind_type = self.get_ind_type_assuming_ind_is_well_typed(ind.clone());

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &index_type_types.raw(),
            ind.value.universe_level,
        )
        .map_err(|(offender_index, offender_level)| {
            TypeError::UniverseInconsistencyInIndDef {
                expr: ind.value.index_types.value[offender_index].clone(),
                level: offender_level,
                max_permitted_level: ind.value.universe_level,
            }
        })?;

        self.assert_ind_vcon_defs_are_well_typed(ind, predicted_ind_type, tcon, scon)?;

        Ok(())
    }

    fn assert_ind_vcon_defs_are_well_typed(
        &mut self,
        ind: RcHashed<Ind>,
        predicted_ind_type: NormalForm,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        for def in ind.value.vcon_defs.value.iter() {
            self.assert_ind_vcon_def_is_well_typed(
                ind.clone(),
                predicted_ind_type.clone(),
                def,
                tcon,
                scon,
            )?;
        }
        Ok(())
    }

    fn assert_ind_vcon_def_is_well_typed(
        &mut self,
        ind: RcHashed<Ind>,
        predicted_ind_type: NormalForm,
        def: &VconDef,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let recursive_ind_entry: Normalized<Vec<Expr>> =
            std::iter::once(predicted_ind_type).collect();
        let tcon_with_recursive_ind_entry =
            LazyTypeContext::Snoc(&tcon, recursive_ind_entry.to_derefed());
        let param_type_types = self.get_types_of_dependent_expressions(
            def.param_types.clone(),
            tcon_with_recursive_ind_entry,
            scon,
        )?;

        let tcon_with_params = LazyTypeContext::Snoc(
            &tcon_with_recursive_ind_entry,
            param_type_types.to_derefed(),
        );
        self.get_types_of_independent_expressions(def.index_args.clone(), tcon_with_params, scon)?;

        if ind.value.index_types.value.len() != def.index_args.value.len() {
            return Err(TypeError::WrongNumberOfIndexArguments {
                def: def.clone(),
                expected: ind.value.index_types.value.len(),
                actual: def.index_args.value.len(),
            });
        }

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &param_type_types.raw(),
            ind.value.universe_level,
        )
        .map_err(|(offender_index, offender_level)| {
            TypeError::UniverseInconsistencyInIndDef {
                expr: def.param_types.value[offender_index].clone(),
                level: offender_level,
                max_permitted_level: ind.value.universe_level,
            }
        })?;

        self.assert_vcon_def_is_strictly_positive(ind, def, tcon, scon)?;

        Ok(())
    }

    fn assert_vcon_def_is_strictly_positive(
        &mut self,
        _ind: RcHashed<Ind>,
        _def: &VconDef,
        _tcon: LazyTypeContext,
        _scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        // TODO: Actually check positivity.
        Ok(())
    }

    /// This function assumes that the index types are well-typed.
    /// If they are not, this will cause (probably undetectable) bugs.
    ///
    /// However, you may safely call this function even if the vcon defs
    /// are ill-typed.
    fn get_ind_type_assuming_ind_is_well_typed(&mut self, ind: RcHashed<Ind>) -> NormalForm {
        let normalized_index_types = self
            .evaluator
            .eval_expressions(ind.value.index_types.clone());
        let return_type = self.get_ind_return_type(ind);
        Normalized::for_(normalized_index_types, return_type).collapse_if_nullary()
    }

    fn get_ind_return_type(&mut self, ind: RcHashed<Ind>) -> NormalForm {
        Normalized::universe(UniverseNode {
            level: ind.value.universe_level,
        })
    }

    fn get_type_of_vcon(
        &mut self,
        vcon: RcHashed<Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_vcon_precheck(vcon.clone(), tcon, scon)?;

        let vcon_index = vcon.value.vcon_index;
        let defs: &[VconDef] = &vcon.value.ind.value.vcon_defs.value;
        let Some(def) = defs.get(vcon_index) else {
            return Err(TypeError::InvalidVconIndex(vcon));
        };
        self.get_type_of_trusted_vcon_def(def, vcon.value.ind.clone())
    }

    fn perform_vcon_precheck(
        &mut self,
        vcon: RcHashed<Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        self.get_type_of_ind(vcon.value.ind.clone(), tcon, scon)?;
        Ok(())
    }

    fn get_type_of_trusted_vcon_def(
        &mut self,
        def: &VconDef,
        ind: RcHashed<Ind>,
    ) -> Result<NormalForm, TypeError> {
        let normalized_param_types = self.evaluator.eval_expressions(def.param_types.clone());
        let normalized_ind = self.evaluator.eval_ind(ind.clone());
        let normalized_index_args = self.evaluator.eval_expressions(def.index_args.clone());
        let return_type = Normalized::app_with_ind_callee(normalized_ind, normalized_index_args)
            .collapse_if_nullary();
        Ok(Normalized::for_(normalized_param_types, return_type).collapse_if_nullary())
    }

    fn get_type_of_match(
        &mut self,
        match_: RcHashed<Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_match_precheck(match_.clone(), tcon, scon)?;

        let normalized_return_type = self.evaluator.eval(match_.value.return_type.clone());
        Ok(normalized_return_type)
    }

    fn perform_match_precheck(
        &mut self,
        match_: RcHashed<Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let matchee_type = self.get_type(match_.value.matchee.clone(), tcon, scon)?;
        let Some((well_typed_matchee_type_ind, well_typed_matchee_type_args)) = matchee_type.clone().ind_or_ind_app() else {
            return Err(TypeError::NonInductiveMatcheeType {
                expr: match_.value.matchee.clone(),
                type_: matchee_type,
            });
        };

        let return_type_type = self.get_type(match_.value.return_type.clone(), tcon, scon)?;
        if !return_type_type.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression {
                expr: match_.value.return_type.clone(),
                type_: return_type_type,
            });
        }

        let vcon_count = well_typed_matchee_type_ind
            .raw()
            .value
            .vcon_defs
            .value
            .len();
        let match_case_count = match_.value.cases.value.len();
        if vcon_count != match_case_count {
            return Err(TypeError::WrongNumberOfMatchCases {
                match_: match_.clone(),
                matchee_type_ind: well_typed_matchee_type_ind.clone(),
            });
        }

        let normalized_match_return_type = self.evaluator.eval(match_.value.return_type.clone());
        self.perform_match_cases_precheck(
            match_,
            normalized_match_return_type,
            well_typed_matchee_type_ind,
            well_typed_matchee_type_args,
            tcon,
            scon,
        )?;

        Ok(())
    }

    fn perform_match_cases_precheck(
        &mut self,
        match_: RcHashed<Match>,
        match_return_type: NormalForm,
        well_typed_matchee_type_ind: Normalized<RcHashed<Ind>>,
        well_typed_matchee_type_args: Normalized<RcHashed<Box<[Expr]>>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let vcon_defs = well_typed_matchee_type_ind.without_digest().vcon_defs();
        let vcon_defs = vcon_defs.without_digest();
        let vcon_defs = vcon_defs.derefed();

        for match_case_index in 0..match_.value.cases.value.len() {
            let well_typed_vcon_def = vcon_defs.index(match_case_index);
            let match_case = &match_.value.cases.value[match_case_index];
            self.perform_match_case_precheck(
                match_case,
                match_case_index,
                well_typed_vcon_def,
                match_.clone(),
                match_return_type.clone(),
                well_typed_matchee_type_ind.clone(),
                well_typed_matchee_type_args.clone(),
                tcon,
                scon,
            )?;
        }

        Ok(())
    }

    fn perform_match_case_precheck(
        &mut self,
        match_case: &MatchCase,
        match_case_index: usize,
        well_typed_vcon_def: Normalized<&VconDef>,
        match_: RcHashed<Match>,
        match_return_type: NormalForm,
        well_typed_matchee_type_ind: Normalized<RcHashed<Ind>>,
        well_typed_matchee_type_args: Normalized<RcHashed<Box<[Expr]>>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let ind_singleton: [Expr; 1] = [well_typed_matchee_type_ind.clone().into_raw().into()];
        let ind_singleton_deb_substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };

        let match_case_param_types = ind_singleton_deb_substituter
            .replace_debs_in_expressions_with_increasing_cutoff(
                well_typed_vcon_def.raw().param_types.clone(),
                0,
            );
        let match_case_param_types = self.evaluator.eval_expressions(match_case_param_types);
        let match_case_param_types = match_case_param_types.without_digest();
        let extended_tcon = LazyTypeContext::Snoc(&tcon, match_case_param_types.derefed());

        let match_case_param_count = match_case_param_types.raw().len();
        let substituted_vcon_index_args = well_typed_vcon_def
            .index_args()
            .replace_deb0_with_ind_with_increasing_cutoff(well_typed_matchee_type_ind.clone());
        let upshifted_matchee_type_args = well_typed_matchee_type_args
            .clone()
            .upshift_expressions_with_constant_cutoff(match_case_param_count);
        let extended_tcon_len = extended_tcon.len();
        let upshifted_matchee =
            DebUpshifter(match_case_param_count).replace_debs(match_.value.matchee.clone(), 0);
        let upshifted_normalized_matchee = self.evaluator.eval(upshifted_matchee);
        let parameterized_vcon_capp = Normalized::vcon_capp(
            well_typed_matchee_type_ind,
            match_case_index,
            match_case_param_count,
        );
        let new_substitutions: Vec<LazySubstitution> =
            (0..substituted_vcon_index_args.raw().value.len())
                .map(|i| {
                    let vcon_index_arg = substituted_vcon_index_args
                        .without_digest()
                        .derefed()
                        .index(i)
                        .cloned();
                    let matchee_index_arg = upshifted_matchee_type_args
                        .without_digest()
                        .derefed()
                        .index(i)
                        .cloned();
                    LazySubstitution {
                        tcon_len: extended_tcon_len,
                        from: vcon_index_arg,
                        to: matchee_index_arg,
                    }
                })
                .chain(std::iter::once(LazySubstitution {
                    tcon_len: extended_tcon_len,
                    from: upshifted_normalized_matchee,
                    to: parameterized_vcon_capp,
                }))
                .collect();
        let extended_scon = LazySubstitutionContext::Snoc(&scon, &new_substitutions);

        let match_case_return_type =
            self.get_type(match_case.return_val.clone(), extended_tcon, extended_scon)?;

        self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: match_case.return_val.clone(),
                expected_type: match_return_type,
                actual_type: match_case_return_type.clone(),
                tcon_len: extended_tcon_len,
            },
            extended_scon,
        )?;

        Ok(())
    }

    fn get_type_of_fun(
        &mut self,
        fun: RcHashed<Fun>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types =
            self.get_types_of_dependent_expressions(fun.value.param_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(param_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: fun.value.param_types.value[offender_index].clone(),
                type_: param_type_types.index(offender_index).cloned(),
            }
        })?;
        let normalized_param_types = self
            .evaluator
            .eval_expressions(fun.value.param_types.clone());

        let return_type_type = self.get_type(fun.value.return_type.clone(), tcon, scon)?;
        if !return_type_type.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression {
                expr: fun.value.return_type.clone(),
                type_: return_type_type,
            });
        }
        let normalized_return_type = self.evaluator.eval(fun.value.return_type.clone());

        let only_possible_fun_type: NormalForm = Normalized::for_(
            normalized_param_types.clone(),
            normalized_return_type.clone(),
        )
        .into();

        let param_types_and_recursive_fun_param_type: Normalized<Vec<Expr>> =
            normalized_param_types
                .without_digest()
                .derefed()
                .to_vec_normalized()
                .into_iter()
                .chain(std::iter::once(only_possible_fun_type.clone()))
                .collect();
        let tcon_extended_with_params_and_recursive_fun_param =
            LazyTypeContext::Snoc(&tcon, param_types_and_recursive_fun_param_type.to_derefed());

        let return_val_type = self.get_type(
            fun.value.return_val.clone(),
            tcon_extended_with_params_and_recursive_fun_param,
            scon,
        )?;

        self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: fun.value.return_val.clone(),
                expected_type: normalized_return_type.clone(),
                actual_type: return_val_type,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        Ok(only_possible_fun_type)
    }

    fn get_type_of_app(
        &mut self,
        app: RcHashed<App>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let callee_type = self
            .get_type(app.value.callee.clone(), tcon, scon)?
            .try_into_for()
            .map_err(|original| TypeError::CalleeTypeIsNotAForExpression {
                app: app.clone(),
                callee_type: original,
            })?;

        let arg_count = app.value.args.value.len();
        let param_count = callee_type.raw().value.param_types.value.len();
        if arg_count != param_count {
            return Err(TypeError::WrongNumberOfAppArguments {
                app,
                callee_type,
                expected: param_count,
                actual: arg_count,
            });
        }

        let arg_types =
            self.get_types_of_independent_expressions(app.value.args.clone(), tcon, scon)?;
        let normalized_args = self.evaluator.eval_expressions(app.value.args.clone());

        let substituted_param_types = self.substitute_param_types(
            callee_type.without_digest().param_types(),
            normalized_args.clone(),
        );
        self.assert_expected_type_equalities_holds_after_applying_scon(
            ExpectedTypeEqualities {
                exprs: app.value.args.clone(),
                expected_types: substituted_param_types,
                actual_types: arg_types,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        let arg_substituter = DebDownshiftSubstituter {
            new_exprs: &normalized_args.raw().value,
        };
        let unnormalized_substituted_return_type =
            arg_substituter.replace_debs(callee_type.raw().value.return_type.clone(), 0);
        let substituted_return_type = self.evaluator.eval(unnormalized_substituted_return_type);
        Ok(substituted_return_type)
    }

    fn substitute_param_types(
        &mut self,
        unsubstituted_param_types: Normalized<RcHashed<Box<[Expr]>>>,
        normalized_args: Normalized<RcHashed<Box<[Expr]>>>,
    ) -> Normalized<Vec<Expr>> {
        let len = normalized_args.raw().value.len();

        let out: Vec<NormalForm> = (0..len)
            .map(|param_index| {
                let unsubstituted_param_type = unsubstituted_param_types
                    .without_digest()
                    .derefed()
                    .index(param_index)
                    .cloned();
                let substituter = DebDownshiftSubstituter {
                    new_exprs: &normalized_args.raw().value[0..param_index],
                };
                let substituted = substituter.replace_debs(unsubstituted_param_type.into_raw(), 0);
                self.evaluator.eval(substituted)
            })
            .collect();

        Normalized::from_vec_normalized(out)
    }

    fn get_type_of_for(
        &mut self,
        for_: RcHashed<For>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types =
            self.get_types_of_dependent_expressions(for_.value.param_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(param_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: for_.value.param_types.value[offender_index].clone(),
                type_: param_type_types.index(offender_index).cloned(),
            }
        })?;

        let tcon_extended_with_params = LazyTypeContext::Snoc(&tcon, param_type_types.to_derefed());
        let return_type_type = self.get_type(
            for_.value.return_type.clone(),
            tcon_extended_with_params,
            scon,
        )?;
        let return_type_type_universe_level = match return_type_type.raw() {
            Expr::Universe(universe_node) => universe_node.value.level,

            _ => {
                return Err(TypeError::UnexpectedNonTypeExpression {
                    expr: for_.value.return_type.clone(),
                    type_: return_type_type,
                })
            }
        };

        let max_level = return_type_type_universe_level
            .max_or_self(get_max_universe_level(param_type_types.raw()));
        Ok(Normalized::universe(UniverseNode { level: max_level }))
    }

    fn get_type_of_deb(
        &mut self,
        deb: RcHashed<DebNode>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        if let Some(expr) = tcon.get(deb.value.deb) {
            return Ok(expr);
        }

        return Err(TypeError::InvalidDeb {
            deb,
            tcon_len: tcon.len(),
        });
    }

    fn get_type_of_universe(
        &mut self,
        universe: RcHashed<UniverseNode>,
    ) -> Result<NormalForm, TypeError> {
        return Ok(self
            .evaluator
            .eval(Expr::Universe(Rc::new(SemanticHashed::new(UniverseNode {
                level: UniverseLevel(universe.value.level.0 + 1),
            })))));
    }

    fn get_types_of_dependent_expressions(
        &mut self,
        exprs: RcHashed<Box<[Expr]>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<Expr>>, TypeError> {
        let mut out: Normalized<Vec<Expr>> =
            Normalized::from_vec_normalized(Vec::with_capacity(exprs.value.len()));

        for expr in exprs.value.iter() {
            let current_tcon = LazyTypeContext::Snoc(&tcon, out.to_derefed());
            let type_ = self.get_type(expr.clone(), current_tcon, scon)?;
            out.push(type_);
        }

        Ok(out)
    }

    fn get_types_of_independent_expressions(
        &mut self,
        exprs: RcHashed<Box<[Expr]>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<Expr>>, TypeError> {
        let mut out: Normalized<Vec<Expr>> =
            Normalized::from_vec_normalized(Vec::with_capacity(exprs.value.len()));

        for expr in exprs.value.iter() {
            let type_ = self.get_type(expr.clone(), tcon, scon)?;
            out.push(type_);
        }

        Ok(out)
    }

    fn assert_expected_type_equalities_holds_after_applying_scon(
        &mut self,
        equalities: ExpectedTypeEqualities,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        for equality in equalities.zip() {
            self.assert_expected_type_equality_holds_after_applying_scon(equality, scon)?;
        }

        Ok(())
    }

    fn assert_expected_type_equality_holds_after_applying_scon(
        &mut self,
        expected_equality: ExpectedTypeEquality,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let ExpectedTypeEquality {
            expr,
            expected_type,
            actual_type,
            tcon_len,
        } = expected_equality;
        if actual_type.raw().digest() == expected_type.raw().digest() {
            return Ok(());
        }

        let (subbed_expected, subbed_actual) =
            self.apply_scon(scon, tcon_len, expected_type.clone(), actual_type.clone());

        if subbed_expected.raw().digest() == subbed_actual.raw().digest() {
            return Ok(());
        }

        return Err(TypeError::TypeMismatch {
            expr,
            expected_type,
            actual_type,
            subbed_expected,
            subbed_actual,
        });
    }

    fn apply_scon(
        &mut self,
        scon: LazySubstitutionContext,
        tcon_len: usize,
        mut expr1: NormalForm,
        mut expr2: NormalForm,
    ) -> (NormalForm, NormalForm) {
        let mut subs = scon.into_concrete_noncompounded_substitutions(tcon_len);

        loop {
            let HasChanged(has_changed) =
                self.perform_substitution_iteration(&mut subs, &mut expr1, &mut expr2);
            if !has_changed {
                return (expr1, expr2);
            }
        }
    }

    fn perform_substitution_iteration(
        &mut self,
        subs: &mut [ConcreteSubstitution],
        expr1: &mut NormalForm,
        expr2: &mut NormalForm,
    ) -> HasChanged {
        let mut has_changed = HasChanged(false);
        for applied_sub_index in 0..subs.len() {
            let applied_sub = subs[applied_sub_index].clone();
            for target_sub_index in 0..subs.len() {
                if target_sub_index == applied_sub_index {
                    continue;
                }

                has_changed |= self.perform_substitution_on_substitution(
                    &applied_sub,
                    &mut subs[target_sub_index],
                );

                has_changed |= self.perform_substitution_on_expr(&applied_sub, expr1);
                has_changed |= self.perform_substitution_on_expr(&applied_sub, expr2);
            }
        }

        has_changed
    }

    fn perform_substitution_on_substitution(
        &mut self,
        applied_sub: &ConcreteSubstitution,
        target_sub: &mut ConcreteSubstitution,
    ) -> HasChanged {
        let mut has_changed = HasChanged(false);
        has_changed |= self.perform_substitution_on_expr(applied_sub, &mut target_sub.from);
        has_changed |= self.perform_substitution_on_expr(applied_sub, &mut target_sub.to);
        has_changed
    }

    fn perform_substitution_on_expr(
        &mut self,
        applied_sub: &ConcreteSubstitution,
        expr: &mut NormalForm,
    ) -> HasChanged {
        let subbed = expr.raw().clone().substitute(applied_sub);
        let normalized = self.evaluator.eval(subbed);

        if expr.raw().digest() == normalized.raw().digest() {
            return HasChanged(false);
        }

        *expr = normalized;
        HasChanged(true)
    }
}

fn assert_every_expr_is_universe(exprs: &[Expr]) -> Result<(), usize> {
    for (i, expr) in exprs.iter().enumerate() {
        if !expr.is_universe() {
            return Err(i);
        }
    }

    Ok(())
}

impl Expr {
    fn is_universe(&self) -> bool {
        match self {
            Expr::Universe(_) => true,
            _ => false,
        }
    }
}

fn assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
    lhs: &[Expr],
    rhs: UniverseLevel,
) -> Result<(), (usize, UniverseLevel)> {
    for (i, expr) in lhs.iter().enumerate() {
        let lhs_level = match expr {
            Expr::Universe(universe) => universe.value.level,
            _ => continue,
        };

        if lhs_level > rhs {
            return Err((i, lhs_level));
        }
    }

    Ok(())
}

fn get_max_universe_level<'a>(exprs: impl IntoIterator<Item = &'a Expr>) -> Option<UniverseLevel> {
    exprs
        .into_iter()
        .filter_map(|expr| match expr {
            Expr::Universe(universe) => Some(universe.value.level),
            _ => None,
        })
        .max()
}

trait MaxOrSelf: Sized {
    fn max_or_self(self, other: Option<Self>) -> Self;
}

impl<T> MaxOrSelf for T
where
    T: Ord,
{
    fn max_or_self(self, other: Option<Self>) -> Self {
        match other {
            Some(other) => self.max(other),
            None => self,
        }
    }
}

#[derive(Clone, Debug)]
struct ExpectedTypeEquality {
    pub expr: Expr,
    pub expected_type: NormalForm,
    pub actual_type: NormalForm,
    pub tcon_len: usize,
}

/// `exprs`, `expected_types`, and `actual_types` **must** all have the same length.
#[derive(Clone, Debug)]
struct ExpectedTypeEqualities {
    pub exprs: RcHashed<Box<[Expr]>>,
    pub expected_types: Normalized<Vec<Expr>>,
    pub actual_types: Normalized<Vec<Expr>>,
    pub tcon_len: usize,
}

impl ExpectedTypeEqualities {
    pub fn zip(self) -> impl Iterator<Item = ExpectedTypeEquality> {
        let tcon_len = self.tcon_len;
        (0..self.len()).into_iter().map(move |i| {
            let expr = self.exprs.value[i].clone();
            let expected_type = self.expected_types.index(i).cloned();
            let actual_type = self.actual_types.index(i).cloned();
            ExpectedTypeEquality {
                expr,
                expected_type,
                actual_type,
                tcon_len,
            }
        })
    }

    pub fn len(&self) -> usize {
        self.exprs.value.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct HasChanged(pub bool);

impl BitOrAssign for HasChanged {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
