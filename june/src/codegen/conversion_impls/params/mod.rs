use super::*;

impl JuneConverter {
    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_typed_nonfun_param_defs_to_context_extension<'a>(
        &mut self,
        params: &'a jnode::CommaSeparatedNonfunParamDefs,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>), SemanticError> {
        match params {
            jnode::CommaSeparatedNonfunParamDefs::One(param) => {
                let param_type = self.convert(&param.type_, context)?;
                let entry = self.get_deb_defining_entry(param.name.val());

                Ok((vec![entry], vec![param_type]))
            }

            jnode::CommaSeparatedNonfunParamDefs::Snoc(rdc, rac) => {
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

impl JuneConverter {
    pub(crate) fn get_deb_defining_entry<'a>(&mut self, key: &'a str) -> UnshiftedEntry<'a> {
        let val = self.cache_deb(znode::DebNode {
            deb: Deb(0),
            aux_data: (),
        });
        UnshiftedEntry {
            key,
            val,
            is_deb: true,
        }
    }
}
