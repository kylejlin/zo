// TODO: Track `original`, or delete it.

use std::rc::Rc;

use crate::{ast::*, deb_shift_cache::DebShiftCache, nohash_hashmap::NoHashHashMap};

#[derive(Clone, Debug)]
pub enum EvalError {}

#[derive(Clone, Debug)]
pub struct Normalized<T>(T);

pub type NormalForm = Normalized<Expr>;

impl From<NormalForm> for Expr {
    fn from(nf: NormalForm) -> Self {
        nf.0
    }
}

impl<T> Normalized<T> {
    pub fn into_raw(self) -> T {
        self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    pub eval_expr_cache: NoHashHashMap<Digest, Result<NormalForm, EvalError>>,
    pub eval_exprs_cache: NoHashHashMap<Digest, Result<Normalized<RcExprs>, EvalError>>,
    pub eval_vcon_def_cache: NoHashHashMap<Digest, Result<Normalized<RcVconDef>, EvalError>>,
    pub eval_vcon_defs_cache: NoHashHashMap<Digest, Result<Normalized<RcVconDefs>, EvalError>>,
    pub deb_shift_cache: DebShiftCache,
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }
}

type RcExprs = Rc<Hashed<Box<[Expr]>>>;
type RcVconDef = Rc<Hashed<VariantConstructorDef>>;
type RcVconDefs = Rc<Hashed<Box<[RcVconDef]>>>;

// #[derive(Clone, Debug)]
// pub struct Ind {
//     pub name: Rc<StringValue>,
//     pub universe_level: usize,
//     pub index_types: Rc<Hashed<Box<[Expr]>>>,
//     pub constructor_defs: Rc<Hashed<Box<[Hashed<VariantConstructorDef>]>>>,
//     pub original: Option<Rc<cst::Ind>>,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
// pub struct StringValue(pub String);

// #[derive(Debug, Clone)]
// pub struct VariantConstructorDef {
//     pub param_types: Rc<Hashed<Box<[Expr]>>>,
//     pub index_args: Rc<Hashed<Box<[Expr]>>>,
//     pub original: Option<Rc<cst::VariantConstructorDef>>,
// }

// #[derive(Debug, Clone)]
// pub struct Vcon {
//     pub ind: Rc<Hashed<Ind>>,
//     pub vcon_index: usize,
//     pub original: Option<Rc<cst::Vcon>>,
// }

// #[derive(Debug, Clone)]
// pub struct Match {
//     pub matchee: Rc<Expr>,
//     pub return_type: Rc<Expr>,
//     pub cases: Rc<Hashed<Box<[Expr]>>>,
//     pub original: Option<Rc<cst::Match>>,
// }

// #[derive(Debug, Clone)]
// pub struct Fun {
//     pub decreasing_index: Option<usize>,
//     pub param_types: Rc<Hashed<Box<[Expr]>>>,
//     pub return_type: Rc<Expr>,
//     pub return_val: Rc<Expr>,
//     pub original: Option<Rc<cst::Fun>>,
// }

// #[derive(Debug, Clone)]
// pub struct App {
//     pub callee: Box<Expr>,
//     pub args: Rc<Hashed<Box<[Expr]>>>,
//     pub original: Option<Rc<cst::App>>,
// }

// #[derive(Debug, Clone)]
// pub struct For {
//     pub param_types: Rc<Hashed<Box<[Expr]>>>,
//     pub return_type: Rc<Expr>,
//     pub original: Option<Rc<cst::For>>,
// }

impl Evaluator {
    pub fn eval(&mut self, expr: Expr) -> Result<NormalForm, EvalError> {
        if let Some(result) = self.eval_expr_cache.get(&expr.digest()) {
            result.clone()
        } else {
            self.eval_unseen_expr(expr)
        }
    }

    fn eval_unseen_expr(&mut self, expr: Expr) -> Result<NormalForm, EvalError> {
        match expr {
            Expr::Ind(e) => self.eval_unseen_ind(e),
            Expr::Vcon(e) => self.eval_unseen_vcon(e),
            Expr::Match(e) => self.eval_unseen_match(e),
            Expr::Fun(e) => self.eval_unseen_fun(e),
            Expr::App(e) => self.eval_unseen_app(e),
            Expr::For(e) => self.eval_unseen_for(e),

            Expr::Deb(_) | Expr::Universe(_) => Ok(Normalized(expr)),
        }
    }

