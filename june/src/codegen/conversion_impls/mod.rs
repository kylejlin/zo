use zoc::eval::NormalForm;

use super::*;

mod chain_enum_def;
mod chain_fun_def;
mod chain_var_def;

mod for_;
mod match_;
mod universe;
mod var_or_app;

mod params;

impl JuneConverter {
    pub(crate) fn convert(
        &mut self,
        expr: &jnode::Expr,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            jnode::Expr::VarDef(e) => self.convert_chain_var_def(e, context),
            jnode::Expr::EnumDef(e) => self.convert_chain_enum_def(e, context),
            jnode::Expr::FunDef(e) => self.convert_chain_fun_def(e, context),

            jnode::Expr::Match(e) => self.convert_match(e, context),
            jnode::Expr::For(e) => self.convert_for(e, context),
            jnode::Expr::VarOrApp(e) => self.convert_var_or_app(e, context),
            jnode::Expr::Universe(e) => self.convert_universe(e, context),
        }
    }
}

impl JuneConverter {
    fn convert_optional_exprs(
        &mut self,
        exprs: Option<&jnode::CommaSeparatedExprs>,
        context: Context,
    ) -> Result<RcHashedVec<znode::Expr>, SemanticError> {
        if let Some(exprs) = exprs {
            self.convert_exprs(exprs, context)
        } else {
            Ok(self.cache_expr_vec(vec![]))
        }
    }

    fn convert_exprs(
        &mut self,
        exprs: &jnode::CommaSeparatedExprs,
        context: Context,
    ) -> Result<RcHashedVec<znode::Expr>, SemanticError> {
        let v = self.convert_exprs_without_hashing_vec(exprs, context)?;
        Ok(self.cache_expr_vec(v))
    }

    fn convert_exprs_without_hashing_vec(
        &mut self,
        exprs: &jnode::CommaSeparatedExprs,
        context: Context,
    ) -> Result<Vec<znode::Expr>, SemanticError> {
        match exprs {
            jnode::CommaSeparatedExprs::One(e) => {
                let e = self.convert(e, context)?;
                Ok(vec![e])
            }
            jnode::CommaSeparatedExprs::Snoc(rdc, rac) => {
                let mut rdc = self.convert_exprs_without_hashing_vec(rdc, context)?;
                let rac = self.convert(rac, context)?;
                rdc.push(rac);
                Ok(rdc)
            }
        }
    }
}

impl JuneConverter {
    fn convert_and_typecheck(
        &mut self,
        expr: &jnode::Expr,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let converted = self.convert(expr, context)?;

        let tcon_entries: zoc::eval::Normalized<Vec<znode::Expr>> = self
            .convert_june_context_to_zo_tcon_excluding_nondeb_entries(context)
            .into_iter()
            .collect();

        if let Err(zo_err) = self.zo_typechecker.get_type(
            converted.clone(),
            zoc::typecheck::LazyTypeContext::Base(tcon_entries.to_derefed()),
        ) {
            return Err(SemanticError::ConvertedExprHasZoErr(
                expr.clone(),
                converted.clone(),
                zo_err,
            ));
        }

        Ok(converted)
    }

    fn convert_june_context_to_zo_tcon_excluding_nondeb_entries(
        &mut self,
        context: Context,
    ) -> Vec<NormalForm> {
        match context {
            Context::Base(entries) => self.get_normalized_values_of_nondeb_entries(entries),

            Context::Snoc(left, right) => {
                let mut v = self.convert_june_context_to_zo_tcon_excluding_nondeb_entries(*left);
                let extension = self.get_normalized_values_of_nondeb_entries(right);
                v.extend(extension);
                v
            }
        }
    }

    fn get_normalized_values_of_nondeb_entries(
        &mut self,
        entries: &[UnshiftedEntry<'_>],
    ) -> Vec<NormalForm> {
        entries
            .iter()
            .filter_map(|entry| -> Option<NormalForm> {
                if !entry.is_deb {
                    return None;
                }

                let normalized = self.zo_typechecker.evaluator.eval(entry.val.clone());
                Some(normalized)
            })
            .collect()
    }
}
