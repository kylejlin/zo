use crate::{
    ast::{self, rc_sem_hashed},
    cst,
};

use std::rc::Rc;

impl From<cst::Expr> for ast::Expr {
    fn from(cst: cst::Expr) -> Self {
        match cst {
            cst::Expr::Ind(cst) => ast::Ind::from(*cst).into(),

            cst::Expr::Vcon(cst) => ast::Vcon::from(*cst).into(),

            cst::Expr::Match(cst) => ast::Match::from(*cst).into(),

            cst::Expr::Fun(cst) => ast::Fun::from(*cst).into(),

            cst::Expr::App(cst) => ast::App::from(*cst).into(),

            cst::Expr::For(cst) => ast::For::from(*cst).into(),

            cst::Expr::Deb(cst) => ast::Expr::Deb(rc_sem_hashed(ast::DebNode {
                deb: ast::Deb(cst.value),
            })),

            cst::Expr::Universe(cst) => ast::Expr::Universe(rc_sem_hashed(ast::UniverseNode {
                level: ast::UniverseLevel(cst.level),
            })),
        }
    }
}

impl From<cst::Ind> for ast::Ind {
    fn from(cst: cst::Ind) -> Self {
        ast::Ind {
            name: Rc::new(ast::StringValue(cst.name.value.clone())),
            universe_level: ast::UniverseLevel(cst.type_.level),
            index_types: rc_sem_hashed(Vec::from(*cst.index_types.clone()).into_boxed_slice()),
            vcon_defs: rc_sem_hashed(Vec::from(*cst.vcon_defs.clone()).into_boxed_slice()),
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

impl From<cst::ZeroOrMoreVconDefs> for Vec<ast::VconDef> {
    fn from(cst: cst::ZeroOrMoreVconDefs) -> Self {
        match cst {
            cst::ZeroOrMoreVconDefs::Nil => vec![],
            cst::ZeroOrMoreVconDefs::Cons(defs, def) => {
                let mut variant_constructor_defs: Self = (*defs).into();
                let def = (*def).into();
                variant_constructor_defs.push(def);
                variant_constructor_defs
            }
        }
    }
}

impl From<cst::VconDef> for ast::VconDef {
    fn from(cst: cst::VconDef) -> Self {
        ast::VconDef {
            param_types: rc_sem_hashed(Vec::from(*cst.param_types.clone()).into_boxed_slice()),
            index_args: rc_sem_hashed(Vec::from(*cst.index_args.clone()).into_boxed_slice()),
        }
    }
}

impl From<cst::Vcon> for ast::Vcon {
    fn from(cst: cst::Vcon) -> Self {
        ast::Vcon {
            ind: rc_sem_hashed((*cst.ind.clone()).into()),
            vcon_index: cst.vcon_index.value,
        }
    }
}

impl From<cst::Match> for ast::Match {
    fn from(cst: cst::Match) -> Self {
        ast::Match {
            matchee: (*cst.matchee.clone()).into(),
            return_type: (*cst.return_type.clone()).into(),
            cases: rc_sem_hashed(Vec::from(*cst.cases.clone()).into_boxed_slice()),
        }
    }
}

impl From<cst::ZeroOrMoreMatchCases> for Vec<ast::MatchCase> {
    fn from(cst: cst::ZeroOrMoreMatchCases) -> Self {
        match cst {
            cst::ZeroOrMoreMatchCases::Nil => vec![],
            cst::ZeroOrMoreMatchCases::Cons(cases, case) => {
                let mut match_cases: Self = (*cases).into();
                let case = (*case).into();
                match_cases.push(case);
                match_cases
            }
        }
    }
}

impl From<cst::MatchCase> for ast::MatchCase {
    fn from(cst: cst::MatchCase) -> Self {
        ast::MatchCase {
            arity: cst.arity.value,
            return_val: (*cst.return_val.clone()).into(),
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
            param_types: rc_sem_hashed(Vec::from(*cst.param_types.clone()).into_boxed_slice()),
            return_type: (*cst.return_type.clone()).into(),
            return_val: (*cst.return_val.clone()).into(),
        }
    }
}

impl From<cst::App> for ast::App {
    fn from(cst: cst::App) -> Self {
        ast::App {
            callee: (*cst.callee.clone()).into(),
            args: rc_sem_hashed(Vec::from(*cst.args.clone()).into_boxed_slice()),
        }
    }
}

impl From<cst::For> for ast::For {
    fn from(cst: cst::For) -> Self {
        ast::For {
            param_types: rc_sem_hashed(Vec::from(*cst.param_types.clone()).into_boxed_slice()),
            return_type: (*cst.return_type.clone()).into(),
        }
    }
}
