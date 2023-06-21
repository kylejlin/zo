use crate::{
    ast::{self, Hashed},
    cst,
};

use std::rc::Rc;

impl From<cst::Expr> for ast::Expr {
    fn from(cst: cst::Expr) -> Self {
        match cst {
            cst::Expr::Ind(cst) => ast::Expr::Ind(Rc::new(Hashed::new((*cst).into()))),

            cst::Expr::Vcon(cst) => ast::Expr::Vcon(Rc::new(Hashed::new((*cst).into()))),

            cst::Expr::Match(cst) => ast::Expr::Match(Rc::new(Hashed::new((*cst).into()))),

            cst::Expr::Fun(cst) => ast::Expr::Fun(Rc::new(Hashed::new((*cst).into()))),

            cst::Expr::App(cst) => ast::Expr::App(Rc::new(Hashed::new((*cst).into()))),

            cst::Expr::For(cst) => ast::Expr::For(Rc::new(Hashed::new((*cst).into()))),

            cst::Expr::Deb(cst) => ast::Expr::Deb(Rc::new(Hashed::new(cst))),

            cst::Expr::Universe(cst) => ast::Expr::Universe(Rc::new(Hashed::new(cst))),
        }
    }
}

impl From<cst::Ind> for ast::Ind {
    fn from(cst: cst::Ind) -> Self {
        ast::Ind {
            name: Rc::new(Hashed::new(cst.name.clone())),
            universe_level: cst.type_.level.clone(),
            index_types: Vec::from(*cst.index_types.clone())
                .into_boxed_slice()
                .into(),
            constructor_defs: Vec::from(*cst.constructor_defs.clone())
                .into_boxed_slice()
                .into(),
            original: Some(Rc::new(cst)),
        }
    }
}

impl From<cst::ZeroOrMoreExprs> for Vec<ast::Expr> {
    fn from(cst: cst::ZeroOrMoreExprs) -> Self {
        match cst {
            cst::ZeroOrMoreExprs::Nil => vec![],
            cst::ZeroOrMoreExprs::Cons(exprs, expr) => {
                let mut exprs: Self = (*exprs).into();
                exprs.push((*expr).into());
                exprs
            }
        }
    }
}

impl From<cst::ZeroOrMoreVariantConstructorDefs> for Vec<ast::VariantConstructorDef> {
    fn from(cst: cst::ZeroOrMoreVariantConstructorDefs) -> Self {
        match cst {
            cst::ZeroOrMoreVariantConstructorDefs::Nil => vec![],
            cst::ZeroOrMoreVariantConstructorDefs::Cons(defs, def) => {
                let mut variant_constructor_defs: Self = (*defs).into();
                variant_constructor_defs.push((*def).into());
                variant_constructor_defs
            }
        }
    }
}

impl From<cst::VariantConstructorDef> for ast::VariantConstructorDef {
    fn from(cst: cst::VariantConstructorDef) -> Self {
        ast::VariantConstructorDef {
            param_types: Vec::from(*cst.param_types.clone())
                .into_boxed_slice()
                .into(),
            index_args: Vec::from(*cst.index_args.clone()).into_boxed_slice().into(),
            original: Some(Rc::new(cst)),
        }
    }
}

impl From<cst::Vcon> for ast::Vcon {
    fn from(cst: cst::Vcon) -> Self {
        ast::Vcon {
            ind: Rc::new(Hashed::new((*cst.ind.clone()).into())),
            vcon_index: cst.vcon_index.value,
            original: Some(Rc::new(cst)),
        }
    }
}

impl From<cst::Match> for ast::Match {
    fn from(cst: cst::Match) -> Self {
        ast::Match {
            matchee: Rc::new((*cst.matchee.clone()).into()),
            return_type: Rc::new((*cst.return_type.clone()).into()),
            cases: Vec::from(*cst.cases.clone()).into_boxed_slice().into(),
            original: Some(Rc::new(cst)),
        }
    }
}

impl From<cst::Fun> for ast::Fun {
    fn from(cst: cst::Fun) -> Self {
        ast::Fun {
            decreasing_index: match &*cst.decreasing_index {
                cst::NumberOrNonrecKw::Number(numlit) => Some(numlit.value),
                cst::NumberOrNonrecKw::NonrecKw(_) => None,
            },
            param_types: Vec::from(*cst.param_types.clone())
                .into_boxed_slice()
                .into(),
            return_type: Rc::new((*cst.return_type.clone()).into()),
            return_val: Rc::new((*cst.return_val.clone()).into()),
            original: Some(Rc::new(cst)),
        }
    }
}

impl From<cst::App> for ast::App {
    fn from(cst: cst::App) -> Self {
        ast::App {
            callee: Box::new((*cst.callee.clone()).into()),
            args: Vec::from(*cst.args.clone()).into_boxed_slice().into(),
            original: Some(Rc::new(cst)),
        }
    }
}

impl From<cst::For> for ast::For {
    fn from(cst: cst::For) -> Self {
        ast::For {
            param_types: Vec::from(*cst.param_types.clone())
                .into_boxed_slice()
                .into(),
            return_type: Rc::new((*cst.return_type.clone()).into()),
            original: Some(Rc::new(cst)),
        }
    }
}
