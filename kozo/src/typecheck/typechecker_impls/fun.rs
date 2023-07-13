use super::*;

impl TypeChecker {
    pub fn get_type_of_fun(
        &mut self,
        fun: RcHashed<cst::Fun>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types =
            self.get_types_of_dependent_expressions(&fun.value.param_types, tcon, scon)?;
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

        let tcon_with_param_types =
            LazyTypeContext::Snoc(&tcon, normalized_param_types.without_digest().derefed());
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
        let tcon_with_param_and_recursive_fun_param_types = LazyTypeContext::Snoc(
            &tcon_with_param_types,
            recursive_fun_param_type_singleton.as_ref().convert_ref(),
        );

        // TODO: Delete
        // TODO: Print `normalized_param_types_without_digest`.
        println!("****START get_type_of_fun.tcon_dump****\n\n");
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
        println!("****END get_type_of_fun.tcon_dump****\n\n");

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
}
