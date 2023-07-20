use super::*;

impl TypeChecker {
    pub fn get_type_of_retype(
        &mut self,
        retype: RcHashed<cst::Retype>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        todo!()
    }
}
