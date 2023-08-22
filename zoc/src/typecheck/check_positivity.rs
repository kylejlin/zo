//! All functions in this module assume
//! that their inputs are well-typed,
//! apart from the positivity condition.
//! If you pass in a term that is ill-typed
//! (for reasons other than failing the positivity condition),
//! it may loop forever or panic.
//!
//! Zo's positivity rules are based on those of Coq.
//!
//! You can learn more from
//! The Coq Proof Assistant Reference Manual
//! (specifically, Version 8.4pl2, published 2013 April 4).
//! This repository contains a copy in the `papers` directory.
//! Alternatively, you can also find a copy at
//! https://flint.cs.yale.edu/cs430/coq/pdf/Reference-Manual.pdf
//! Pages 122 and 123 are relevant.
//!
//! You can find a more concise (but less detailed) explanation at
//! > Christine Paulin-Mohring. Introduction to the Calculus of Inductive Constructions.
//! > Bruno Woltzenlogel Paleo; David Delahaye.
//! > All about Proofs, Proofs for All, 55, College Publications, 2015, Studies
//! > in Logic (Mathematical logic and foundations), 978-1-84890-166-7. ffhal-01094195f
//! This repository contains a copy in the `papers` directory.
//! Alternatively, you can also find a copy at
//! https://inria.hal.science/hal-01094195/document
//! Page 7 is relevant.

use super::*;

use crate::syntax_tree::minimal_ast::node_path::{self, NodeEdge, NodePath};

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

#[derive(Clone, Copy, Debug)]
enum Context<'a> {
    Base(RestrictionStatusVec),
    Snoc(&'a Context<'a>, RestrictionStatusVec),
}

type RestrictionStatusVec = RepeatVec<IsRestrictedRecursiveIndEntry>;

/// A repeat vec is a vec where all the elements are the same.
/// For example, `vec![]`, `vec![true]`, `vec![true, true]`,
/// and `vec![true; 3]` are all repeat vecs.
///
/// While we _could_ represent a repeat vec with a normal `Vec`,
/// it would waste memory.
///
/// Since all the elements are the same,
/// we can save memory by only storing one copy of the element.
/// This is exactly what `RepeatVec` does.
#[derive(Clone, Copy, Debug)]
struct RepeatVec<T> {
    val: T,
    len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IsRestrictedRecursiveIndEntry(pub bool);

impl PositivityChecker<'_> {
    pub fn check_ind_positivity_assuming_it_is_otherwise_well_typed<A: AuxDataFamily>(
        &mut self,
        ind: RcHashed<ast::Ind<A>>,
        tcon_len: usize,
    ) -> Result<(), TypeError<A>> {
        let base = RestrictionStatusVec::unrestricted(tcon_len);
        self.check_ind(&ind.hashee, Context::Base(base))
    }
}

