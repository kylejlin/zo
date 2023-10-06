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
    pub(crate) fn convert<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Expr,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        match expr {
            mnode::Expr::Let(e) => self.convert_let(e, context, converter),
            mnode::Expr::Ind(e) => self.convert_ind(e, context, converter),
            mnode::Expr::Fun(e) => self.convert_fun(e, context, converter),
            mnode::Expr::Aind(e) => self.convert_aind(e, context, converter),
            mnode::Expr::Vcon(e) => self.convert_vcon(e, context, converter),
            mnode::Expr::Match(e) => self.convert_match(e, context, converter),
            mnode::Expr::Afun(e) => self.convert_afun(e, context, converter),
            mnode::Expr::For(e) => self.convert_for(e, context, converter),
            mnode::Expr::VarOrApp(e) => self.convert_var_or_app(e, context, converter),
            mnode::Expr::Universe(e) => self.convert_universe(e, context, converter),
        }
    }
}

impl MayConverter {
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
                let (e, _) = self.convert(e, context, &DropContext)?;
                Ok(vec![e])
            }
            mnode::CommaSeparatedExprs::Snoc(rdc, rac) => {
                let mut rdc = self.convert_exprs_without_hashing_vec(rdc, context)?;
                let (rac, _) = self.convert(rac, context, &DropContext)?;
                rdc.push(rac);
                Ok(rdc)
            }
        }
    }
}
