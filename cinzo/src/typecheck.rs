use crate::{
    ast::*,
    eval::{EvalError, Evaluator, NormalForm},
};

use std::rc::Rc;

type RcHashed<T> = Rc<Hashed<T>>;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: RcHashed<DebNode>,
        tcon_len: usize,
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
        if ind.value.index_types.value.is_empty() {
            self.get_ind_return_type(ind)
        } else {
            self.get_type_of_indexed_ind(ind, tcon, scon)
        }
    }

    fn get_ind_return_type(&mut self, ind: RcHashed<Ind>) -> Result<NormalForm, TypeError> {
        Ok(self
            .eval(Expr::Universe(Rc::new(Hashed::new(UniverseNode {
                level: ind.value.universe_level,
            }))))
            .expect("A universe should always evaluate to itself."))
    }

    fn get_type_of_indexed_ind(
        &mut self,
        ind: RcHashed<Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_types =
            self.get_type_of_dependent_expressions(ind.value.index_types.clone(), tcon, scon)?;
        let return_type = self.get_ind_return_type(ind)?.into_raw();
        return Ok(self
            .eval(Expr::For(Rc::new(Hashed::new(For {
                param_types,
                return_type,
            }))))
            .expect("A forall with normalized param types and return type should be normalized, and thus trivially evaluatable."));
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
        todo!()
    }

    fn get_type_of_match(
        &mut self,
        r#match: RcHashed<Match>,
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
        r#for: RcHashed<For>,
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
}
