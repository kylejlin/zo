use super::*;

impl TypeChecker {
    pub fn get_type_of_vcon<A: AstFamily>(
        &mut self,
        vcon: RcHashed<ast::Vcon<A>>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        self.assert_vcon_index_is_valid(vcon.clone())?;

        let normalized_ind = self.typecheck_and_normalize_ind(vcon.hashee.ind.clone(), tcon)?;

        let vcon_index = vcon.hashee.vcon_index;
        Ok(
            self.get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
                normalized_ind.clone(),
                vcon_index,
            ),
        )
    }

    fn assert_vcon_index_is_valid<A: AstFamily>(
        &mut self,
        vcon: RcHashed<ast::Vcon<A>>,
    ) -> Result<(), TypeError<A>> {
        let vcon_index = vcon.hashee.vcon_index;
        let defs = &vcon.hashee.ind.hashee.vcon_defs;
        if vcon_index >= defs.hashee.len() {
            return Err(TypeError::InvalidVconIndex(vcon.hashee.clone()));
        }
        Ok(())
    }

    fn typecheck_and_normalize_ind<A: AstFamily>(
        &mut self,
        ind: RcHashed<ast::Ind<A>>,
        tcon: LazyTypeContext,
    ) -> Result<Normalized<RcHashed<minimal_ast::Ind>>, TypeError<A>> {
        self.get_type_of_ind(ind.clone(), tcon)?;

        let ind_minimal = self.aux_remover.convert_ind(ind);
        let normalized = self.evaluator.eval_ind(ind_minimal);
        Ok(normalized)
    }

    pub fn get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
        &mut self,
        ind: Normalized<RcHashed<minimal_ast::Ind>>,
        vcon_index: usize,
    ) -> NormalForm {
        let defs = ind.to_hashee().vcon_defs().hashee().derefed();
        let def: Normalized<&minimal_ast::VconDef> = defs.index_ref(vcon_index);

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