impl PositivityChecker<'_> {
    fn check<A: AuxDataFamily>(
        &mut self,
        expr: ast::Expr<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        match expr {
            ast::Expr::Ind(e) => self.check_ind(&e.hashee, context),
            ast::Expr::Vcon(e) => self.check_vcon(&e.hashee, context),
            ast::Expr::Match(e) => self.check_match(&e.hashee, context),
            ast::Expr::Fun(e) => self.check_fun(&e.hashee, context),
            ast::Expr::App(e) => self.check_app(&e.hashee, context),
            ast::Expr::For(e) => self.check_for(&e.hashee, context),
            ast::Expr::Deb(_) | ast::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_ind<A: AuxDataFamily>(
        &mut self,
        ind: &ast::Ind<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check_dependent_exprs(&ind.index_types.hashee, context)?;

        let singleton = RestrictionStatusVec::restricted_singleton();
        let extended_context = context.collapsing_snoc(singleton);
        self.check_vcon_defs(&ind.vcon_defs.hashee, extended_context)?;

        Ok(())
    }

    fn check_vcon_defs<A: AuxDataFamily>(
        &mut self,
        defs: &[ast::VconDef<A>],
        context: Context,
    ) -> Result<(), TypeError<A>> {
        for def in defs {
            self.check_vcon_def(def, context)?;
        }
        Ok(())
    }

    fn check_vcon_def<A: AuxDataFamily>(
        &mut self,
        def: &ast::VconDef<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check_dependent_exprs(&def.param_types.hashee, context)?;

        let extension = RestrictionStatusVec::unrestricted(def.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
        self.check_independent_exprs(&def.index_args.hashee, extended_context)?;

        self.vcon_positivity_checker()
            .assert_vcon_type_satisfies_positivity_condition(def, context)?;

        Ok(())
    }

    fn check_vcon<A: AuxDataFamily>(
        &mut self,
        vcon: &ast::Vcon<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check_ind(&vcon.ind.hashee, context)
    }

    fn check_match<A: AuxDataFamily>(
        &mut self,
        match_: &ast::Match<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check(match_.matchee.clone(), context)?;

        let return_type_extension = RestrictionStatusVec::unrestricted(match_.return_type_arity);
        let return_type_context = context.collapsing_snoc(return_type_extension);
        self.check(match_.return_type.clone(), return_type_context)?;

        self.check_match_cases(&match_.cases.hashee, context)?;

        Ok(())
    }

    fn check_match_cases<A: AuxDataFamily>(
        &mut self,
        cases: &[ast::MatchCase<A>],
        context: Context,
    ) -> Result<(), TypeError<A>> {
        for case in cases {
            self.check_match_case(case, context)?;
        }
        Ok(())
    }

    fn check_match_case<A: AuxDataFamily>(
        &mut self,
        case: &ast::MatchCase<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        let return_val_extension = RestrictionStatusVec::unrestricted(case.arity);
        let return_val_context = context.collapsing_snoc(return_val_extension);
        self.check(case.return_val.clone(), return_val_context)?;

        Ok(())
    }

    fn check_fun<A: AuxDataFamily>(
        &mut self,
        fun: &ast::Fun<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check_dependent_exprs(&fun.param_types.hashee, context)?;

        let return_type_extension =
            RestrictionStatusVec::unrestricted(fun.param_types.hashee.len());
        let context_with_params = context.collapsing_snoc(return_type_extension);
        self.check(fun.return_type.clone(), context_with_params)?;

        let recursive_fun_singleton = RestrictionStatusVec::unrestricted(1);
        let context_with_params_and_recursive_fun =
            context_with_params.collapsing_snoc(recursive_fun_singleton);
        self.check(
            fun.return_val.clone(),
            context_with_params_and_recursive_fun,
        )?;

        Ok(())
    }

    fn check_app<A: AuxDataFamily>(
        &mut self,
        app: &ast::App<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check(app.callee.clone(), context)?;
        self.check_independent_exprs(&app.args.hashee, context)?;
        Ok(())
    }

    fn check_for<A: AuxDataFamily>(
        &mut self,
        for_: &ast::For<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check_dependent_exprs(&for_.param_types.hashee, context)?;

        let extension = RestrictionStatusVec::unrestricted(for_.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
        self.check(for_.return_type.clone(), extended_context)?;

        Ok(())
    }

    fn check_dependent_exprs<A: AuxDataFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        context: Context,
    ) -> Result<(), TypeError<A>> {
        if exprs.is_empty() {
            return Ok(());
        }

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extension = RestrictionStatusVec::unrestricted(i);
            let extended_context = context.collapsing_snoc(extension);
            self.check(expr, extended_context)?;
        }

        Ok(())
    }

    fn check_independent_exprs<A: AuxDataFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        context: Context,
    ) -> Result<(), TypeError<A>> {
        for expr in exprs.iter().cloned() {
            self.check(expr, context)?;
        }
        Ok(())
    }
}

impl PositivityChecker<'_> {
    fn vcon_positivity_checker(&mut self) -> VconPositivityChecker {
        VconPositivityChecker(self.clone_mut())
    }

    fn clone_mut<'a>(&'a mut self) -> PositivityChecker<'a> {
        PositivityChecker {
            typechecker: &mut self.typechecker,
        }
    }
}

