use super::*;

impl MayConverter {
    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_optional_typed_param_defs_to_context_extension<'a, D: DashPolicy>(
        &mut self,
        params: Option<&'a mnode::CommaSeparatedParamDefs>,
        context: Context,
        dash_policy: D,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, Option<D::Output>), SemanticError> {
        if let Some(params) = params {
            let (entries, param_types, dash_index) =
                self.convert_typed_param_defs_to_context_extension(params, context, dash_policy)?;
            Ok((entries, param_types, Some(dash_index)))
        } else {
            Ok((vec![], vec![], None))
        }
    }

    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_typed_param_defs_to_context_extension<'a, D: DashPolicy>(
        &mut self,
        params: &'a mnode::CommaSeparatedParamDefs,
        context: Context,
        mut dash_policy: D,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, D::Output), SemanticError> {
        dash_policy.reset();

        let (entries, param_types) = self
            .convert_param_defs_to_context_extension_without_finishing(
                params,
                context,
                &mut dash_policy,
            )?;

        let dash_index = dash_policy.finish()?;

        Ok((entries, param_types, dash_index))
    }

    fn convert_param_defs_to_context_extension_without_finishing<'a, D: DashPolicy>(
        &mut self,
        params: &'a mnode::CommaSeparatedParamDefs,
        context: Context,
        dash_policy: &mut D,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>), SemanticError> {
        match params {
            mnode::CommaSeparatedParamDefs::One(param) => {
                dash_policy.check(param, 0)?;

                let (param_type, _) = self.convert(&param.type_, context, &DropContext)?;
                let entry = self.get_deb_defining_entry(param.name.val());

                Ok((vec![entry], vec![param_type]))
            }

            mnode::CommaSeparatedParamDefs::Snoc(rdc, rac) => {
                let (mut entries, mut param_types) = self
                    .convert_param_defs_to_context_extension_without_finishing(
                        rdc,
                        context,
                        dash_policy,
                    )?;

                dash_policy.check(rac, param_types.len())?;

                let extended_context = Context::Snoc(&context, &entries);
                let (rac_type, _) = self.convert(&rac.type_, extended_context, &DropContext)?;

                entries.push(self.get_deb_defining_entry(rac.name.val()));
                param_types.push(rac_type);
                Ok((entries, param_types))
            }
        }
    }
}

pub trait DashPolicy {
    type Output;

    fn reset(&mut self);

    fn check(&mut self, param: &mnode::ParamDef, param_index: usize) -> Result<(), SemanticError>;

    fn finish(&mut self) -> Result<Self::Output, SemanticError>;
}

pub struct ForbidDash;

impl DashPolicy for ForbidDash {
    type Output = ();

    fn reset(&mut self) {
        // No-op.
    }

    fn check(&mut self, param: &mnode::ParamDef, _: usize) -> Result<(), SemanticError> {
        match &*param.dash {
            mnode::OptDash::None => Ok(()),
            mnode::OptDash::Some(_) => Err(SemanticError::IllegalDashedParam(param.clone())),
        }
    }

    fn finish(&mut self) -> Result<Self::Output, SemanticError> {
        // No-op.
        Ok(())
    }
}

pub struct AtMostOneDash {
    dashed_index_and_param: Option<(usize, mnode::ParamDef)>,
}

impl Default for AtMostOneDash {
    fn default() -> Self {
        Self {
            dashed_index_and_param: None,
        }
    }
}

impl DashPolicy for AtMostOneDash {
    type Output = Option<usize>;

    fn reset(&mut self) {
        self.dashed_index_and_param = None;
    }

    fn check(&mut self, param: &mnode::ParamDef, param_index: usize) -> Result<(), SemanticError> {
        match &*param.dash {
            mnode::OptDash::None => Ok(()),
            mnode::OptDash::Some(_) => {
                if let Some((_, existing_param)) = self.dashed_index_and_param.as_ref() {
                    return Err(SemanticError::MultipleDashedParams(
                        existing_param.clone(),
                        param.clone(),
                    ));
                }

                self.dashed_index_and_param = Some((param_index, param.clone()));
                Ok(())
            }
        }
    }

    fn finish(&mut self) -> Result<Self::Output, SemanticError> {
        Ok(self
            .dashed_index_and_param
            .as_ref()
            .map(|(index, _)| *index))
    }
}
