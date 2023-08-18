//! All functions in this module assume
//! that their inputs are well-typed,
//! apart from the positivity condition.
//! If you pass in a term that is ill-typed
//! (for reasons other than failing the positivity condition),
//! it may loop forever or panic.
//!
//! Zo's positivity rules are based on those of Coq.
//! You can learn more from
//! The Coq Proof Assistant Reference Manual
//! (specifically, Version 8.4pl2, published 2013 April 4).
//! A copy can be found at
//! https://flint.cs.yale.edu/cs430/coq/pdf/Reference-Manual.pdf
//! Pages 122 and 123 are relevant.

use std::path::Path;

use super::*;

use crate::syntax_tree::ast::node_path::{self, NodeEdge, NodePath};

#[derive(Debug)]
pub struct PositivityChecker<'a> {
    pub typechecker: &'a mut TypeChecker,
}

/// We need to create the methods
/// - `check_strict_positivity`, `check_strict_positivity_in_ind`,
///   `check_strict_positivity_in_vcon`, etc.
/// - `check_nested_positivity`, `check_nested_positivity_in_ind`,
///   `check_nested_positivity_in_vcon`, etc.
/// - `check_absence`, `check_absence_in_ind`, `check_absence_in_vcon`, etc.
///
/// These method names are very verbose.
/// It would be much more readable if we simply name them
/// `check`, `check_ind`, `check_vcon`, etc.
/// However, Rust forbids use from defining multiple `check` methods
/// on `PositivityChecker` (for good reason--it would be ambiguous).
///
/// So, we create the "namespace structs" `StrictPositivityChecker`,
/// `NestedPositivityChecker`, `AbsenceChecker`, and `VconPositivityChecker`.
/// These structs are wrappers around `PositivityChecker`.
/// Their sole purpose is to let us organize our methods and use shorter names.
///
/// For example, instead of `PositivityChecker::check_strict_positivity_in_ind`,
/// we have `StrictPositivityChecker::check_ind`.
mod namespace_structs {
    use super::*;

    #[derive(Debug)]
    pub struct VconPositivityChecker<'a>(pub PositivityChecker<'a>);

    /// `StrictPositivityChecker`'s methods assert that every recursive ind entry
    /// in the context appears strictly positively in the given expression.
    #[derive(Debug)]
    pub struct StrictPositivityChecker<'a>(pub PositivityChecker<'a>);

    // TODO: Add explanation comment.
    #[derive(Debug)]
    pub struct NestedPositivityChecker<'a>(pub PositivityChecker<'a>);

    /// `AbsenceChecker`'s methods assert that every recursive ind entry
    /// in the context does **not** appear (i.e., is absent)
    /// from the given expression.
    #[derive(Debug)]
    pub struct AbsenceChecker<'a>(pub PositivityChecker<'a>);
}
use namespace_structs::*;

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

        VconPositivityChecker(self.clone_mut())
            .assert_vcon_type_satisfies_positivity_condition(def, context)?;

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

impl PositivityChecker<'_> {
    fn clone_mut<'a>(&'a mut self) -> PositivityChecker<'a> {
        PositivityChecker {
            typechecker: &mut self.typechecker,
        }
    }
}

impl VconPositivityChecker<'_> {
    fn assert_vcon_type_satisfies_positivity_condition(
        &mut self,
        def: &cst::VconDef,
        context: Context,
    ) -> Result<(), TypeError> {
        {
            let param_count = def.param_types.len();
            let param_types_ast = self
                .0
                .typechecker
                .cst_converter
                .convert_expressions(&def.param_types);
            let normalized_param_types = self
                .0
                .typechecker
                .evaluator
                .eval_expressions(param_types_ast);

            let extension = vec![IsRecursiveIndEntry(false); param_count.saturating_sub(1)];

            let mut strict = StrictPositivityChecker(self.0.clone_mut());

            for (i, param_type) in normalized_param_types
                .raw()
                .hashee
                .iter()
                .cloned()
                .enumerate()
            {
                let extended_context = Context::Snoc(&context, &extension[..i]);
                strict
                    .check(param_type, extended_context, NodePath::Nil)
                    .map_err(|path_from_param_type_to_problematic_deb| {
                        TypeError::VconDefParamTypeFailsStrictPositivityCondition {
                            def: def.clone(),
                            param_type_index: i,
                            normalized_param_type: normalized_param_types
                                .to_hashee()
                                .index(i)
                                .cloned(),
                            path_from_param_type_to_problematic_deb,
                        }
                    })?;
            }
        }

        let extension = vec![IsRecursiveIndEntry(false); def.param_types.len()];
        let extended_context = Context::Snoc(&context, &extension);

        {
            let index_args_ast = self
                .0
                .typechecker
                .cst_converter
                .convert_expressions(&def.index_args);
            let normalized_index_args = self
                .0
                .typechecker
                .evaluator
                .eval_expressions(index_args_ast);

            let mut absence = AbsenceChecker(self.0.clone_mut());

            for (i, index_arg) in normalized_index_args
                .raw()
                .hashee
                .iter()
                .cloned()
                .enumerate()
            {
                absence
                    .check(index_arg, extended_context, NodePath::Nil)
                    .map_err(|path_from_index_arg_to_problematic_deb| {
                        TypeError::RecursiveIndParamAppearsInVconDefIndexArg {
                            def: def.clone(),
                            index_arg_index: i,
                            normalized_index_arg: normalized_index_args
                                .to_hashee()
                                .index(i)
                                .cloned(),
                            path_from_index_arg_to_problematic_deb,
                        }
                    })?;
            }
        }

        Ok(())
    }
}

