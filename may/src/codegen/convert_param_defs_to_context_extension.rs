use super::*;

impl MayConverter {
    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_param_defs_to_context_extension<'a, D: DashPolicy>(
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
                dash_policy.check(param)?;

                let param_type = self.convert(&param.type_, context)?;
                let entry = self.get_deb_defining_entry(param.name.val());

                Ok((vec![entry], vec![param_type]))
            }

            mnode::CommaSeparatedParamDefs::Snoc(rdc, rac) => {
                dash_policy.check(rac)?;

                let (mut entries, mut param_types) = self
                    .convert_param_defs_to_context_extension_without_finishing(
                        rdc,
                        context,
                        dash_policy,
                    )?;

                let extended_context = Context::Snoc(&context, &entries);
                let rac_type = self.convert(&rac.type_, extended_context)?;

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

    fn check(&mut self, param: &mnode::ParamDef) -> Result<(), SemanticError>;

    fn finish(&mut self) -> Result<Self::Output, SemanticError>;
}

pub struct ForbidDash;

impl DashPolicy for ForbidDash {
    type Output = ();

    fn reset(&mut self) {
        // No-op.
    }

    fn check(&mut self, param: &mnode::ParamDef) -> Result<(), SemanticError> {
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