impl VconPositivityChecker<'_> {
    fn assert_vcon_type_satisfies_positivity_condition<A: AuxDataFamily>(
        &mut self,
        def: &ast::VconDef<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        self.check_vcon_def_param_types(def, context)?;

        let extension = RestrictionStatusVec::unrestricted(def.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
        self.check_vcon_def_index_args(def, extended_context)?;

        Ok(())
    }

    fn check_vcon_def_param_types<A: AuxDataFamily>(
        &mut self,
        def: &ast::VconDef<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        let param_types_ast = self
            .0
            .typechecker
            .span_remover
            .convert_expressions(&def.param_types.hashee);
        let normalized_param_types = self
            .0
            .typechecker
            .evaluator
            .eval_expressions(param_types_ast);

        for (i, param_type) in normalized_param_types
            .raw()
            .hashee
            .iter()
            .cloned()
            .enumerate()
        {
            let extension = RestrictionStatusVec::unrestricted(i);
            let extended_context = context.collapsing_snoc(extension);
            self.strict_positivity_checker()
                .check(param_type, extended_context, NodePath::Nil)
                .map_err(|path_from_param_type_to_problematic_deb| {
                    TypeError::VconDefParamTypeFailsStrictPositivityCondition {
                        def: def.clone(),
                        param_type_index: i,
                        normalized_param_type: normalized_param_types.to_hashee().index(i).cloned(),
                        path_from_param_type_to_problematic_deb,
                    }
                })?;
        }

        Ok(())
    }

    fn check_vcon_def_index_args<A: AuxDataFamily>(
        &mut self,
        def: &ast::VconDef<A>,
        context: Context,
    ) -> Result<(), TypeError<A>> {
        let index_args_ast = self
            .0
            .typechecker
            .span_remover
            .convert_expressions(&def.index_args.hashee);
        let normalized_index_args = self
            .0
            .typechecker
            .evaluator
            .eval_expressions(index_args_ast);

        for (i, index_arg) in normalized_index_args
            .raw()
            .hashee
            .iter()
            .cloned()
            .enumerate()
        {
            self.absence_checker()
                .check(index_arg, context, NodePath::Nil)
                .map_err(|path_from_index_arg_to_problematic_deb| {
                    TypeError::RecursiveIndParamAppearsInVconDefIndexArg {
                        def: def.clone(),
                        index_arg_index: i,
                        normalized_index_arg: normalized_index_args.to_hashee().index(i).cloned(),
                        path_from_index_arg_to_problematic_deb,
                    }
                })?;
        }

        Ok(())
    }
}

impl VconPositivityChecker<'_> {
    fn strict_positivity_checker(&mut self) -> StrictPositivityChecker {
        StrictPositivityChecker(self.0.clone_mut())
    }

    fn absence_checker(&mut self) -> AbsenceChecker {
        AbsenceChecker(self.0.clone_mut())
    }
}

impl StrictPositivityChecker<'_> {
    fn check(
        &mut self,
        expr: minimal_ast::Expr,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match expr {
            minimal_ast::Expr::Ind(e) => self.check_ind(&e.hashee, context, path),

            minimal_ast::Expr::Deb(_) => Ok(()),

            minimal_ast::Expr::App(e) => self.check_app(&e.hashee, context, path),

            minimal_ast::Expr::For(e) => self.check_for(&e.hashee, context, path),

            minimal_ast::Expr::Vcon(_)
            | minimal_ast::Expr::Match(_)
            | minimal_ast::Expr::Fun(_)
            | minimal_ast::Expr::Universe(_) => self.absence_checker().check(expr, context, path),
        }
    }

    fn check_ind(
        &mut self,
        ind: &minimal_ast::Ind,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_index_types = NodePath::Snoc(&path, node_path::IND_INDEX_TYPES);
        self.absence_checker().check_dependent_exprs(
            &ind.index_types.hashee,
            context,
            path_to_index_types,
        )?;

        let extension = RestrictionStatusVec::restricted_singleton();
        let extended_context = context.collapsing_snoc(extension);
        let path_to_vcon_defs = NodePath::Snoc(&path, node_path::IND_VCON_DEFS);
        self.check_vcon_defs(&ind.vcon_defs.hashee, extended_context, path_to_vcon_defs)?;

        Ok(())
    }

    fn check_vcon_defs(
        &mut self,
        defs: &[minimal_ast::VconDef],
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
        def: minimal_ast::VconDef,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::VCON_DEF_PARAM_TYPES);
        self.check_dependent_exprs(&def.param_types.hashee, context, path_to_param_types)?;

        let extension = RestrictionStatusVec::unrestricted(def.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
        let path_to_index_args = NodePath::Snoc(&path, node_path::VCON_DEF_INDEX_ARGS);
        self.absence_checker().check_independent_exprs(
            &def.index_args.hashee,
            extended_context,
            path_to_index_args,
        )?;

        Ok(())
    }

    fn check_app(
        &mut self,
        app: &minimal_ast::App,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_callee = NodePath::Snoc(&path, node_path::APP_CALLEE);
        self.check_app_callee(app.callee.clone(), context, path_to_callee)?;

        let path_to_args = NodePath::Snoc(&path, node_path::APP_ARGS);
        self.absence_checker()
            .check_independent_exprs(&app.args.hashee, context, path_to_args)?;

        Ok(())
    }

    fn check_app_callee(
        &mut self,
        callee: minimal_ast::Expr,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match callee {
            minimal_ast::Expr::Ind(e) => self.check_ind(&e.hashee, context, path),

            minimal_ast::Expr::Deb(_) => Ok(()),

            minimal_ast::Expr::Vcon(_)
            | minimal_ast::Expr::Match(_)
            | minimal_ast::Expr::Fun(_)
            | minimal_ast::Expr::App(_)
            | minimal_ast::Expr::For(_)
            | minimal_ast::Expr::Universe(_) => self.absence_checker().check(callee, context, path),
        }
    }

    fn check_for(
        &mut self,
        for_: &minimal_ast::For,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::FOR_PARAM_TYPES);
        self.absence_checker().check_dependent_exprs(
            &for_.param_types.hashee,
            context,
            path_to_param_types,
        )?;

        let extension = RestrictionStatusVec::unrestricted(for_.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
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
        exprs: &[minimal_ast::Expr],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        if exprs.is_empty() {
            return Ok(());
        }

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extension = RestrictionStatusVec::unrestricted(i);
            let extended_context = context.collapsing_snoc(extension);
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(expr, extended_context, extended_path)?;
        }

        Ok(())
    }
}

