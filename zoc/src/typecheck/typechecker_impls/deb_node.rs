use super::*;

impl TypeChecker {
    pub fn get_type_of_deb(
        &mut self,
        deb: RcHashed<NumberLiteral>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        if let Some(expr) = tcon.get(Deb(deb.hashee.value)) {
            return Ok(expr);
        }

        return Err(TypeError::InvalidDeb {
            deb: deb.hashee.clone(),
            tcon_len: tcon.len(),
        });
    }
}
