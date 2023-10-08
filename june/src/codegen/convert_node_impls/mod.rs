use super::*;

mod def;
mod enum_;
mod var_def;

mod for_;
mod fun;
mod match_;
mod universe;
mod var_or_app;

impl JuneConverter {
    pub(crate) fn convert(
        &mut self,
        expr: &mnode::Expr,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::Expr::VarDef(e) => self.convert_chain_var_def(e, context),
            mnode::Expr::Enum(e) => self.convert_chain_enum(e, context),
            mnode::Expr::Def(e) => self.convert_chain_def(e, context),
            mnode::Expr::Match(e) => self.convert_match(e, context),
            mnode::Expr::Fun(e) => self.convert_fun(e, context),
            mnode::Expr::For(e) => self.convert_for(e, context),
            mnode::Expr::VarOrApp(e) => self.convert_var_or_app(e, context),
            mnode::Expr::Universe(e) => self.convert_universe(e, context),
        }
    }
}

impl JuneConverter {
    fn convert_optional_exprs(
        &mut self,
        exprs: Option<&mnode::CommaSeparatedExprs>,
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
        exprs: &mnode::CommaSeparatedExprs,
        context: Context,
    ) -> Result<RcHashedVec<znode::Expr>, SemanticError> {
        let v = self.convert_exprs_without_hashing_vec(exprs, context)?;
        Ok(self.cache_expr_vec(v))
    }

    fn convert_exprs_without_hashing_vec(
        &mut self,
        exprs: &mnode::CommaSeparatedExprs,
        context: Context,
    ) -> Result<Vec<znode::Expr>, SemanticError> {
        match exprs {
            mnode::CommaSeparatedExprs::One(e) => {
                let e = self.convert(e, context)?;
                Ok(vec![e])
            }
            mnode::CommaSeparatedExprs::Snoc(rdc, rac) => {
                let mut rdc = self.convert_exprs_without_hashing_vec(rdc, context)?;
                let rac = self.convert(rac, context)?;
                rdc.push(rac);
                Ok(rdc)
            }
        }
    }
}
