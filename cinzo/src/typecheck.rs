use crate::{
    ast::*,
    eval::{Evaluator, NormalForm, Normalized},
};

use std::rc::Rc;

type RcHashed<T> = Rc<Hashed<T>>;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: RcHashed<DebNode>,
        tcon_len: usize,
    },
    InvalidVconIndex(RcHashed<Vcon>),
    UnexpectedNonTypeExpression {
        expr: Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        expr: Expr,
        level: UniverseLevel,
        max_permitted_level: UniverseLevel,
    },
    WrongNumberOfIndexArguments {
        def: VconDef,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum LazyTypeContext<'a> {
    Base(Normalized<&'a [Expr]>),
    Snoc(&'a LazyTypeContext<'a>, Normalized<&'a [Expr]>),
}

impl LazyTypeContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazyTypeContext::Base(types) => types.raw().len(),
            LazyTypeContext::Snoc(subcontext, types) => subcontext.len() + types.raw().len(),
        }
    }

    pub fn get(&self, deb: Deb) -> Option<NormalForm> {
        match self {
            LazyTypeContext::Base(types) => {
                let index = (types.raw().len() - 1).checked_sub(deb.0)?;
                types.get(index).map(Normalized::cloned)
            }
            LazyTypeContext::Snoc(subcontext, types) => {
                if let Some(index) = (types.raw().len() - 1).checked_sub(deb.0) {
                    types.get(index).map(Normalized::cloned)
                } else {
                    subcontext.get(Deb(deb.0 - types.raw().len()))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LazySubstitutionContext<'a> {
    Base(&'a [LazySubstitution<'a>]),
    Cons(&'a [LazySubstitution<'a>], &'a LazySubstitutionContext<'a>),
}

#[derive(Debug, Clone, Copy)]
pub struct LazySubstitution<'a> {
    pub tcon_len: usize,
    pub left: &'a NormalForm,
    pub right: &'a NormalForm,
}

impl LazySubstitutionContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazySubstitutionContext::Base(subs) => subs.len(),
            LazySubstitutionContext::Cons(subs, rest) => subs.len() + rest.len(),
        }
    }
}

const WELL_TYPED_IMPLIES_EVALUATABLE_MESSAGE: &str =
    "A well-typed expression should be evaluatable.";

#[derive(Clone, Debug, Default)]
pub struct TypeChecker {
    pub evaluator: Evaluator,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TypeChecker {
    pub fn get_type(
        &mut self,
        expr: Expr,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        match expr {
            Expr::Ind(e) => self.get_type_of_ind(e, tcon, scon),
            Expr::Vcon(e) => self.get_type_of_vcon(e, tcon, scon),
            Expr::Match(e) => self.get_type_of_match(e, tcon, scon),
            Expr::Fun(e) => self.get_type_of_fun(e, tcon, scon),
            Expr::App(e) => self.get_type_of_app(e, tcon, scon),
            Expr::For(e) => self.get_type_of_for(e, tcon, scon),
            Expr::Deb(e) => self.get_type_of_deb(e, tcon),
            Expr::Universe(e) => self.get_type_of_universe(e),
        }
    }

    fn get_type_of_ind(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_ind_precheck(ind.clone(), tcon, scon)?;
        Ok(self.get_ind_type_assuming_ind_is_well_typed(ind))
    }

    fn perform_ind_precheck(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let index_type_types =
            self.get_types_of_dependent_expressions(ind.value.index_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(&index_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: ind.value.index_types.value[offender_index].clone(),
                type_: index_type_types.index(offender_index).cloned(),
            }
        })?;

        // Once we verify that the index types are all well-typed,
        // it is safe to construct a predicted type for the ind type.
        let predicted_ind_type = self.get_ind_type_assuming_ind_is_well_typed(ind.clone());

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &index_type_types.raw(),
            ind.value.universe_level,
        )
        .map_err(|(offender_index, offender_level)| {
            TypeError::UniverseInconsistencyInIndDef {
                expr: ind.value.index_types.value[offender_index].clone(),
                level: offender_level,
                max_permitted_level: ind.value.universe_level,
            }
        })?;

