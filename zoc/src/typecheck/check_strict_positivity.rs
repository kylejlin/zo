//! All functions in this module assume
//! that their inputs are well-typed,
//! apart from the positivity condition.
//! If you pass in a term that is ill-typed
//! (for reasons other than failing the positivity condition),
//! it may loop forever or panic.

use super::*;

#[derive(Debug)]
pub struct PositivityChecker<'a> {
    pub typechecker: &'a mut TypeChecker,
}

#[derive(Clone, Copy, Debug)]
enum Context<'a> {
    Base(&'a [IsInd]),
    Snoc(&'a Context<'a>, &'a [IsInd]),
}

#[derive(Clone, Copy, Debug)]
struct IsInd(pub bool);

impl Context<'static> {
    pub fn empty() -> Self {
        Self::Base(&[])
    }
}

impl PositivityChecker<'_> {
    pub fn check_ind_positivity_assuming_it_is_otherwise_well_typed(
        &mut self,
        ind: RcHashed<cst::Ind>,
    ) -> Result<(), TypeError> {
        self.check_ind(&ind.hashee, Context::empty())
    }

    fn check_ind(&mut self, ind: &cst::Ind, context: Context) -> Result<(), TypeError> {
        let singleton = [IsInd(true)];
        let extended_context = Context::Snoc(&context, &singleton);
        self.check_vcon_defs(&ind.vcon_defs, extended_context)
    }

    fn check_vcon_defs(
        &mut self,
        defs: &[cst::VconDef],
        context: Context,
    ) -> Result<(), TypeError> {
        for def in defs {
            self.check_vcon_def(def, context)?;
        }
        Ok(())
    }

    fn check_vcon_def(&mut self, def: &cst::VconDef, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }
}
