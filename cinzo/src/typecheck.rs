use crate::{
    ast::*,
    eval::{EvalError, Evaluator, NormalForm, Normalized},
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
}

#[derive(Debug, Clone, Copy)]
pub enum LazyTypeContext<'a> {
    Base(&'a [NormalForm]),
    Snoc(&'a LazyTypeContext<'a>, &'a [NormalForm]),
}

impl LazyTypeContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazyTypeContext::Base(types) => types.len(),
            LazyTypeContext::Snoc(subcontext, types) => subcontext.len() + types.len(),
        }
    }

    pub fn get(&self, deb: Deb) -> Option<NormalForm> {
        match self {
            LazyTypeContext::Base(types) => {
                let index = (types.len() - 1).checked_sub(deb.0)?;
                types.get(index).cloned()
            }
            LazyTypeContext::Snoc(subcontext, types) => {
                if let Some(index) = (types.len() - 1).checked_sub(deb.0) {
                    types.get(index).cloned()
                } else {
                    subcontext.get(Deb(deb.0 - types.len()))
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
    pub left: &'a Expr,
    pub right: &'a Expr,
}

impl LazySubstitutionContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazySubstitutionContext::Base(subs) => subs.len(),
            LazySubstitutionContext::Cons(subs, rest) => subs.len() + rest.len(),
        }
    }
}

impl App {
    fn collapse_if_nullary(self) -> Expr {
        if self.args.value.is_empty() {
            self.callee
        } else {
            Expr::App(Rc::new(Hashed::new(self)))
        }
    }
}

impl For {
    fn collapse_if_nullary(self) -> Expr {
        if self.param_types.value.is_empty() {
            self.return_type
        } else {
            Expr::For(Rc::new(Hashed::new(self)))
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

        let normalized_index_types = self
            .eval_expressions(ind.value.index_types.clone())
            .expect(WELL_TYPED_IMPLIES_EVALUATABLE_MESSAGE)
            .into_raw();
        let return_type = self.get_ind_return_type(ind).into_raw();
        let already_normalized = For {
            param_types: normalized_index_types,
            return_type,
        }
        .collapse_if_nullary();
        Ok(self.assert_normal_form_or_panic(already_normalized))
    }

    fn perform_ind_precheck(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let index_type_types =
            self.get_type_of_dependent_expressions(ind.value.index_types.clone(), tcon, scon)?;
        assert_every_expr_is_universe(&index_type_types.value).map_err(
            |offending_index_type_index| TypeError::UnexpectedNonTypeExpression {
                expr: ind.value.index_types.value[offending_index_type_index].clone(),
                type_: self.assert_normal_form_or_panic(
                    index_type_types.value[offending_index_type_index].clone(),
                ),
            },
        )?;

        assert_every_lhs_universe_is_less_than_or_equal_to_rhs(
            &index_type_types.value,
            ind.value.universe_level,
        )
        .map_err(
            |offending_index_type_index| TypeError::UnexpectedNonTypeExpression {
                expr: ind.value.index_types.value[offending_index_type_index].clone(),
                type_: self.assert_normal_form_or_panic(
                    index_type_types.value[offending_index_type_index].clone(),
                ),
            },
        )?;

        self.assert_ind_vcon_defs_are_well_typed(ind, tcon, scon)?;

        Ok(())
    }

    fn assert_ind_vcon_defs_are_well_typed(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        todo!()
    }

    fn get_ind_return_type(&mut self, ind: RcHashed<Ind>) -> NormalForm {
        self.assert_normal_form_or_panic(Expr::Universe(Rc::new(Hashed::new(UniverseNode {
            level: ind.value.universe_level,
        }))))
    }

    fn get_type_of_dependent_expressions(
        &mut self,
        exprs: RcHashed<Box<[Expr]>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<RcHashed<Box<[Expr]>>, TypeError> {
        let mut out: Vec<NormalForm> = Vec::with_capacity(exprs.value.len());

        for expr in exprs.value.iter() {
            let current_tcon = LazyTypeContext::Snoc(&tcon, &out);
            let type_ = self.get_type(expr.clone(), current_tcon, scon)?;
            out.push(type_);
        }

        let out: Vec<Expr> = out.into_iter().map(NormalForm::into_raw).collect();

        Ok(Rc::new(Hashed::new(out.into_boxed_slice())))
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
        self.get_type_of_vcon_def(def, vcon.value.ind.clone(), tcon, scon)
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

    fn get_type_of_vcon_def(
        &mut self,
        def: &VconDef,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_types =
            self.get_type_of_dependent_expressions(def.param_types.clone(), tcon, scon)?;
        let return_type = App {
            callee: Expr::Ind(ind),
            args: def.index_args.clone(),
        }
        .collapse_if_nullary();
        let already_normalized = For {
            param_types,
            return_type,
        }
        .collapse_if_nullary();
        let trivially_normalized = self.assert_normal_form_or_panic(already_normalized);
        Ok(trivially_normalized)
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
            .eval(Expr::Universe(Rc::new(Hashed::new(UniverseNode {
                level: UniverseLevel(universe.value.level.0 + 1),
            }))))
            .expect("A universe should always evaluate to itself."));
    }

    fn eval(&mut self, expr: Expr) -> Result<NormalForm, EvalError> {
        self.evaluator.eval(expr)
    }

    fn eval_expressions(
        &mut self,
        exprs: RcHashed<Box<[Expr]>>,
    ) -> Result<Normalized<RcHashed<Box<[Expr]>>>, EvalError> {
        self.evaluator.eval_expressions(exprs)
    }

    fn assert_normal_form_or_panic(&mut self, expr: Expr) -> NormalForm {
        let original_digest = expr.digest().clone();
        let normal_form = self
            .evaluator
            .eval(expr)
            .expect("The input should already be in normal form.");
        let new_digest = normal_form.raw().digest();
        assert_eq!(original_digest, *new_digest);
        normal_form
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
) -> Result<(), usize> {
    for (i, expr) in lhs.iter().enumerate() {
        let lhs_level = match expr {
            Expr::Universe(universe) => universe.value.level,
            _ => continue,
        };

        if lhs_level > rhs {
            return Err(i);
        }
    }

    Ok(())
}
