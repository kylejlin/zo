use super::*;

impl JuneConverter {
    pub(crate) fn convert_universe(
        &mut self,
        expr: &jnode::UniverseLiteral,
        _: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let converted_leaf = self.cache_universe(znode::UniverseNode {
            universe: Universe {
                level: UniverseLevel(expr.level),
                erasable: expr.erasable,
            },
            aux_data: (),
        });
        Ok(converted_leaf)
    }
}