impl StrictPositivityChecker<'_> {
    fn absence_checker(&mut self) -> AbsenceChecker {
        AbsenceChecker(self.0.clone_mut())
    }
}

impl AbsenceChecker<'_> {
    fn check(
        &mut self,
        expr: minimal_ast::Expr,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        match expr {
            minimal_ast::Expr::Ind(e) => self.check_ind(&e.hashee, context, path),
            minimal_ast::Expr::Vcon(e) => self.check_vcon(&e.hashee, context, path),
            minimal_ast::Expr::Match(e) => self.check_match(&e.hashee, context, path),
            minimal_ast::Expr::Fun(e) => self.check_fun(&e.hashee, context, path),
            minimal_ast::Expr::App(e) => self.check_app(&e.hashee, context, path),
            minimal_ast::Expr::For(e) => self.check_for(&e.hashee, context, path),
            minimal_ast::Expr::Deb(e) => self.check_deb(&e.hashee, context, path),
            minimal_ast::Expr::Universe(_) => Ok(()),
        }
    }

    fn check_ind(
        &mut self,
        ind: &minimal_ast::Ind,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_index_types = NodePath::Snoc(&path, node_path::IND_INDEX_TYPES);
        self.check_dependent_exprs(&ind.index_types.hashee, context, path_to_index_types)?;

        let singleton = RestrictionStatusVec::unrestricted(1);
        let extended_context = context.collapsing_snoc(singleton);
        let path_to_vcon_defs = NodePath::Snoc(&path, node_path::IND_VCON_DEFS);
        self.check_vcon_defs(&ind.vcon_defs.hashee, extended_context, path_to_vcon_defs)?;

        Ok(())
    }

    fn check_vcon_defs(
        &mut self,
        defs: &[minimal_ast::VconDef],
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
        def: minimal_ast::VconDef,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::VCON_DEF_PARAM_TYPES);
        self.check_dependent_exprs(&def.param_types.hashee, context, path_to_param_types)?;

        let extension = RestrictionStatusVec::unrestricted(def.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
        let path_to_index_args = NodePath::Snoc(&path, node_path::VCON_DEF_INDEX_ARGS);
        self.check_independent_exprs(&def.index_args.hashee, extended_context, path_to_index_args)?;

        Ok(())
    }

    fn check_vcon(
        &mut self,
        vcon: &minimal_ast::Vcon,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_ind = NodePath::Snoc(&path, node_path::VCON_IND);
        self.check_ind(&vcon.ind.hashee, context, path_to_ind)
    }

    fn check_match(
        &mut self,
        match_: &minimal_ast::Match,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_matchee = NodePath::Snoc(&path, node_path::MATCH_MATCHEE);
        self.check(match_.matchee.clone(), context, path_to_matchee)?;

        let return_type_extension = RestrictionStatusVec::unrestricted(match_.return_type_arity);
        let context_with_return_type_extension = context.collapsing_snoc(return_type_extension);
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
        cases: &[minimal_ast::MatchCase],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        for (i, case) in cases.iter().enumerate() {
            let extension = RestrictionStatusVec::unrestricted(case.arity);
            let extended_context = context.collapsing_snoc(extension);
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(case.return_val.clone(), extended_context, extended_path)?;
        }

        Ok(())
    }

    fn check_fun(
        &mut self,
        fun: &minimal_ast::Fun,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::FUN_PARAM_TYPES);
        self.check_dependent_exprs(&fun.param_types.hashee, context, path_to_param_types)?;

        let param_extension = RestrictionStatusVec::unrestricted(fun.param_types.hashee.len());
        let context_with_params = context.collapsing_snoc(param_extension);
        let path_to_return_type = NodePath::Snoc(&path, node_path::FUN_RETURN_TYPE);
        self.check(
            fun.return_type.clone(),
            context_with_params,
            path_to_return_type,
        )?;

        let recursive_fun_singleton = RestrictionStatusVec::unrestricted(1);
        let context_with_params_and_recursive_fun =
            context_with_params.collapsing_snoc(recursive_fun_singleton);
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
        app: &minimal_ast::App,
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
        for_: &minimal_ast::For,
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        let path_to_param_types = NodePath::Snoc(&path, node_path::FOR_PARAM_TYPES);
        self.check_dependent_exprs(&for_.param_types.hashee, context, path_to_param_types)?;

        let extension = RestrictionStatusVec::unrestricted(for_.param_types.hashee.len());
        let extended_context = context.collapsing_snoc(extension);
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
        deb: &minimal_ast::DebNode,
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
        exprs: &[minimal_ast::Expr],
        context: Context,
        path: NodePath,
    ) -> Result<(), Vec<NodeEdge>> {
        if exprs.is_empty() {
            return Ok(());
        }

        for (i, expr) in exprs.iter().cloned().enumerate() {
            let extension = RestrictionStatusVec::unrestricted(i);
            let extended_context = context.collapsing_snoc(extension);
            let extended_path = NodePath::Snoc(&path, NodeEdge(i));
            self.check(expr, extended_context, extended_path)?;
        }

        Ok(())
    }

    fn check_independent_exprs(
        &mut self,
        exprs: &[minimal_ast::Expr],
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
    /// This function returns a context that is equivalent to `Context::Snoc(self, extension)`.
    /// However, it tries to minimize the number of nodes in the linked list
    /// by performing some optimizations:
    /// - If `extension` is empty, it returns the original context.
    /// - If `extension.restricted` is the same as the last entry in the original context,
    ///   it simply returns a copy of the original context with the rac extended by `extension.len`.
    /// - Otherwise, it returns `Context::Snoc(self, extension)`.
    pub fn collapsing_snoc<'a>(&'a self, extension: RestrictionStatusVec) -> Context<'a> {
        if extension.is_empty() {
            return *self;
        }

        match self {
            Context::Base(rac) if rac.val == extension.val => Context::Base(RestrictionStatusVec {
                val: extension.val,
                len: rac.len + extension.len,
            }),

            Context::Snoc(rdc, rac) if rac.val == extension.val => Context::Snoc(
                rdc,
                RestrictionStatusVec {
                    val: extension.val,
                    len: rac.len + extension.len,
                },
            ),

            _ => Context::Snoc(self, extension),
        }
    }
}

impl Context<'_> {
    pub fn get(&self, deb: Deb) -> Option<IsRestrictedRecursiveIndEntry> {
        match self {
            Context::Base(entries) => {
                let index = (entries.len()).checked_sub(1 + deb.0)?;
                entries.get_copied(index)
            }

            Context::Snoc(subcontext, entries) => {
                if let Some(index) = (entries.len()).checked_sub(1 + deb.0) {
                    entries.get_copied(index)
                } else {
                    subcontext.get(Deb(deb.0 - entries.len()))
                }
            }
        }
    }
}

impl RestrictionStatusVec {
    pub fn restricted_singleton() -> Self {
        Self {
            val: IsRestrictedRecursiveIndEntry(true),
            len: 1,
        }
    }

    pub fn unrestricted(len: usize) -> Self {
        Self {
            val: IsRestrictedRecursiveIndEntry(false),
            len,
        }
    }
}

impl<T> RepeatVec<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// We name this method `get_copied` instead of `get` because
    /// `get` conventionally returns `Option<&T>` instead of `Option<T>`.
    pub fn get_copied(&self, index: usize) -> Option<T>
    where
        T: Copy,
    {
        if index < self.len {
            Some(self.val)
        } else {
            None
        }
    }
}
