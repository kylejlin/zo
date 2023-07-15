use super::*;

impl TypeChecker {
    pub fn get_type_of_fun(
        &mut self,
        fun: RcHashed<cst::Fun>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let normalized_param_types = self.typecheck_and_normalize_param_types_with_limit(
            &fun.hashee.param_types,
            NoLimit,
            tcon,
            scon,
        )?;

        let tcon_with_param_types =
            LazyTypeContext::Snoc(&tcon, normalized_param_types.to_derefed());
        let return_type_type =
            self.get_type(fun.hashee.return_type.clone(), tcon_with_param_types, scon)?;
        if !return_type_type.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression {
                expr: fun.hashee.return_type.clone(),
                type_: return_type_type,
            });
        }
        let return_type_ast = self.cst_converter.convert(fun.hashee.return_type.clone());
        let unshifted_normalized_return_type = self.evaluator.eval(return_type_ast);

        let only_possible_fun_type: NormalForm = Normalized::for_(
            normalized_param_types.clone().into_rc_sem_hashed(),
            unshifted_normalized_return_type.clone(),
        )
        .into();

        let shifted_fun_type = only_possible_fun_type
            .clone()
            .upshift(normalized_param_types.raw().len() + 1);
        let recursive_fun_param_type_singleton =
            Normalized::<[ast::Expr; 1]>::new(shifted_fun_type.clone());
        let tcon_with_param_and_recursive_fun_param_types = LazyTypeContext::Snoc(
            &tcon_with_param_types,
            recursive_fun_param_type_singleton.as_ref().convert_ref(),
        );

        let return_val_type = self.get_type(
            fun.hashee.return_val.clone(),
            tcon_with_param_and_recursive_fun_param_types,
            scon,
        )?;

        let shifted_normalized_return_type = unshifted_normalized_return_type.upshift(1);
        self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: fun.hashee.return_val.clone(),
                expected_type: shifted_normalized_return_type.clone(),
                actual_type: return_val_type,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        Ok(only_possible_fun_type)
    }
}