impl StrictPositivityChecker<'_> {
    fn check(
        &mut self,
        expr: ast::Expr,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match expr {
            // TODO
            ast::Expr::Ind(e) => Ok(()),

            ast::Expr::Deb(_) => Ok(()),

            // TODO
            ast::Expr::App(e) => self.check_app(&e.hashee, context, path),

            // TODO
            ast::Expr::For(e) => Ok(()),

            ast::Expr::Vcon(_)
            | ast::Expr::Match(_)
            | ast::Expr::Fun(_)
            | ast::Expr::Universe(_) => {
                let mut absent = AbsenceChecker(self.0.clone_mut());
                absent.check(expr, context, path)
            }
        }
    }

    fn check_app(
        &mut self,
        app: &ast::App,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        self.check_app_callee(app, context, path)?;

        let path_to_args = NodePath::Snoc(&path, node_path::APP_ARGS);
        let mut absent = AbsenceChecker(self.0.clone_mut());
        absent.check_independent_exprs(&app.args.hashee, context, path_to_args)?;

        Ok(())
    }

    fn check_app_callee(
        &mut self,
        app: &ast::App,
        context: Context,
        path_to_app: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match &app.callee {
            // TODO
            ast::Expr::Ind(e) => Ok(()),

            ast::Expr::Deb(_) => Ok(()),

            ast::Expr::Vcon(_)
            | ast::Expr::Match(_)
            | ast::Expr::Fun(_)
            | ast::Expr::App(_)
            | ast::Expr::For(_)
            | ast::Expr::Universe(_) => {
                let mut absent = AbsenceChecker(self.0.clone_mut());
                let path_to_callee = NodePath::Snoc(&path_to_app, node_path::APP_CALLEE);
                absent.check(app.callee.clone(), context, path_to_callee)
            }
        }
    }

    fn check_dependent_exprs(
        &mut self,
        exprs: &[ast::Expr],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        if exprs.is_empty() {
            return Ok(());
        }

        let extension = vec![IsRecursiveIndEntry(false); exprs.len() - 1];

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_context = Context::Snoc(&context, &extension[..i]);
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(expr, extended_context, extended_path)?;
        }

        Ok(())
    }

    fn check_independent_exprs(
        &mut self,
        exprs: &[ast::Expr],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        if exprs.is_empty() {
            return Ok(());
        }

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(expr, context, extended_path)?;
        }

        Ok(())
    }
}

impl AbsenceChecker<'_> {
    fn check(
        &mut self,
        expr: ast::Expr,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match expr {
            ast::Expr::Ind(e) => self.check_ind(&e.hashee, context, path),
            ast::Expr::Vcon(e) => self.check_vcon(&e.hashee, context, path),
            ast::Expr::Match(e) => self.check_match(&e.hashee, context, path),
            ast::Expr::Fun(e) => self.check_fun(&e.hashee, context, path),
            ast::Expr::App(e) => self.check_app(&e.hashee, context, path),
            ast::Expr::For(e) => self.check_for(&e.hashee, context, path),
            ast::Expr::Deb(e) => self.check_deb(&e.hashee, context, path),
            ast::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_ind(
        &mut self,
        ind: &ast::Ind,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_vcon(
        &mut self,
        vcon: &ast::Vcon,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_match(
        &mut self,
        m: &ast::Match,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_fun(
        &mut self,
        fun: &ast::Fun,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_app(
        &mut self,
        app: &ast::App,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_for(
        &mut self,
        for_: &ast::For,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_deb(
        &mut self,
        deb: &ast::DebNode,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }

    fn check_independent_exprs(
        &mut self,
        exprs: &[ast::Expr],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        // TODO
        Ok(())
    }
}
