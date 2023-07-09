use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    hash::sha256::*,
    pretty_print::PrettyPrinted,
    syntax_tree::{
        ast::{self, Deb, RcSemHashed, UniverseLevel},
        rch_cst::{self as cst, RcHashed},
        rch_cst_to_ast::RchCstToAstConverter,
        replace_debs::*,
        token::*,
    },
};

use std::{ops::BitOrAssign, rc::Rc};

mod concrete_substitution;
use concrete_substitution::*;

mod cst_impls;

mod equality_judgment;
use equality_judgment::*;

mod error;
use error::*;

mod scon;
pub use scon::*;

mod tcon;
pub use tcon::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub struct TypeChecker {
    pub evaluator: Evaluator,
    pub cst_converter: RchCstToAstConverter,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TypeChecker {
    pub fn get_type(
        &mut self,
        expr: cst::Expr,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        match expr {
            cst::Expr::Ind(e) => self.get_type_of_ind(e, tcon, scon),
            cst::Expr::Vcon(e) => self.get_type_of_vcon(e, tcon, scon),
            cst::Expr::Match(e) => self.get_type_of_match(e, tcon, scon),
            cst::Expr::Fun(e) => self.get_type_of_fun(e, tcon, scon),
            cst::Expr::App(e) => self.get_type_of_app(e, tcon, scon),
            cst::Expr::For(e) => self.get_type_of_for(e, tcon, scon),
            cst::Expr::Deb(e) => self.get_type_of_deb(e, tcon),
            cst::Expr::Universe(e) => self.get_type_of_universe(e),
        }
    }

    fn get_type_of_ind(
        &mut self,
        ind: RcHashed<cst::Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_ind_precheck(ind.clone(), tcon, scon)?;
        Ok(self.get_ind_type_assuming_ind_is_well_typed(ind))
    }

    fn perform_ind_precheck(
        &mut self,
        ind: RcHashed<cst::Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let index_type_types =
            self.get_types_of_dependent_expressions(ind.value.index_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(&index_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: ind.value.index_types[offender_index].clone(),
                type_: index_type_types.index(offender_index).cloned(),
            }
        })?;

        // Once we verify that the index types are all well-typed,
        // it is safe to construct a predicted type for the ind type.
        let predicted_ind_type = self.get_ind_type_assuming_ind_is_well_typed(ind.clone());

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &index_type_types.raw(),
            UniverseLevel(ind.value.type_.level),
        )
        .map_err(|(offender_index, offender_level)| {
            TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type: ind.value.index_types[offender_index].clone(),
                level: offender_level,
                ind: ind.value.clone(),
            }
        })?;

        self.assert_ind_vcon_defs_are_well_typed(ind, predicted_ind_type, tcon, scon)?;

        Ok(())
    }

    fn assert_ind_vcon_defs_are_well_typed(
        &mut self,
        ind: RcHashed<cst::Ind>,
        predicted_ind_type: NormalForm,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        for def in ind.value.vcon_defs.to_vec() {
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
        ind: RcHashed<cst::Ind>,
        predicted_ind_type: NormalForm,
        def: &cst::VconDef,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let recursive_ind_entry: Normalized<Vec<ast::Expr>> =
            std::iter::once(predicted_ind_type).collect();
        let tcon_with_ind_type = LazyTypeContext::Snoc(&tcon, recursive_ind_entry.to_derefed());
        let param_type_types = self.get_types_of_dependent_expressions(
            def.param_types.clone(),
            tcon_with_ind_type,
            scon,
        )?;

        let param_types_ast = self
            .cst_converter
            .convert_expressions(def.param_types.clone());
        let normalized_param_types = self.evaluator.eval_expressions(param_types_ast);
        let normalized_param_types_without_digest = normalized_param_types.without_digest();
        let tcon_with_ind_and_param_types = LazyTypeContext::Snoc(
            &tcon_with_ind_type,
            normalized_param_types_without_digest.derefed(),
        );
        self.get_types_of_independent_expressions(
            def.index_args.clone(),
            tcon_with_ind_and_param_types,
            scon,
        )?;

        if ind.value.index_types.len() != def.index_args.len() {
            return Err(TypeError::WrongNumberOfIndexArguments {
                def: def.clone(),
                expected: ind.value.index_types.len(),
                actual: def.index_args.len(),
            });
        }

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &param_type_types.raw(),
            UniverseLevel(ind.value.type_.level),
        )
        .map_err(|(offender_index, offender_level)| {
            TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type: def.param_types[offender_index].clone(),
                level: offender_level,
                ind: ind.value.clone(),
            }
        })?;

        self.assert_vcon_def_is_strictly_positive(ind, def, tcon, scon)?;

        Ok(())
    }

    fn assert_vcon_def_is_strictly_positive(
        &mut self,
        _ind: RcHashed<cst::Ind>,
        _def: &cst::VconDef,
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
    fn get_ind_type_assuming_ind_is_well_typed(&mut self, ind: RcHashed<cst::Ind>) -> NormalForm {
        let index_types_ast = self
            .cst_converter
            .convert_expressions(ind.value.index_types.clone());
        let normalized_index_types = self.evaluator.eval_expressions(index_types_ast);
        let return_type = self.get_ind_return_type(ind);
        Normalized::for_(normalized_index_types, return_type).collapse_if_nullary()
    }

    fn get_ind_return_type(&mut self, ind: RcHashed<cst::Ind>) -> NormalForm {
        Normalized::universe(ast::UniverseNode {
            level: UniverseLevel(ind.value.type_.level),
        })
    }

    fn get_type_of_vcon(
        &mut self,
        vcon: RcHashed<cst::Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_vcon_precheck(vcon.clone(), tcon, scon)?;

        let vcon_index = vcon.value.vcon_index;
        let defs: &cst::ZeroOrMoreVconDefs = &vcon.value.ind.value.vcon_defs;
        let Some(def) = defs.get(vcon_index.value) else {
            return Err(TypeError::InvalidVconIndex(vcon.value.clone()));
        };
        self.get_type_of_trusted_vcon_def(def, vcon.value.ind.clone())
    }

    fn perform_vcon_precheck(
        &mut self,
        vcon: RcHashed<cst::Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        self.get_type_of_ind(vcon.value.ind.clone(), tcon, scon)?;
        Ok(())
    }

    fn get_type_of_trusted_vcon_def(
        &mut self,
        def: &cst::VconDef,
        ind: RcHashed<cst::Ind>,
    ) -> Result<NormalForm, TypeError> {
        let unsubstituted_param_types_ast = self
            .cst_converter
            .convert_expressions(def.param_types.clone());
        let unsubstituted_index_args_ast = self
            .cst_converter
            .convert_expressions(def.index_args.clone());
        let ind_ast = self.cst_converter.convert_ind(ind);
        let normalized_ind = self.evaluator.eval_ind(ind_ast);

        let ind_singleton: [ast::Expr; 1] = [normalized_ind.raw().clone().into()];
        let ind_singleton_deb_substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };

        let substituted_param_types_ast = ind_singleton_deb_substituter
            .replace_debs_in_expressions_with_increasing_cutoff(unsubstituted_param_types_ast, 0);
        let normalized_param_types = self.evaluator.eval_expressions(substituted_param_types_ast);

        let param_count = def.param_types.len();
        let substituted_index_args_ast = ind_singleton_deb_substituter
            .replace_debs_in_expressions_with_constant_cutoff(
                unsubstituted_index_args_ast,
                param_count,
            );
        let normalized_index_args = self.evaluator.eval_expressions(substituted_index_args_ast);
        let shifted_normalized_ind = normalized_ind.upshift(param_count);
        let return_type =
            Normalized::app_with_ind_callee(shifted_normalized_ind, normalized_index_args)
                .collapse_if_nullary();
        Ok(Normalized::for_(normalized_param_types, return_type).collapse_if_nullary())
    }

    fn get_type_of_match(
        &mut self,
        match_: RcHashed<cst::Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_match_precheck(match_.clone(), tcon, scon)?;

        let return_type_ast = self.cst_converter.convert(match_.value.return_type.clone());
        let normalized_return_type = self.evaluator.eval(return_type_ast);
        Ok(normalized_return_type)
    }

    fn perform_match_precheck(
        &mut self,
        match_: RcHashed<cst::Match>,
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
        let match_case_count = match_.value.cases.len();
        if vcon_count != match_case_count {
            return Err(TypeError::WrongNumberOfMatchCases {
                match_: match_.value.clone(),
                matchee_type_ind: well_typed_matchee_type_ind.without_digest().cloned(),
            });
        }

        let return_type_ast = self.cst_converter.convert(match_.value.return_type.clone());
        let unshifted_normalized_match_return_type = self.evaluator.eval(return_type_ast);
        self.perform_match_cases_precheck(
            match_,
            unshifted_normalized_match_return_type,
            well_typed_matchee_type_ind,
            well_typed_matchee_type_args,
            tcon,
            scon,
        )?;

        Ok(())
    }

    fn perform_match_cases_precheck(
        &mut self,
        match_: RcHashed<cst::Match>,
        unshifted_match_return_type: NormalForm,
        well_typed_matchee_type_ind: Normalized<RcSemHashed<ast::Ind>>,
        well_typed_matchee_type_args: Normalized<RcSemHashed<Box<[ast::Expr]>>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let vcon_defs = well_typed_matchee_type_ind.without_digest().vcon_defs();
        let vcon_defs = vcon_defs.without_digest();
        let vcon_defs = vcon_defs.derefed();

        for match_case_index in 0..match_.value.cases.len() {
            let well_typed_vcon_def = vcon_defs.index(match_case_index);
            let match_case = &match_.value.cases[match_case_index];
            self.perform_match_case_precheck(
                match_case,
                match_case_index,
                well_typed_vcon_def,
                match_.clone(),
                unshifted_match_return_type.clone(),
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
        match_case: &cst::MatchCase,
        match_case_index: usize,
        well_typed_vcon_def: Normalized<&ast::VconDef>,
        match_: RcHashed<cst::Match>,
        unshifted_match_return_type: NormalForm,
        well_typed_matchee_type_ind: Normalized<RcSemHashed<ast::Ind>>,
        well_typed_matchee_type_args: Normalized<RcSemHashed<Box<[ast::Expr]>>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let ind_singleton: [ast::Expr; 1] = [well_typed_matchee_type_ind.clone().into_raw().into()];
        let ind_singleton_deb_substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };

        let actual_arity = match_case.arity.value;
        let expected_arity = well_typed_vcon_def.raw().param_types.value.len();
        if actual_arity != expected_arity {
            return Err(TypeError::WrongMatchCaseArity {
                actual_node: match_case.arity.clone(),
                actual: actual_arity,
                expected: expected_arity,
                match_: match_.value.clone(),
                match_case_index,
            });
        }

        let match_case_param_types = ind_singleton_deb_substituter
            .replace_debs_in_expressions_with_increasing_cutoff(
                well_typed_vcon_def.raw().param_types.clone(),
                0,
            );
        let match_case_param_types = self.evaluator.eval_expressions(match_case_param_types);
        let match_case_param_types = match_case_param_types.without_digest();
        let tcon_with_match_case_param_types =
            LazyTypeContext::Snoc(&tcon, match_case_param_types.derefed());

        let match_case_param_count = match_case_param_types.raw().len();
        let substituted_vcon_index_args = well_typed_vcon_def
            .index_args()
            .replace_deb0_with_ind_with_increasing_cutoff(well_typed_matchee_type_ind.clone());
        let upshifted_matchee_type_args = well_typed_matchee_type_args
            .clone()
            .upshift_expressions_with_constant_cutoff(match_case_param_count);
        let extended_tcon_len = tcon_with_match_case_param_types.len();
        let matchee_ast = self.cst_converter.convert(match_.value.matchee.clone());
        let upshifted_matchee = DebUpshifter(match_case_param_count).replace_debs(matchee_ast, 0);
        let upshifted_normalized_matchee = self.evaluator.eval(upshifted_matchee);
        let parameterized_vcon_capp = Normalized::vcon_capp(
            well_typed_matchee_type_ind,
            match_case_index,
            match_case_param_count,
        );
        let new_substitutions: Vec<LazySubstitution> =
            (0..substituted_vcon_index_args.raw().value.len())
                .map(|i| {
                    // TODO: Upshift `substituted_vcon_index_args`
                    // by `match_case_param_count` WITH A CUTOFF OF
                    // `match_case_param_count`.
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

        let match_case_return_type = self.get_type(
            match_case.return_val.clone(),
            tcon_with_match_case_param_types,
            extended_scon,
        )?;

        let shifted_match_return_type = unshifted_match_return_type.upshift(expected_arity);
        // TODO: Replace with normal `?` syntax.
        let res = self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: match_case.return_val.clone(),
                expected_type: shifted_match_return_type,
                actual_type: match_case_return_type.clone(),
                tcon_len: extended_tcon_len,
            },
            extended_scon,
        );
        if let Err(err) = res {
            println!(
                "****match_case_param_types.len:****\n{}\n\n",
                match_case_param_types.raw().len()
            );
            for raw_deb in 0..tcon.len() {
                println!(
                    "****tcon[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(tcon.get(Deb(raw_deb)).unwrap().raw())
                );
                println!(
                    "****tcon.UNSHIFTED[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(tcon.get_unshifted(Deb(raw_deb)).unwrap().raw())
                );
            }

            for raw_deb in 0..tcon_with_match_case_param_types.len() {
                println!(
                    "****tcon_with_match_case_param_types[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(
                        tcon_with_match_case_param_types
                            .get(Deb(raw_deb))
                            .unwrap()
                            .raw()
                    )
                );
                println!(
                    "****tcon_with_match_case_param_types.UNSHIFTED[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(
                        tcon_with_match_case_param_types
                            .get_unshifted(Deb(raw_deb))
                            .unwrap()
                            .raw()
                    )
                );
            }
            return Err(err);
        }

        Ok(())
    }

    fn get_type_of_fun(
        &mut self,
        fun: RcHashed<cst::Fun>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types =
            self.get_types_of_dependent_expressions(fun.value.param_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(param_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: fun.value.param_types[offender_index].clone(),
                type_: param_type_types.index(offender_index).cloned(),
            }
        })?;
        let param_types_ast = self
            .cst_converter
            .convert_expressions(fun.value.param_types.clone());
        let normalized_param_types = self.evaluator.eval_expressions(param_types_ast);

        let normalized_param_types_without_digest = normalized_param_types.without_digest();
        let tcon_with_param_types =
            LazyTypeContext::Snoc(&tcon, normalized_param_types_without_digest.derefed());
        let return_type_type =
            self.get_type(fun.value.return_type.clone(), tcon_with_param_types, scon)?;
        if !return_type_type.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression {
                expr: fun.value.return_type.clone(),
                type_: return_type_type,
            });
        }
        let return_type_ast = self.cst_converter.convert(fun.value.return_type.clone());
        let unshifted_normalized_return_type = self.evaluator.eval(return_type_ast);

        let only_possible_fun_type: NormalForm = Normalized::for_(
            normalized_param_types.clone(),
            unshifted_normalized_return_type.clone(),
        )
        .into();

        let shifted_fun_type = only_possible_fun_type
            .clone()
            .upshift(normalized_param_types.raw().value.len() + 1);
        let recursive_fun_param_type_singleton =
            Normalized::<[ast::Expr; 1]>::new(shifted_fun_type.clone());
        let recursive_fun_param_type_singleton_ref = recursive_fun_param_type_singleton.as_ref();
        let tcon_with_param_and_recursive_fun_param_types = LazyTypeContext::Snoc(
            &tcon_with_param_types,
            recursive_fun_param_type_singleton_ref.convert(),
        );

        let return_val_type = self.get_type(
            fun.value.return_val.clone(),
            tcon_with_param_and_recursive_fun_param_types,
            scon,
        )?;

        let shifted_normalized_return_type = unshifted_normalized_return_type.upshift(1);
        self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: fun.value.return_val.clone(),
                expected_type: shifted_normalized_return_type.clone(),
                actual_type: return_val_type,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        Ok(only_possible_fun_type)
    }

    fn get_type_of_app(
        &mut self,
        app: RcHashed<cst::App>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let callee_type = self
            .get_type(app.value.callee.clone(), tcon, scon)?
            .try_into_for()
            .map_err(|original| TypeError::CalleeTypeIsNotAForExpression {
                app: app.value.clone(),
                callee_type: original,
            })?;

        let arg_count = app.value.args.len();
        let param_count = callee_type.raw().value.param_types.value.len();
        if arg_count != param_count {
            return Err(TypeError::WrongNumberOfAppArguments {
                app: app.value.clone(),
                callee_type: callee_type.without_digest().cloned(),
                expected: param_count,
                actual: arg_count,
            });
        }

        let arg_types =
            self.get_types_of_independent_expressions(app.value.args.clone(), tcon, scon)?;
        let args_ast = self
            .cst_converter
            .convert_expressions(app.value.args.clone());
        let normalized_args = self.evaluator.eval_expressions(args_ast);

        let substituted_param_types = self.substitute_param_types(
            callee_type.without_digest().param_types(),
            normalized_args.clone(),
        );
        self.assert_expected_type_equalities_holds_after_applying_scon(
            ExpectedTypeEqualities {
                exprs: app.value.args.to_vec_of_cloned(),
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
        unsubstituted_param_types: Normalized<RcSemHashed<Box<[ast::Expr]>>>,
        normalized_args: Normalized<RcSemHashed<Box<[ast::Expr]>>>,
    ) -> Normalized<Vec<ast::Expr>> {
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
        for_: RcHashed<cst::For>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types =
            self.get_types_of_dependent_expressions(for_.value.param_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(param_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: for_.value.param_types[offender_index].clone(),
                type_: param_type_types.index(offender_index).cloned(),
            }
        })?;

        let param_types_ast = self
            .cst_converter
            .convert_expressions(for_.value.param_types.clone());
        let normalized_param_types = self.evaluator.eval_expressions(param_types_ast);
        let normalized_param_types_without_digest = normalized_param_types.without_digest();
        let tcon_with_param_types =
            LazyTypeContext::Snoc(&tcon, normalized_param_types_without_digest.derefed());
        let return_type_type =
            self.get_type(for_.value.return_type.clone(), tcon_with_param_types, scon)?;
        let return_type_type_universe_level = match return_type_type.raw() {
            ast::Expr::Universe(universe_node) => universe_node.value.level,

            _ => {
                return Err(TypeError::UnexpectedNonTypeExpression {
                    expr: for_.value.return_type.clone(),
                    type_: return_type_type,
                })
            }
        };

        let max_level = return_type_type_universe_level
            .max_or_self(get_max_universe_level(param_type_types.raw()));
        Ok(Normalized::universe(ast::UniverseNode { level: max_level }))
    }

    fn get_type_of_deb(
        &mut self,
        deb: RcHashed<NumberLiteral>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        if let Some(expr) = tcon.get(Deb(deb.value.value)) {
            return Ok(expr);
        }

        return Err(TypeError::InvalidDeb {
            deb: deb.value.clone(),
            tcon_len: tcon.len(),
        });
    }

    fn get_type_of_universe(
        &mut self,
        universe: RcHashed<UniverseLiteral>,
    ) -> Result<NormalForm, TypeError> {
        return Ok(self.evaluator.eval(ast::Expr::Universe(Rc::new(Hashed::new(
            ast::UniverseNode {
                level: UniverseLevel(universe.value.level + 1),
            },
        )))));
    }

    fn get_types_of_dependent_expressions(
        &mut self,
        exprs: cst::ZeroOrMoreExprs,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<ast::Expr>>, TypeError> {
        let mut out: Normalized<Vec<ast::Expr>> =
            Normalized::from_vec_normalized(Vec::with_capacity(exprs.len()));
        let mut normalized_visited_exprs: Normalized<Vec<ast::Expr>> =
            Normalized::from_vec_normalized(Vec::with_capacity(exprs.len()));

        for expr in exprs.to_vec() {
            let current_tcon = LazyTypeContext::Snoc(&tcon, normalized_visited_exprs.to_derefed());
            let type_ = self.get_type(expr.clone(), current_tcon, scon)?;
            out.push(type_);

            let expr_ast = self.cst_converter.convert(expr.clone());
            let normalized = self.evaluator.eval(expr_ast);
            normalized_visited_exprs.push(normalized);
        }

        Ok(out)
    }

    fn get_types_of_independent_expressions(
        &mut self,
        exprs: cst::ZeroOrMoreExprs,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<ast::Expr>>, TypeError> {
        let mut out: Normalized<Vec<ast::Expr>> =
            Normalized::from_vec_normalized(Vec::with_capacity(exprs.len()));

        for expr in exprs.to_vec() {
            let type_ = self.get_type(expr.clone(), tcon, scon)?;
            out.push(type_);
        }

        Ok(out)
    }
}

fn assert_every_expr_is_universe(exprs: &[ast::Expr]) -> Result<(), usize> {
    for (i, expr) in exprs.iter().enumerate() {
        if !expr.is_universe() {
            return Err(i);
        }
    }

    Ok(())
}

impl ast::Expr {
    fn is_universe(&self) -> bool {
        match self {
            ast::Expr::Universe(_) => true,
            _ => false,
        }
    }
}

fn assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
    lhs: &[ast::Expr],
    rhs: UniverseLevel,
) -> Result<(), (usize, UniverseLevel)> {
    for (i, expr) in lhs.iter().enumerate() {
        let lhs_level = match expr {
            ast::Expr::Universe(universe) => universe.value.level,
            _ => continue,
        };

        if lhs_level > rhs {
            return Err((i, lhs_level));
        }
    }

    Ok(())
}

fn get_max_universe_level<'a>(
    exprs: impl IntoIterator<Item = &'a ast::Expr>,
) -> Option<UniverseLevel> {
    exprs
        .into_iter()
        .filter_map(|expr| match expr {
            ast::Expr::Universe(universe) => Some(universe.value.level),
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
