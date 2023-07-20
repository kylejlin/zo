use super::*;

impl TypeChecker {
    pub fn get_type_of_fun(
        &mut self,
        fun_g0: RcHashed<cst::Fun>,
        tcon_g0: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let normalized_param_types_g0 = self.typecheck_and_normalize_param_types_with_limit(
            &fun_g0.hashee.param_types,
            NoLimit,
            tcon_g0,
            scon,
        )?;
        let param_count = normalized_param_types_g0.raw().len();

        let tcon_with_param_types_g1 =
            LazyTypeContext::Snoc(&tcon_g0, normalized_param_types_g0.to_derefed());

        let normalized_return_type_g1 = self.assert_expr_type_is_universe_and_then_eval(
            fun_g0.hashee.return_type.clone(),
            tcon_with_param_types_g1,
            scon,
        )?;

        let normalized_param_types_g1 = normalized_param_types_g0
            .clone()
            .upshift_with_increasing_cutoff(param_count);

        let normalized_return_type_g1f = normalized_return_type_g1
            .clone()
            .upshift(param_count, param_count);

        let fun_type_g1: NormalForm = Normalized::for_(
            normalized_param_types_g1.clone().into_rc_hashed(),
            normalized_return_type_g1f.clone(),
        )
        .into();

        let fun_type_g1_singleton = Normalized::<[_; 1]>::new(fun_type_g1.clone());
        let tcon_with_param_types_and_fun_types_g2 = LazyTypeContext::Snoc(
            &tcon_with_param_types_g1,
            fun_type_g1_singleton.as_ref().convert_ref(),
        );

        let normalized_return_type_g2 = normalized_return_type_g1.clone().upshift(1, 0);

        let return_val_type_g2 = self.get_type(
            fun_g0.hashee.return_val.clone(),
            tcon_with_param_types_and_fun_types_g2,
            scon,
        )?;

        self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: fun_g0.hashee.return_val.clone(),
                expected_type: normalized_return_type_g2,
                actual_type: return_val_type_g2,
                tcon_len: tcon_with_param_types_and_fun_types_g2.len(),
            },
            scon,
        )?;

        Ok(Normalized::for_(
            normalized_param_types_g0.into_rc_hashed(),
            normalized_return_type_g1,
        )
        .into())
    }
}
