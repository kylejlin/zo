use super::*;

impl TypeChecker {
    pub fn get_type_of_vcon(
        &mut self,
        vcon: RcHashed<cst::Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.assert_vcon_index_is_valid(vcon.clone())?;

        let normalized_ind =
            self.typecheck_and_normalize_ind(vcon.hashee.ind.clone(), tcon, scon)?;

        let vcon_index = vcon.hashee.vcon_index.value;
        Ok(
            self.get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
                normalized_ind.clone(),
                vcon_index,
            ),
        )
    }

    fn assert_vcon_index_is_valid(&mut self, vcon: RcHashed<cst::Vcon>) -> Result<(), TypeError> {
        let vcon_index = vcon.hashee.vcon_index.value;
        let defs: &cst::ZeroOrMoreVconDefs = &vcon.hashee.ind.hashee.vcon_defs;
        if vcon_index >= defs.len() {
            return Err(TypeError::InvalidVconIndex(vcon.hashee.clone()));
        }
        Ok(())
    }

    fn typecheck_and_normalize_ind(
        &mut self,
        ind: RcHashed<cst::Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<RcSemHashed<ast::Ind>>, TypeError> {
        self.get_type_of_ind(ind.clone(), tcon, scon)?;

        let ind_ast = self.cst_converter.convert_ind(ind);
        let normalized = self.evaluator.eval_ind(ind_ast);
        Ok(normalized)
    }

    pub(in crate::typecheck::typechecker_impls) fn get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
        &mut self,
        ind: Normalized<RcSemHashed<ast::Ind>>,
        vcon_index: usize,
    ) -> NormalForm {
        let defs = ind.to_hashee().vcon_defs().hashee().derefed();
        let def: Normalized<&ast::VconDef> = defs.index_ref(vcon_index);

        let substituted_downshifted_param_types = def
            .param_types()
            .cloned()
            .replace_deb0_with_ind_with_increasing_cutoff(ind.clone(), 0);

        let param_count = def.raw().param_types.hashee.len();
        let substituted_downshifted_index_args = def
            .index_args()
            .cloned()
            .replace_deb0_with_ind_with_constant_cutoff(ind.clone(), param_count);

        let upshifted_ind = ind.upshift(param_count, 0);
        let capp =
            Normalized::app_with_ind_callee(upshifted_ind, substituted_downshifted_index_args)
                .collapse_if_nullary();
        Normalized::for_(substituted_downshifted_param_types, capp).collapse_if_nullary()
    }
}