        self.assert_ind_vcon_defs_are_well_typed(ind, predicted_ind_type, tcon, scon)?;

        Ok(())
    }

    fn assert_ind_vcon_defs_are_well_typed(
        &mut self,
        ind: RcHashed<Ind>,
        predicted_ind_type: NormalForm,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        for def in ind.value.vcon_defs.value.iter() {
            self.assert_ind_vcon_def_is_well_typed(
                ind.clone(),
                predicted_ind_type.clone(),
                def,
                tcon,
                scon,
            )?;
        }
        Ok(())
    }

    fn assert_ind_vcon_def_is_well_typed(
        &mut self,
        ind: RcHashed<Ind>,
        predicted_ind_type: NormalForm,
        def: &VconDef,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let recursive_ind_entry: Normalized<Vec<Expr>> =
            std::iter::once(predicted_ind_type).collect();
        let tcon_with_recursive_ind_entry =
            LazyTypeContext::Snoc(&tcon, recursive_ind_entry.as_slice());
        let param_type_types = self.get_types_of_dependent_expressions(
            def.param_types.clone(),
            tcon_with_recursive_ind_entry,
            scon,
        )?;

        let tcon_with_params =
            LazyTypeContext::Snoc(&tcon_with_recursive_ind_entry, param_type_types.as_slice());
        self.get_types_of_independent_expressions(def.index_args.clone(), tcon_with_params, scon)?;

        if ind.value.index_types.value.len() != def.index_args.value.len() {
            return Err(TypeError::WrongNumberOfIndexArguments {
                def: def.clone(),
                expected: ind.value.index_types.value.len(),
                actual: def.index_args.value.len(),
            });
        }

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &param_type_types.raw(),
            ind.value.universe_level,
        )
        .map_err(|(offender_index, offender_level)| {
            TypeError::UniverseInconsistencyInIndDef {
                expr: def.param_types.value[offender_index].clone(),
                level: offender_level,
                max_permitted_level: ind.value.universe_level,
            }
        })?;

        self.assert_vcon_def_is_strictly_positive(ind, def, tcon, scon)?;

        Ok(())
    }

    fn assert_vcon_def_is_strictly_positive(
        &mut self,
        _ind: RcHashed<Ind>,
        _def: &VconDef,
        _tcon: LazyTypeContext,
        _scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        // TODO: Actually check positivity.
        Ok(())
    }

    /// This function assumes that the index types are well-typed.
    /// If they are not, this will cause (probably undetectable) bugs.
    ///
    /// However, you may safely call this function even if the vcon defs
    /// are ill-typed.
    fn get_ind_type_assuming_ind_is_well_typed(&mut self, ind: RcHashed<Ind>) -> NormalForm {
        let normalized_index_types = self
            .evaluator
            .eval_expressions(ind.value.index_types.clone())
            .expect(WELL_TYPED_IMPLIES_EVALUATABLE_MESSAGE);
        let return_type = self.get_ind_return_type(ind);
        Normalized::for_(normalized_index_types, return_type).collapse_if_nullary()
    }

    fn get_ind_return_type(&mut self, ind: RcHashed<Ind>) -> NormalForm {
        Normalized::universe(UniverseNode {
            level: ind.value.universe_level,
        })
    }

    fn get_type_of_vcon(
        &mut self,
        vcon: RcHashed<Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_vcon_precheck(vcon.clone(), tcon, scon)?;

        let vcon_index = vcon.value.vcon_index;
        let defs: &[VconDef] = &vcon.value.ind.value.vcon_defs.value;
        let Some(def) = defs.get(vcon_index) else {
            return Err(TypeError::InvalidVconIndex(vcon));
        };
        self.get_type_of_trusted_vcon_def(def, vcon.value.ind.clone())
    }

    fn perform_vcon_precheck(
        &mut self,
        vcon: RcHashed<Vcon>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        self.get_type_of_ind(vcon.value.ind.clone(), tcon, scon)?;
        Ok(())
    }

    fn get_type_of_trusted_vcon_def(
        &mut self,
        def: &VconDef,
        ind: RcHashed<Ind>,
    ) -> Result<NormalForm, TypeError> {
        let normalized_param_types = self
            .evaluator
            .eval_expressions(def.param_types.clone())
            .expect(WELL_TYPED_IMPLIES_EVALUATABLE_MESSAGE);
        let normalized_ind = self
            .evaluator
            .eval_ind(ind.clone())
            .expect(WELL_TYPED_IMPLIES_EVALUATABLE_MESSAGE);
        let normalized_index_args = self
            .evaluator
            .eval_expressions(def.index_args.clone())
            .expect(WELL_TYPED_IMPLIES_EVALUATABLE_MESSAGE);
        let return_type = Normalized::app_with_ind_callee(normalized_ind, normalized_index_args)
            .collapse_if_nullary();
        Ok(Normalized::for_(normalized_param_types, return_type).collapse_if_nullary())
    }

    fn get_type_of_match(
        &mut self,
        match_: RcHashed<Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        todo!()
    }

    fn get_type_of_fun(
        &mut self,
        fun: RcHashed<Fun>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        todo!()
    }

    fn get_type_of_app(
        &mut self,
        app: RcHashed<App>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        todo!()
    }

    fn get_type_of_for(
        &mut self,
        for_: RcHashed<For>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        todo!()
    }

    fn get_type_of_deb(
        &mut self,
        deb: RcHashed<DebNode>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        if let Some(expr) = tcon.get(deb.value.deb) {
            return Ok(expr);
        }

        return Err(TypeError::InvalidDeb {
            deb,
            tcon_len: tcon.len(),
        });
    }

    fn get_type_of_universe(
        &mut self,
        universe: RcHashed<UniverseNode>,
    ) -> Result<NormalForm, TypeError> {
        return Ok(self
            .evaluator
            .eval(Expr::Universe(Rc::new(Hashed::new(UniverseNode {
                level: UniverseLevel(universe.value.level.0 + 1),
            }))))
            .expect("A universe should always evaluate to itself."));
    }

    fn get_types_of_dependent_expressions(
        &mut self,
        exprs: RcHashed<Box<[Expr]>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<Expr>>, TypeError> {
        let mut out: Normalized<Vec<Expr>> =
            Normalized::transpose_from_vec(Vec::with_capacity(exprs.value.len()));

        for expr in exprs.value.iter() {
            let current_tcon = LazyTypeContext::Snoc(&tcon, out.as_slice());
            let type_ = self.get_type(expr.clone(), current_tcon, scon)?;
            out.push(type_);
        }

        Ok(out)
    }

    fn get_types_of_independent_expressions(
        &mut self,
        exprs: RcHashed<Box<[Expr]>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<RcHashed<Box<[Expr]>>, TypeError> {
        let mut out: Vec<Expr> = Vec::with_capacity(exprs.value.len());

        for expr in exprs.value.iter() {
            let type_ = self.get_type(expr.clone(), tcon, scon)?.into_raw();
            out.push(type_);
        }

        Ok(Rc::new(Hashed::new(out.into_boxed_slice())))
    }
}

fn assert_every_expr_is_universe(exprs: &[Expr]) -> Result<(), usize> {
    for (i, expr) in exprs.iter().enumerate() {
        if !expr.is_universe() {
            return Err(i);
        }
    }

    Ok(())
}

impl Expr {
    fn is_universe(&self) -> bool {
        match self {
            Expr::Universe(_) => true,
            _ => false,
        }
    }
}

fn assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
    lhs: &[Expr],
    rhs: UniverseLevel,
) -> Result<(), (usize, UniverseLevel)> {
    for (i, expr) in lhs.iter().enumerate() {
        let lhs_level = match expr {
            Expr::Universe(universe) => universe.value.level,
            _ => continue,
        };

        if lhs_level > rhs {
            return Err((i, lhs_level));
        }
    }

    Ok(())
}