    fn eval_unseen_ind(&mut self, ind: Rc<Hashed<Ind>>) -> Result<NormalForm, EvalError> {
        let ind_digest = ind.digest.clone();
        let ind = &ind.value;
        let normalized = Ind {
            name: ind.name.clone(),
            universe_level: ind.universe_level,
            index_types: self
                .eval_expressions(ind.index_types.clone())
                .map(Normalized::into_raw)?,
            constructor_defs: self
                .eval_vcon_defs(ind.constructor_defs.clone())
                .map(Normalized::into_raw)?,
            original: None,
        };

        let result = Ok(Normalized(Expr::Ind(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(ind_digest, result.clone());
        result
    }

    fn eval_expressions(&mut self, exprs: RcExprs) -> Result<Normalized<RcExprs>, EvalError> {
        if let Some(result) = self.eval_exprs_cache.get(&exprs.digest) {
            result.clone()
        } else {
            self.eval_unseen_expressions(exprs)
        }
    }

    fn eval_unseen_expressions(
        &mut self,
        exprs: RcExprs,
    ) -> Result<Normalized<RcExprs>, EvalError> {
        let exprs_digest = exprs.digest.clone();
        let exprs = &exprs.value;
        let normalized: Vec<Expr> = exprs
            .iter()
            .map(|expr| self.eval(expr.clone()).map(Normalized::into_raw))
            .collect::<Result<Vec<_>, _>>()?;

        let result = Ok(Normalized(Rc::new(Hashed::new(
            normalized.into_boxed_slice(),
        ))));
        self.eval_exprs_cache.insert(exprs_digest, result.clone());
        result
    }

    fn eval_vcon_defs(&mut self, defs: RcVconDefs) -> Result<Normalized<RcVconDefs>, EvalError> {
        if let Some(result) = self.eval_vcon_defs_cache.get(&defs.digest) {
            result.clone()
        } else {
            self.eval_unseen_vcon_defs(defs)
        }
    }

    fn eval_unseen_vcon_defs(
        &mut self,
        defs: RcVconDefs,
    ) -> Result<Normalized<RcVconDefs>, EvalError> {
        let defs_digest = defs.digest.clone();
        let defs = &defs.value;
        let normalized: Vec<RcVconDef> = defs
            .iter()
            .map(|def| self.eval_vcon_def(def.clone()).map(Normalized::into_raw))
            .collect::<Result<Vec<_>, _>>()?;

        let result = Ok(Normalized(Rc::new(Hashed::new(
            normalized.into_boxed_slice(),
        ))));
        self.eval_vcon_defs_cache
            .insert(defs_digest, result.clone());
        result
    }

    fn eval_vcon_def(&mut self, def: RcVconDef) -> Result<Normalized<RcVconDef>, EvalError> {
        if let Some(result) = self.eval_vcon_def_cache.get(&def.digest) {
            result.clone()
        } else {
            self.eval_unseen_vcon_def(def)
        }
    }

    fn eval_unseen_vcon_def(&mut self, def: RcVconDef) -> Result<Normalized<RcVconDef>, EvalError> {
        let def_digest = def.digest.clone();
        let def = &def.value;
        let normalized = VariantConstructorDef {
            param_types: self.eval_expressions(def.param_types.clone())?.into_raw(),
            index_args: self.eval_expressions(def.index_args.clone())?.into_raw(),
            original: None,
        };

        let result = Ok(Normalized(Rc::new(Hashed::new(normalized))));
        self.eval_vcon_def_cache.insert(def_digest, result.clone());
        result
    }

    fn eval_unseen_vcon(&mut self, vcon: Rc<Hashed<Vcon>>) -> Result<NormalForm, EvalError> {
        let vcon_digest = vcon.digest.clone();
        let vcon = &vcon.value;
        let normalized = Vcon {
            ind: self.eval_ind(vcon.ind.clone())?.into_raw(),
            vcon_index: vcon.vcon_index,
            original: None,
        };

        let result = Ok(Normalized(Expr::Vcon(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(vcon_digest, result.clone());
        result
    }

    fn eval_ind(&mut self, ind: Rc<Hashed<Ind>>) -> Result<Normalized<Rc<Hashed<Ind>>>, EvalError> {
        todo!()
    }

    fn eval_unseen_match(&mut self, m: Rc<Hashed<Match>>) -> Result<NormalForm, EvalError> {
        todo!()
    }

    fn eval_unseen_fun(&mut self, f: Rc<Hashed<Fun>>) -> Result<NormalForm, EvalError> {
        todo!()
    }

    fn eval_unseen_app(&mut self, app: Rc<Hashed<App>>) -> Result<NormalForm, EvalError> {
        todo!()
    }

    fn eval_unseen_for(&mut self, f: Rc<Hashed<For>>) -> Result<NormalForm, EvalError> {
        todo!()
    }
}
