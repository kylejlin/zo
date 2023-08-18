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

    /// `AbsenceChecker`'s methods assert that every recursive ind entry
    /// in the context does **not** appear (i.e., is absent)
    /// from the given expression.
    #[derive(Debug)]
    pub struct AbsenceChecker<'a>(pub PositivityChecker<'a>);
}
use namespace_structs::*;

// TODO: We can design Context to use a `usize` representing
// repetitions of `IsRestrictedRecursiveIndEntry(false)`,
// since that will be the bulk of entries.

#[derive(Clone, Copy, Debug)]
enum Context<'a> {
    Base(&'a [IsRestrictedRecursiveIndEntry]),
    Snoc(&'a Context<'a>, &'a [IsRestrictedRecursiveIndEntry]),
}

#[derive(Clone, Copy, Debug)]
struct IsRestrictedRecursiveIndEntry(pub bool);

impl PositivityChecker<'_> {
    pub fn check_ind_positivity_assuming_it_is_otherwise_well_typed(
        &mut self,
        ind: RcHashed<cst::Ind>,
        tcon_len: usize,
    ) -> Result<(), TypeError> {
        let base = vec![IsRestrictedRecursiveIndEntry(false); tcon_len];
        self.check_ind(&ind.hashee, Context::Base(&base))
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
            cst::Expr::Deb(_) | cst::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_ind(&mut self, ind: &cst::Ind, context: Context) -> Result<(), TypeError> {
        self.check_dependent_exprs(&ind.index_types, context)?;

        let singleton = [IsRestrictedRecursiveIndEntry(true)];
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

        let extension = vec![IsRestrictedRecursiveIndEntry(false); def.param_types.len()];
        let extended_context = Context::Snoc(&context, &extension);
        self.check_independent_exprs(&def.index_args, extended_context)?;

        VconPositivityChecker(self.clone_mut())
            .assert_vcon_type_satisfies_positivity_condition(def, context)?;

        Ok(())
    }

    fn check_vcon(&mut self, vcon: &cst::Vcon, context: Context) -> Result<(), TypeError> {
        self.check_ind(&vcon.ind.hashee, context)
    }

    fn check_match(&mut self, match_: &cst::Match, context: Context) -> Result<(), TypeError> {
        self.check(match_.matchee.clone(), context)?;

        let return_type_extension =
            vec![IsRestrictedRecursiveIndEntry(false); match_.return_type_arity.value];
        let return_type_context = Context::Snoc(&context, &return_type_extension);
        self.check(match_.return_type.clone(), return_type_context)?;

        self.check_match_cases(&match_.cases, context)?;

        Ok(())
    }

    fn check_match_cases(
        &mut self,
        cases: &[cst::MatchCase],
        context: Context,
    ) -> Result<(), TypeError> {
        for case in cases {
            self.check_match_case(case, context)?;
        }
        Ok(())
    }

    fn check_match_case(
        &mut self,
        case: &cst::MatchCase,
        context: Context,
    ) -> Result<(), TypeError> {
        let return_val_extension = vec![IsRestrictedRecursiveIndEntry(false); case.arity.value];
        let return_val_context = Context::Snoc(&context, &return_val_extension);
        self.check(case.return_val.clone(), return_val_context)?;

        Ok(())
    }

    fn check_fun(&mut self, fun: &cst::Fun, context: Context) -> Result<(), TypeError> {
        self.check_dependent_exprs(&fun.param_types, context)?;

        let return_type_extension =
            vec![IsRestrictedRecursiveIndEntry(false); fun.param_types.len()];
        let context_with_params = Context::Snoc(&context, &return_type_extension);
        self.check(fun.return_type.clone(), context_with_params)?;

        let singleton = [IsRestrictedRecursiveIndEntry(false)];
        let context_with_params_and_recursive_fun = Context::Snoc(&context_with_params, &singleton);
        self.check(
            fun.return_val.clone(),
            context_with_params_and_recursive_fun,
        )?;

        Ok(())
    }

    fn check_app(&mut self, app: &cst::App, context: Context) -> Result<(), TypeError> {
        self.check(app.callee.clone(), context)?;
        self.check_independent_exprs(&app.args, context)?;
        Ok(())
    }

    fn check_for(&mut self, for_: &cst::For, context: Context) -> Result<(), TypeError> {
        self.check_dependent_exprs(&for_.param_types, context)?;

        let extension = vec![IsRestrictedRecursiveIndEntry(false); for_.param_types.len()];
        let extended_context = Context::Snoc(&context, &extension);
        self.check(for_.return_type.clone(), extended_context)?;

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

        let extension = vec![IsRestrictedRecursiveIndEntry(false); exprs.len() - 1];

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

            let extension =
                vec![IsRestrictedRecursiveIndEntry(false); param_count.saturating_sub(1)];

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

        let extension = vec![IsRestrictedRecursiveIndEntry(false); def.param_types.len()];
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
            ast::Expr::Ind(e) => self.check_ind(&e.hashee, context, path),

            ast::Expr::Deb(_) => Ok(()),

            ast::Expr::App(e) => self.check_app(&e.hashee, context, path),

            ast::Expr::For(e) => self.check_for(&e.hashee, context, path),

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
        let path_to_callee = NodePath::Snoc(&path, node_path::APP_CALLEE);
        self.check_app_callee(app.callee.clone(), context, path_to_callee)?;

        let path_to_args = NodePath::Snoc(&path, node_path::APP_ARGS);
        let mut absent = AbsenceChecker(self.0.clone_mut());
        absent.check_independent_exprs(&app.args.hashee, context, path_to_args)?;

        Ok(())
    }

    fn check_app_callee(
        &mut self,
        callee: ast::Expr,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match callee {
            ast::Expr::Ind(e) => self.check_ind(&e.hashee, context, path),

            ast::Expr::Deb(_) => Ok(()),

            ast::Expr::Vcon(_)
            | ast::Expr::Match(_)
            | ast::Expr::Fun(_)
            | ast::Expr::App(_)
            | ast::Expr::For(_)
            | ast::Expr::Universe(_) => {
                let mut absent = AbsenceChecker(self.0.clone_mut());
                absent.check(callee, context, path)
            }
        }
    }

    fn check_ind(
        &mut self,
        ind: &ast::Ind,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_index_types = NodePath::Snoc(&path, node_path::IND_INDEX_TYPES);
        let mut absence = AbsenceChecker(self.0.clone_mut());
        absence.check_dependent_exprs(&ind.index_types.hashee, context, path_to_index_types)?;

        let extension = [IsRestrictedRecursiveIndEntry(true)];
        let extended_context = Context::Snoc(&context, &extension);
        let path_to_vcon_defs = NodePath::Snoc(&path, node_path::IND_VCON_DEFS);
        self.check_vcon_defs(&ind.vcon_defs.hashee, extended_context, path_to_vcon_defs)?;

        Ok(())
    }

    fn check_vcon_defs(
        &mut self,
        defs: &[ast::VconDef],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        for (i, def) in defs.iter().cloned().enumerate() {
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check_vcon_def(def, context, extended_path)?;
        }
        Ok(())
    }

    fn check_vcon_def(
        &mut self,
        def: ast::VconDef,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::VCON_DEF_PARAM_TYPES);
        self.check_dependent_exprs(&def.param_types.hashee, context, path_to_param_types)?;

        let extension = vec![IsRestrictedRecursiveIndEntry(false); def.param_types.hashee.len()];
        let extended_context = Context::Snoc(&context, &extension);
        let path_to_index_args = NodePath::Snoc(&path, node_path::VCON_DEF_INDEX_ARGS);
        let mut absence = AbsenceChecker(self.0.clone_mut());
        absence.check_independent_exprs(
            &def.index_args.hashee,
            extended_context,
            path_to_index_args,
        )?;

        Ok(())
    }

    fn check_for(
        &mut self,
        for_: &ast::For,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let mut absent = AbsenceChecker(self.0.clone_mut());
        let path_to_param_types = NodePath::Snoc(&path, node_path::FOR_PARAM_TYPES);
        absent.check_dependent_exprs(&for_.param_types.hashee, context, path_to_param_types)?;

        let extension = vec![IsRestrictedRecursiveIndEntry(false); for_.param_types.hashee.len()];
        let extended_context = Context::Snoc(&context, &extension);
        let path_to_return_type = NodePath::Snoc(&path, node_path::FOR_RETURN_TYPE);
        self.check(
            for_.return_type.clone(),
            extended_context,
            path_to_return_type,
        )?;

        Ok(())
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

        let extension = vec![IsRestrictedRecursiveIndEntry(false); exprs.len() - 1];

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_context = Context::Snoc(&context, &extension[..i]);
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(expr, extended_context, extended_path)?;
        }

        Ok(())
    }
}

