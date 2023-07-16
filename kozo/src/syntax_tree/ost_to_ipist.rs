use crate::{
    syntax_tree::ipist::{self as rch, rc_hashed},
    syntax_tree::ost,
};

impl From<ost::Expr> for rch::Expr {
    fn from(cst: ost::Expr) -> Self {
        match cst {
            ost::Expr::Ind(cst) => rch::Ind::from(*cst).into(),

            ost::Expr::Vcon(cst) => rch::Vcon::from(*cst).into(),

            ost::Expr::Match(cst) => rch::Match::from(*cst).into(),

            ost::Expr::Fun(cst) => rch::Fun::from(*cst).into(),

            ost::Expr::App(cst) => rch::App::from(*cst).into(),

            ost::Expr::For(cst) => rch::For::from(*cst).into(),

            ost::Expr::Deb(cst) => rch::Expr::Deb(rc_hashed(cst)),

            ost::Expr::Universe(cst) => rch::Expr::Universe(rc_hashed(cst)),
        }
    }
}

impl From<ost::Ind> for rch::Ind {
    fn from(cst: ost::Ind) -> Self {
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

impl From<ost::ZeroOrMoreExprs> for rch::ZeroOrMoreExprs {
    fn from(cst: ost::ZeroOrMoreExprs) -> Self {
        match cst {
            ost::ZeroOrMoreExprs::Nil => rch::ZeroOrMoreExprs::Nil,
            ost::ZeroOrMoreExprs::Snoc(exprs, expr) => {
                rch::ZeroOrMoreExprs::Snoc(Box::new((*exprs).into()), (*expr).into())
            }
        }
    }
}

impl From<ost::ZeroOrMoreVconDefs> for rch::ZeroOrMoreVconDefs {
    fn from(cst: ost::ZeroOrMoreVconDefs) -> Self {
        match cst {
            ost::ZeroOrMoreVconDefs::Nil => rch::ZeroOrMoreVconDefs::Nil,
            ost::ZeroOrMoreVconDefs::Snoc(vcon_defs, vcon_def) => {
                rch::ZeroOrMoreVconDefs::Snoc(Box::new((*vcon_defs).into()), (*vcon_def).into())
            }
        }
    }
}

impl From<ost::VconDef> for rch::VconDef {
    fn from(cst: ost::VconDef) -> Self {
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

impl From<ost::Vcon> for rch::Vcon {
    fn from(cst: ost::Vcon) -> Self {
        rch::Vcon {
            lparen: cst.lparen,
            ind: rc_hashed((*cst.ind).into()),
            vcon_index: cst.vcon_index,
            rparen: cst.rparen,
        }
    }
}

impl From<ost::Match> for rch::Match {
    fn from(cst: ost::Match) -> Self {
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

impl From<ost::ZeroOrMoreMatchCases> for rch::ZeroOrMoreMatchCases {
    fn from(cst: ost::ZeroOrMoreMatchCases) -> Self {
        match cst {
            ost::ZeroOrMoreMatchCases::Nil => rch::ZeroOrMoreMatchCases::Nil,
            ost::ZeroOrMoreMatchCases::Snoc(match_cases, match_case) => {
                rch::ZeroOrMoreMatchCases::Snoc(
                    Box::new((*match_cases).into()),
                    (*match_case).into(),
                )
            }
        }
    }
}

impl From<ost::MatchCase> for rch::MatchCase {
    fn from(cst: ost::MatchCase) -> Self {
        rch::MatchCase {
            lparen: cst.lparen,
            arity: cst.arity,
            return_val: (*cst.return_val.clone()).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<ost::Fun> for rch::Fun {
    fn from(cst: ost::Fun) -> Self {
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

impl From<ost::App> for rch::App {
    fn from(cst: ost::App) -> Self {
        rch::App {
            lparen: cst.lparen,
            callee: (*cst.callee).into(),
            args: (*cst.args).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<ost::For> for rch::For {
    fn from(cst: ost::For) -> Self {
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
