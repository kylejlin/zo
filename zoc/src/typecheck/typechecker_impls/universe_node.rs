use super::*;

impl TypeChecker {
    pub fn get_type_of_universe(
        &mut self,
        universe: RcHashed<UniverseLiteral>,
    ) -> Result<NormalForm, TypeError> {
        return Ok(self.evaluator.eval(ast::Expr::Universe(Rc::new(Hashed::new(
            ast::UniverseNode {
                level: UniverseLevel(universe.hashee.level + 1),
            },
        )))));
    }
}
