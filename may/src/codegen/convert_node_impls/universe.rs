use super::*;

impl MayConverter {
    pub(crate) fn convert_universe(
        &mut self,
        expr: &mnode::UniverseLiteral,
    ) -> Result<znode::Expr, SemanticError> {
        Ok(self.cache_universe(znode::UniverseNode {
            universe: Universe {
                level: UniverseLevel(expr.level),
                erasable: expr.erasable,
            },
            aux_data: (),
        }))
    }
}
