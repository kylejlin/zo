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
    Base(&'a [IsRecursiveIndEntry]),
    Snoc(&'a Context<'a>, &'a [IsRecursiveIndEntry]),
}

#[derive(Clone, Copy, Debug)]
struct IsRecursiveIndEntry(pub bool);

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
}

impl PositivityChecker<'_> {
    fn check(&mut self, expr: cst::Expr, context: Context) -> Result<(), TypeError> {
        match expr {
            cst::Expr::Ind(e) => self.check_ind(&e.hashee, context),
            cst::Expr::Vcon(e) => self.check_vcon(&e.hashee, context),
            cst::Expr::Match(e) => self.check_match(&e.hashee, context),
            cst::Expr::Fun(e) => self.check_fun(&e.hashee, context),
            cst::Expr::App(e) => self.check_app(&e.hashee, context),
            cst::Expr::For(e) => self.check_for(&e.hashee, context),
            cst::Expr::Deb(e) => self.check_deb(&e.hashee, context),
            cst::Expr::Universe(e) => self.check_universe(&e.hashee, context),
        }
    }

    fn check_ind(&mut self, ind: &cst::Ind, context: Context) -> Result<(), TypeError> {
        self.check_dependent_exprs(&ind.index_types, context)?;

        let singleton = [IsRecursiveIndEntry(true)];
        let extended_context = Context::Snoc(&context, &singleton);
        self.check_vcon_defs(&ind.vcon_defs, extended_context)?;

        Ok(())
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
        self.check_dependent_exprs(&def.param_types, context)?;

        let extension = vec![IsRecursiveIndEntry(false); def.param_types.len()];
        let extended_context = Context::Snoc(&context, &extension);
        self.check_independent_exprs(&def.index_args, extended_context)?;

        // TODO: Get internal vcon type and check positivity.

        Ok(())
    }

    fn check_vcon(&mut self, vcon: &cst::Vcon, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }

    fn check_match(&mut self, m: &cst::Match, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }

    fn check_fun(&mut self, fun: &cst::Fun, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }

    fn check_app(&mut self, app: &cst::App, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }

    fn check_for(&mut self, for_: &cst::For, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }

    fn check_deb(&mut self, deb: &cst::NumberLiteral, context: Context) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }

    fn check_universe(
        &mut self,
        universe: &cst::UniverseLiteral,
        context: Context,
    ) -> Result<(), TypeError> {
        // TODO
        Ok(())
    }
}

impl PositivityChecker<'_> {
    fn check_dependent_exprs(
        &mut self,
        exprs: &[cst::Expr],
        context: Context,
    ) -> Result<(), TypeError> {
        if exprs.is_empty() {
            return Ok(());
        }

        let extension = vec![IsRecursiveIndEntry(false); exprs.len() - 1];

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_context = Context::Snoc(&context, &extension[..i]);
            self.check(expr, extended_context)?;
        }

        Ok(())
    }

    fn check_independent_exprs(
        &mut self,
        exprs: &[cst::Expr],
        context: Context,
    ) -> Result<(), TypeError> {
        for expr in exprs.iter().cloned() {
            self.check(expr, context)?;
        }
        Ok(())
    }
}
