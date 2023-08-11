use super::*;

#[derive(Clone, Copy)]
pub enum RecursionCheckingContext<'a> {
    Base(&'a [UnshiftedEntry<'a>]),
    Snoc(&'a RecursionCheckingContext<'a>, &'a [UnshiftedEntry<'a>]),
}

impl RecursionCheckingContext<'static> {
    pub fn empty() -> Self {
        RecursionCheckingContext::Base(&[])
    }
}

impl RecursionCheckingContext<'_> {
    fn get_call_requirement(&self, deb: Deb) -> Option<CallRequirement> {
        todo!()
    }
}

#[derive(Clone)]
pub struct UnshiftedEntry<'a>(pub Entry<'a>);

impl UnshiftedEntry<'static> {
    fn irrelevant() -> Self {
        Self(Entry::Irrelevant)
    }
}

#[derive(Clone)]
pub enum Entry<'a> {
    Irrelevant,
    RecursiveFun {
        arg_index: usize,
        definition_src: &'a cst::Fun,
    },
    DecreasingParam,
    DecreasingParamSubstruct,
}

#[derive(Clone, Copy)]
struct CallRequirement<'a> {
    arg_index: usize,
    strict_superstruct: Deb,
    definition_src: &'a cst::Fun,
}

impl TypeChecker {
    pub(crate) fn check_recursion(
        &mut self,
        expr: cst::Expr,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        match expr {
            cst::Expr::Ind(e) => self.check_recursion_in_ind(e, rcon),
            cst::Expr::Vcon(e) => self.check_recursion_in_vcon(e, rcon),
            cst::Expr::Match(e) => self.check_recursion_in_match(e, rcon),
            cst::Expr::Fun(e) => self.check_recursion_in_fun(e, rcon),
            cst::Expr::App(e) => self.check_recursion_in_app(e, rcon),
            cst::Expr::For(e) => self.check_recursion_in_for(e, rcon),
            cst::Expr::Deb(e) => self.check_recursion_in_deb(e, rcon),
            cst::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_recursion_in_ind(
        &mut self,
        ind: RcHashed<cst::Ind>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&ind.hashee.index_types, rcon)?;

        let singleton = vec![UnshiftedEntry::irrelevant()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &singleton);
        self.check_recursion_in_vcon_defs(&ind.hashee.vcon_defs, extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_vcon_defs(
        &mut self,
        defs: &[cst::VconDef],
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for def in defs {
            self.check_recursion_in_vcon_def(def, rcon)?;
        }
        Ok(())
    }

    fn check_recursion_in_vcon_def(
        &mut self,
        def: &cst::VconDef,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&def.param_types, rcon)?;

        let extension = vec![UnshiftedEntry::irrelevant(); def.param_types.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion_in_independent_exprs(&def.index_args, extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_vcon(
        &mut self,
        vcon: RcHashed<cst::Vcon>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_ind(vcon.hashee.ind.clone(), rcon)
    }

    fn check_recursion_in_match(
        &mut self,
        m: RcHashed<cst::Match>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        todo!()
    }

    fn check_recursion_in_fun(
        &mut self,
        fun: RcHashed<cst::Fun>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        todo!()
    }

    fn check_recursion_in_app(
        &mut self,
        app: RcHashed<cst::App>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        if let cst::Expr::Deb(callee) = &app.hashee.callee {
            let callee_deb = Deb(callee.hashee.value);
            if let Some(requirement) = rcon.get_call_requirement(callee_deb) {
                self.assert_arg_satisfies_requirement(app.clone(), requirement, rcon)?;

                // We don't check recursion in the callee because
                // it would trigger a false positive.
                // Since the callee is a deb, and we already checked
                // the call requirement, we can safely skip this check.
                self.check_recursion_in_independent_exprs(&app.hashee.args, rcon)?;
                return Ok(());
            }
        }

        self.check_recursion(app.hashee.callee.clone(), rcon)?;
        self.check_recursion_in_independent_exprs(&app.hashee.args, rcon)?;
        Ok(())
    }

    fn assert_arg_satisfies_requirement(
        &mut self,
        app: RcHashed<cst::App>,
        requirement: CallRequirement,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        if requirement.arg_index >= app.hashee.args.len() {
            // Do nothing.
            //
            // The user-provided decreasing index is either invalid
            // or the number of arguments is illegal.
            // In either case, this is a type error that will be
            // caught elsewhere in the typechecking process.
            // We don't want to return an error _here_ because
            // that would complicated this code.
            // Such complication is not necessary because
            // the other typechecking code will catch the error.
            return Ok(());
        }

        let arg = &app.hashee.args[requirement.arg_index];
        if !self.is_strict_substruct(arg, requirement.strict_superstruct, rcon) {
            return Err(TypeError::IllegalRecursiveCall {
                app: app.hashee.clone(),
                callee_deb_definition_src: requirement.definition_src.clone(),
                required_decreasing_arg_index: requirement.arg_index,
                required_strict_superstruct: requirement.strict_superstruct,
            });
        }

        Ok(())
    }

    fn check_recursion_in_for(
        &mut self,
        for_: RcHashed<cst::For>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        self.check_recursion_in_dependent_exprs(&for_.hashee.param_types, rcon)?;

        let extension = vec![UnshiftedEntry::irrelevant(); for_.hashee.param_types.len()];
        let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &extension);
        self.check_recursion(for_.hashee.return_type.clone(), extended_rcon)?;

        Ok(())
    }

    fn check_recursion_in_deb(
        &mut self,
        deb: RcHashed<cst::NumberLiteral>,
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        if let Some(requirement) = rcon.get_call_requirement(Deb(deb.hashee.value)) {
            return Err(TypeError::IllegalRecursiveReference {
                deb: deb.hashee.clone(),
                definition_src: requirement.definition_src.clone(),
            });
        }

        Ok(())
    }
}

impl TypeChecker {
    fn check_recursion_in_dependent_exprs(
        &mut self,
        exprs: &[cst::Expr],
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        let rcon_extension = vec![UnshiftedEntry::irrelevant(); exprs.len()];

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_rcon = RecursionCheckingContext::Snoc(&rcon, &rcon_extension[..i]);
            self.check_recursion(expr, extended_rcon)?;
        }

        Ok(())
    }

    fn check_recursion_in_independent_exprs(
        &mut self,
        exprs: &[cst::Expr],
        rcon: RecursionCheckingContext,
    ) -> Result<(), TypeError> {
        for expr in exprs {
            self.check_recursion(expr.clone(), rcon)?;
        }
        Ok(())
    }
}

impl TypeChecker {
    fn is_strict_substruct(
        &mut self,
        arg: &cst::Expr,
        strict_superstruct: Deb,
        rcon: RecursionCheckingContext,
    ) -> bool {
        todo!()
    }
}
