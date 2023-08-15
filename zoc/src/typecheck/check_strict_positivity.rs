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
enum PositivityContext<'a> {
    Base(&'a [bool]),
    Snoc(&'a PositivityContext<'a>, &'a [bool]),
}

impl PositivityContext<'static> {
    pub fn empty() -> Self {
        Self::Base(&[])
    }
}

impl PositivityChecker<'_> {
    pub fn check_ind_positivity_assuming_it_is_otherwise_well_typed(
        &mut self,
        ind: RcHashed<cst::Ind>,
    ) -> Result<(), TypeError> {
        self.check_ind(&ind.hashee, PositivityContext::empty())
    }

    fn check_ind(&mut self, ind: &cst::Ind, pcon: PositivityContext) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }
}
