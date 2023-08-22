use super::*;

impl TypeChecker {
    pub fn get_type_of_deb<A: AuxDataFamily>(
        &mut self,
        deb_node: RcHashed<ast::DebNode<A>>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        if let Some(expr) = tcon.get(deb_node.hashee.deb) {
            return Ok(expr);
        }

        return Err(TypeError::InvalidDeb {
            deb: deb_node.hashee.clone(),
            tcon_len: tcon.len(),
        });
    }
}
