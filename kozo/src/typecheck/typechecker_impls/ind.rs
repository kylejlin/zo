use super::*;

impl TypeChecker {
    pub fn get_type_of_ind(
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
}
