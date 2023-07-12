use super::*;

impl TypeChecker {
    pub fn get_type_of_vcon(
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
}
