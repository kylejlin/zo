use crate::{
    cst,
    rch_cst::{self as rch, rc_hashed},
};

impl From<cst::Expr> for rch::Expr {
    fn from(cst: cst::Expr) -> Self {
        match cst {
            cst::Expr::Ind(cst) => rch::Ind::from(*cst).into(),

            cst::Expr::Vcon(cst) => rch::Vcon::from(*cst).into(),

            cst::Expr::Match(cst) => rch::Match::from(*cst).into(),

            cst::Expr::Fun(cst) => rch::Fun::from(*cst).into(),

            cst::Expr::App(cst) => rch::App::from(*cst).into(),

            cst::Expr::For(cst) => rch::For::from(*cst).into(),

            cst::Expr::Deb(cst) => rch::Expr::Deb(rc_hashed(cst)),

            cst::Expr::Universe(cst) => rch::Expr::Universe(rc_hashed(cst)),
        }
    }
}

impl From<cst::Ind> for rch::Ind {
    fn from(cst: cst::Ind) -> Self {
        rch::Ind {
            lparen: cst.lparen,
            type_: cst.type_,
            name: cst.name,
            index_types_lparen: cst.index_types_lparen,
            index_types: (*cst.index_types).into(),
            index_types_rparen: cst.index_types_rparen,
            vcon_defs_lparen: cst.vcon_defs_lparen,
            vcon_defs: (*cst.vcon_defs).into(),
            vcon_defs_rparen: cst.vcon_defs_rparen,
            rparen: cst.rparen,
        }
    }
}

impl From<cst::ZeroOrMoreExprs> for rch::ZeroOrMoreExprs {
    fn from(cst: cst::ZeroOrMoreExprs) -> Self {
        match cst {
            cst::ZeroOrMoreExprs::Nil => rch::ZeroOrMoreExprs::Nil,
            cst::ZeroOrMoreExprs::Cons(exprs, expr) => {
                rch::ZeroOrMoreExprs::Cons(Box::new((*exprs).into()), (*expr).into())
            }
        }
    }
}

impl From<cst::ZeroOrMoreVconDefs> for rch::ZeroOrMoreVconDefs {
    fn from(cst: cst::ZeroOrMoreVconDefs) -> Self {
        match cst {
            cst::ZeroOrMoreVconDefs::Nil => rch::ZeroOrMoreVconDefs::Nil,
            cst::ZeroOrMoreVconDefs::Cons(vcon_defs, vcon_def) => {
                rch::ZeroOrMoreVconDefs::Cons(Box::new((*vcon_defs).into()), (*vcon_def).into())
            }
        }
    }
}

impl From<cst::VconDef> for rch::VconDef {
    fn from(cst: cst::VconDef) -> Self {
        rch::VconDef {
            lparen: cst.lparen,
            param_types_lparen: cst.param_types_lparen,
            param_types: (*cst.param_types).into(),
            param_types_rparen: cst.param_types_rparen,
            index_args_lparen: cst.index_args_lparen,
            index_args: (*cst.index_args).into(),
            index_args_rparen: cst.index_args_rparen,
            rparen: cst.rparen,
        }
    }
}

impl From<cst::Vcon> for rch::Vcon {
    fn from(cst: cst::Vcon) -> Self {
        rch::Vcon {
            lparen: cst.lparen,
            ind: rc_hashed((*cst.ind).into()),
            vcon_index: cst.vcon_index,
            rparen: cst.rparen,
        }
    }
}

impl From<cst::Match> for rch::Match {
    fn from(cst: cst::Match) -> Self {
        rch::Match {
            lparen: cst.lparen,
            matchee: (*cst.matchee).into(),
            return_type: (*cst.return_type).into(),
            cases_lparen: cst.cases_lparen,
            cases: (*cst.cases).into(),
            cases_rparen: cst.cases_rparen,
            rparen: cst.rparen,
        }
    }
}

impl From<cst::ZeroOrMoreMatchCases> for rch::ZeroOrMoreMatchCases {
    fn from(cst: cst::ZeroOrMoreMatchCases) -> Self {
        match cst {
            cst::ZeroOrMoreMatchCases::Nil => rch::ZeroOrMoreMatchCases::Nil,
            cst::ZeroOrMoreMatchCases::Cons(match_cases, match_case) => {
                rch::ZeroOrMoreMatchCases::Cons(
                    Box::new((*match_cases).into()),
                    (*match_case).into(),
                )
            }
        }
    }
}

impl From<cst::MatchCase> for rch::MatchCase {
    fn from(cst: cst::MatchCase) -> Self {
        rch::MatchCase {
            lparen: cst.lparen,
            arity: cst.arity,
            return_val: (*cst.return_val.clone()).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<cst::Fun> for rch::Fun {
    fn from(cst: cst::Fun) -> Self {
        rch::Fun {
            lparen: cst.lparen,
            decreasing_index: *cst.decreasing_index,
            param_types_lparen: cst.param_types_lparen,
            param_types: (*cst.param_types).into(),
            param_types_rparen: cst.param_types_rparen,
            return_type: (*cst.return_type.clone()).into(),
            return_val: (*cst.return_val).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<cst::App> for rch::App {
    fn from(cst: cst::App) -> Self {
        rch::App {
            lparen: cst.lparen,
            callee: (*cst.callee).into(),
            args: (*cst.args).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<cst::For> for rch::For {
    fn from(cst: cst::For) -> Self {
        rch::For {
            lparen: cst.lparen,
            param_types_lparen: cst.param_types_lparen,
            param_types: (*cst.param_types).into(),
            param_types_rparen: cst.param_types_rparen,
            return_type: (*cst.return_type.clone()).into(),
            rparen: cst.rparen,
        }
    }
}
