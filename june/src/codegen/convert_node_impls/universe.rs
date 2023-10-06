use super::*;

impl MayConverter {
    pub(crate) fn convert_universe<C: ContextToOwned>(
        &mut self,
        expr: &mnode::UniverseLiteral,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let converted_leaf = self.cache_universe(znode::UniverseNode {
            universe: Universe {
                level: UniverseLevel(expr.level),
                erasable: expr.erasable,
            },
            aux_data: (),
        });
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }
}
