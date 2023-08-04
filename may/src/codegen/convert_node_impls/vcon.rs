use super::*;

impl MayConverter {
    pub(crate) fn convert_vcon(
        &mut self,
        expr: &mnode::Vcon,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.innards.params {
            mnode::OptParenthesizedParamDefs::None => {
                self.convert_unparameterized_vcon(expr, context)
            }

            mnode::OptParenthesizedParamDefs::Some(parenthesized) => {
                self.convert_parameterized_vcon(expr, &parenthesized.params, context)
            }
        }
    }

    fn convert_unparameterized_vcon(
        &mut self,
        expr: &mnode::Vcon,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_unparameterized_ind_innards_to_zo_ind(&expr.innards, context)?;
        let ind = self
            .cache_ind(ind)
            .try_into_ind()
            .expect("cache_ind should always return the same ind");

        let vcon_index = expr.vcon_index.index;

        Ok(self.cache_vcon(znode::Vcon { ind, vcon_index }))
    }

    fn convert_parameterized_vcon(
        &mut self,
        expr: &mnode::Vcon,
        params: &mnode::CommaSeparatedParamDefs,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }
}
