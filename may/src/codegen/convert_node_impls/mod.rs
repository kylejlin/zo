use super::*;

mod fun;
mod ind;
mod let_;

mod afun;
mod aind;
mod for_;
mod match_;
mod universe;
mod var_or_app;
mod vcon;

impl MayConverter {
    pub(crate) fn convert(
        &mut self,
        expr: &mnode::Expr,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::Expr::Let(e) => self.convert_let(e, context),
            mnode::Expr::Ind(e) => self.convert_ind(e, context),
            mnode::Expr::Fun(e) => self.convert_fun(e, context),
            mnode::Expr::Aind(e) => self.convert_aind(e, context),
            mnode::Expr::Vcon(e) => self.convert_vcon(e, context),
            mnode::Expr::Match(e) => self.convert_match(e, context),
            mnode::Expr::Afun(e) => self.convert_afun(e, context),
            mnode::Expr::For(e) => self.convert_for(e, context),
            mnode::Expr::VarOrApp(e) => self.convert_var_or_app(e, context),
            mnode::Expr::Universe(e) => self.convert_universe(e),
        }
    }
}

impl MayConverter {
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