// TODO: For `AbsenceChecker`, we can replace `context: Context`
// with `context: ContextExtendedWithUnrestrictedEntries = (usize, Context)`.
// This is because we never add `IsRestrictedRecursiveIndEntry(true)` to the context
// within the absence-checking process.

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
        let path_to_index_types = NodePath::Snoc(&path, node_path::IND_INDEX_TYPES);
        self.check_dependent_exprs(&ind.index_types.hashee, context, path_to_index_types)?;

        let singleton = [IsRestrictedRecursiveIndEntry(false)];
        let extended_context = Context::Snoc(&context, &singleton);
        let path_to_vcon_defs = NodePath::Snoc(&path, node_path::IND_VCON_DEFS);
        self.check_vcon_defs(&ind.vcon_defs.hashee, extended_context, path_to_vcon_defs)?;

        Ok(())
    }

    fn check_vcon_defs(
        &mut self,
        defs: &[ast::VconDef],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        for (i, def) in defs.iter().cloned().enumerate() {
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check_vcon_def(def, context, extended_path)?;
        }

        Ok(())
    }

    fn check_vcon_def(
        &mut self,
        def: ast::VconDef,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::VCON_DEF_PARAM_TYPES);
        self.check_dependent_exprs(&def.param_types.hashee, context, path_to_param_types)?;

        let extension = vec![IsRestrictedRecursiveIndEntry(false); def.param_types.hashee.len()];
        let extended_context = Context::Snoc(&context, &extension);
        let path_to_index_args = NodePath::Snoc(&path, node_path::VCON_DEF_INDEX_ARGS);
        self.check_independent_exprs(&def.index_args.hashee, extended_context, path_to_index_args)?;

        Ok(())
    }

    fn check_vcon(
        &mut self,
        vcon: &ast::Vcon,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_ind = NodePath::Snoc(&path, node_path::VCON_IND);
        self.check_ind(&vcon.ind.hashee, context, path_to_ind)
    }

    fn check_match(
        &mut self,
        match_: &ast::Match,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_matchee = NodePath::Snoc(&path, node_path::MATCH_MATCHEE);
        self.check(match_.matchee.clone(), context, path_to_matchee)?;

        let return_type_extension =
            vec![IsRestrictedRecursiveIndEntry(false); match_.return_type_arity];
        let context_with_return_type_extension = Context::Snoc(&context, &return_type_extension);
        let path_to_return_type = NodePath::Snoc(&path, node_path::MATCH_RETURN_TYPE);
        self.check(
            match_.return_type.clone(),
            context_with_return_type_extension,
            path_to_return_type,
        )?;

        let path_to_cases = NodePath::Snoc(&path, node_path::MATCH_CASES);
        self.check_match_cases(&match_.cases.hashee, context, path_to_cases)?;

        Ok(())
    }

    fn check_match_cases(
        &mut self,
        cases: &[ast::MatchCase],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        for (i, case) in cases.iter().enumerate() {
            let extension = vec![IsRestrictedRecursiveIndEntry(false); case.arity];
            let extended_context = Context::Snoc(&context, &extension);
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(case.return_val.clone(), extended_context, extended_path)?;
        }

        Ok(())
    }

    fn check_fun(
        &mut self,
        fun: &ast::Fun,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::FUN_PARAM_TYPES);
        self.check_dependent_exprs(&fun.param_types.hashee, context, path_to_param_types)?;

        let param_extension =
            vec![IsRestrictedRecursiveIndEntry(false); fun.param_types.hashee.len()];
        let context_with_params = Context::Snoc(&context, &param_extension);
        let path_to_return_type = NodePath::Snoc(&path, node_path::FUN_RETURN_TYPE);
        self.check(
            fun.return_type.clone(),
            context_with_params,
            path_to_return_type,
        )?;

        let singleton = [IsRestrictedRecursiveIndEntry(false)];
        let context_with_params_and_recursive_fun = Context::Snoc(&context_with_params, &singleton);
        let path_to_return_val = NodePath::Snoc(&path, node_path::FUN_RETURN_VAL);
        self.check(
            fun.return_val.clone(),
            context_with_params_and_recursive_fun,
            path_to_return_val,
        )?;

        Ok(())
    }

    fn check_app(
        &mut self,
        app: &ast::App,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_callee = NodePath::Snoc(&path, node_path::APP_CALLEE);
        self.check(app.callee.clone(), context, path_to_callee)?;

        let path_to_args = NodePath::Snoc(&path, node_path::APP_ARGS);
        self.check_independent_exprs(&app.args.hashee, context, path_to_args)?;

        Ok(())
    }

    fn check_for(
        &mut self,
        for_: &ast::For,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::FOR_PARAM_TYPES);
        self.check_dependent_exprs(&for_.param_types.hashee, context, path_to_param_types)?;

        let extension = vec![IsRestrictedRecursiveIndEntry(false); for_.param_types.hashee.len()];
        let extended_context = Context::Snoc(&context, &extension);
        let path_to_return_type = NodePath::Snoc(&path, node_path::FOR_RETURN_TYPE);
        self.check(
            for_.return_type.clone(),
            extended_context,
            path_to_return_type,
        )?;

        Ok(())
    }

    fn check_deb(
        &mut self,
        deb: &ast::DebNode,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        if context.get(deb.deb).expect("if an expression is well-typed apart from positivity, its debs should all be valid").0 {
            return Err(path.to_vec());
        }

        Ok(())
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

        let extension = vec![IsRestrictedRecursiveIndEntry(false); exprs.len() - 1];

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
        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(expr, context, extended_path)?;
        }

        Ok(())
    }
}

impl Context<'_> {
    pub fn get(&self, deb: Deb) -> Option<IsRestrictedRecursiveIndEntry> {
        match self {
            Context::Base(entries) => {
                let index = (entries.len()).checked_sub(1 + deb.0)?;
                entries.get(index).copied()
            }

            Context::Snoc(subcontext, entries) => {
                if let Some(index) = (entries.len()).checked_sub(1 + deb.0) {
                    entries.get(index).copied()
                } else {
                    subcontext.get(Deb(deb.0 - entries.len()))
                }
            }
        }
    }
}
