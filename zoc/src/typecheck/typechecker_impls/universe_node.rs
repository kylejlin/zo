use super::*;

impl TypeChecker {
    pub fn get_type_of_universe(
        &mut self,
        universe_node: RcHashed<spanned_ast::UniverseNode>,
    ) -> Result<NormalForm, TypeError> {
        return Ok(self
            .evaluator
            .eval(minimal_ast::Expr::Universe(Rc::new(Hashed::new(
                minimal_ast::UniverseNode {
                    universe: Universe {
                        level: UniverseLevel(universe_node.hashee.universe.level.0 + 1),
                        erasable: true,
                    },
                    aux_data: (),
                },
            )))));
    }
}
