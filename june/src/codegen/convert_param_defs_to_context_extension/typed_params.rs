use super::*;

impl JuneConverter {
    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_optional_typed_nonfun_param_defs_to_context_extension<'a>(
        &mut self,
        params: Option<&'a mnode::CommaSeparatedNonfunParamDefs>,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>), SemanticError> {
        if let Some(params) = params {
            let (entries, param_types) =
                self.convert_typed_nonfun_param_defs_to_context_extension(params, context)?;
            Ok((entries, param_types))
        } else {
            Ok((vec![], vec![]))
        }
    }

    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_typed_nonfun_param_defs_to_context_extension<'a>(
        &mut self,
        params: &'a mnode::CommaSeparatedNonfunParamDefs,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>), SemanticError> {
        match params {
            mnode::CommaSeparatedNonfunParamDefs::One(param) => {
                let param_type = self.convert(&param.type_, context)?;
                let entry = self.get_deb_defining_entry(param.name.val());

                Ok((vec![entry], vec![param_type]))
            }

            mnode::CommaSeparatedNonfunParamDefs::Snoc(rdc, rac) => {
                let (mut entries, mut param_types) =
                    self.convert_typed_nonfun_param_defs_to_context_extension(rdc, context)?;

                let extended_context = Context::Snoc(&context, &entries);
                let rac_type = self.convert(&rac.type_, extended_context)?;

                entries.push(self.get_deb_defining_entry(rac.name.val()));
                param_types.push(rac_type);
                Ok((entries, param_types))
            }
        }
    }
}

//  TODO: Delete.
// impl JuneConverter {
//     /// If the params are valid,
//     /// this function returns `Ok((entries, param_types, dash_index))`.
//     pub(crate) fn convert_optional_typed_fun_param_defs_to_context_extension<'a>(
//         &mut self,
//         params: Option<&'a mnode::CommaSeparatedFunParamDefs>,
//         context: Context,
//     ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, Option<usize>), SemanticError> {
//         if let Some(params) = params {
//             self.convert_fun_param_defs_to_context_extension(params, context, None)
//         } else {
//             Ok((vec![], vec![], None))
//         }
//     }

//     /// If the params are valid,
//     /// this function returns `Ok((entries, param_types, dash_index))`.
//     fn convert_fun_param_defs_to_context_extension<'a>(
//         &mut self,
//         params: &'a mnode::CommaSeparatedFunParamDefs,
//         context: Context,
//         previous_dec_param_and_index: Option<(&mnode::FunParamDef, usize)>,
//     ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, Option<usize>), SemanticError> {
//         match params {
//             mnode::CommaSeparatedFunParamDefs::One(param) => {
//                 if let (Some((existing_param, _)), mnode::OptDecKw::Some(_)) =
//                     (previous_dec_param_and_index, &*param.deckw)
//                 {
//                     return Err(SemanticError::MultipleDashedParams(
//                         existing_param.clone(),
//                         *param.clone(),
//                     ));
//                 }

//                 let param_type = self.convert(&param.type_, context)?;
//                 let entry = self.get_deb_defining_entry(param.name.val());

//                 Ok((vec![entry], vec![param_type]))
//             }

//             mnode::CommaSeparatedFunParamDefs::Snoc(rdc, rac) => {
//                 if let (Some(existing_param), mnode::OptDecKw::Some(_)) =
//                     (previous_dec_param_and_index, &*rac.deckw)
//                 {
//                     return Err(SemanticError::MultipleDashedParams(
//                         existing_param.clone(),
//                         *rac.clone(),
//                     ));
//                 }

//                 let previous_dec_param = previous_dec_param_and_index.or_else(|| {
//                     if let mnode::OptDecKw::Some(_) = &*rac.deckw {
//                         Some(rac)
//                     } else {
//                         None
//                     }
//                 });

//                 let (mut entries, mut param_types) = self
//                     .convert_fun_param_defs_to_context_extension(
//                         rdc,
//                         context,
//                         previous_dec_param,
//                     )?;

//                 let extended_context = Context::Snoc(&context, &entries);
//                 let rac_type = self.convert(&rac.type_, extended_context)?;

//                 entries.push(self.get_deb_defining_entry(rac.name.val()));
//                 param_types.push(rac_type);
//                 Ok((entries, param_types))
//             }
//         }
//     }
// }

impl JuneConverter {
    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_optional_typed_fun_param_defs_to_context_extension<'a>(
        &mut self,
        params: Option<&'a mnode::CommaSeparatedFunParamDefs>,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, Option<usize>), SemanticError> {
        todo!()
    }

    pub(crate) fn convert_typed_fun_param_defs_to_context_extension<'a>(
        &mut self,
        params: &'a mnode::CommaSeparatedFunParamDefs,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, Option<usize>), SemanticError> {
        todo!()
    }
}
