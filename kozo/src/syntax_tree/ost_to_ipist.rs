use crate::{
    syntax_tree::ipist::{self, rc_hashed},
    syntax_tree::ost,
};

// TODO: Rename `cst: ...` params to `ost: ...`.

impl From<ost::Expr> for ipist::Expr {
    fn from(cst: ost::Expr) -> Self {
        match cst {
            ost::Expr::Ind(cst) => ipist::Ind::from(*cst).into(),

            ost::Expr::Vcon(cst) => ipist::Vcon::from(*cst).into(),

            ost::Expr::Match(cst) => ipist::Match::from(*cst).into(),

            ost::Expr::Retype(cst) => ipist::Retype::from(*cst).into(),

            ost::Expr::Fun(cst) => ipist::Fun::from(*cst).into(),

            ost::Expr::App(cst) => ipist::App::from(*cst).into(),

            ost::Expr::For(cst) => ipist::For::from(*cst).into(),

            ost::Expr::Deb(cst) => ipist::Expr::Deb(rc_hashed(cst)),

            ost::Expr::Universe(cst) => ipist::Expr::Universe(rc_hashed(cst)),
        }
    }
}

impl From<ost::Ind> for ipist::Ind {
    fn from(cst: ost::Ind) -> Self {
        ipist::Ind {
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

impl From<ost::ZeroOrMoreExprs> for Vec<ipist::Expr> {
    fn from(cst: ost::ZeroOrMoreExprs) -> Self {
        match cst {
            ost::ZeroOrMoreExprs::Nil => vec![],
            ost::ZeroOrMoreExprs::Snoc(rdc, rac) => {
                let mut rdc: Vec<ipist::Expr> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<ost::ZeroOrMoreVconDefs> for Vec<ipist::VconDef> {
    fn from(cst: ost::ZeroOrMoreVconDefs) -> Self {
        match cst {
            ost::ZeroOrMoreVconDefs::Nil => vec![],
            ost::ZeroOrMoreVconDefs::Snoc(rdc, rac) => {
                let mut rdc: Vec<ipist::VconDef> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<ost::VconDef> for ipist::VconDef {
    fn from(cst: ost::VconDef) -> Self {
        ipist::VconDef {
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

impl From<ost::Vcon> for ipist::Vcon {
    fn from(cst: ost::Vcon) -> Self {
        ipist::Vcon {
            lparen: cst.lparen,
            ind: rc_hashed((*cst.ind).into()),
            vcon_index: cst.vcon_index,
            rparen: cst.rparen,
        }
    }
}

impl From<ost::Match> for ipist::Match {
    fn from(cst: ost::Match) -> Self {
        ipist::Match {
            lparen: cst.lparen,
            matchee: (*cst.matchee).into(),
            econ_extension_len: cst.econ_extension_len,
            return_type: (*cst.return_type).into(),
            cases_lparen: cst.cases_lparen,
            cases: (*cst.cases).into(),
            cases_rparen: cst.cases_rparen,
            rparen: cst.rparen,
        }
    }
}

impl From<ost::ZeroOrMoreMatchCases> for Vec<ipist::MatchCase> {
    fn from(cst: ost::ZeroOrMoreMatchCases) -> Self {
        match cst {
            ost::ZeroOrMoreMatchCases::Nil => vec![],
            ost::ZeroOrMoreMatchCases::Snoc(rdc, rac) => {
                let mut rdc: Vec<ipist::MatchCase> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<ost::MatchCase> for ipist::MatchCase {
    fn from(cst: ost::MatchCase) -> Self {
        match cst {
            ost::MatchCase::Dismissed(kw_index) => ipist::MatchCase::Dismissed(kw_index),
            ost::MatchCase::Nondismissed(cst) => ipist::MatchCase::Nondismissed((*cst).into()),
        }
    }
}

impl From<ost::NondismissedMatchCase> for ipist::NondismissedMatchCase {
    fn from(cst: ost::NondismissedMatchCase) -> Self {
        ipist::NondismissedMatchCase {
            lparen: cst.lparen,
            arity: cst.arity,
            return_val: (*cst.return_val.clone()).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<ost::Retype> for ipist::Retype {
    fn from(cst: ost::Retype) -> Self {
        ipist::Retype {
            lparen: cst.lparen,
            in_term: (*cst.in_term).into(),
            in_type: (*cst.in_type).into(),
            out_type: (*cst.out_type).into(),
            in_rewrites_lparen: cst.in_rewrites_lparen,
            in_rewrites: (*cst.in_rewrites).into(),
            in_rewrites_rparen: cst.in_rewrites_rparen,
            out_rewrites_lparen: cst.out_rewrites_lparen,
            out_rewrites: (*cst.out_rewrites).into(),
            out_rewrites_rparen: cst.out_rewrites_rparen,
            rparen: cst.rparen,
        }
    }
}

impl From<ost::ZeroOrMoreRewrites> for Vec<ipist::RewriteLiteral> {
    fn from(cst: ost::ZeroOrMoreRewrites) -> Self {
        match cst {
            ost::ZeroOrMoreRewrites::Nil => vec![],
            ost::ZeroOrMoreRewrites::Snoc(rdc, rac) => {
                let mut rdc: Vec<_> = (*rdc).into();
                rdc.push(rac);
                rdc
            }
        }
    }
}

impl From<ost::Fun> for ipist::Fun {
    fn from(cst: ost::Fun) -> Self {
        ipist::Fun {
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

impl From<ost::App> for ipist::App {
    fn from(cst: ost::App) -> Self {
        ipist::App {
            lparen: cst.lparen,
            callee: (*cst.callee).into(),
            args: (*cst.args).into(),
            rparen: cst.rparen,
        }
    }
}

impl From<ost::For> for ipist::For {
    fn from(cst: ost::For) -> Self {
        ipist::For {
            lparen: cst.lparen,
            param_types_lparen: cst.param_types_lparen,
            param_types: (*cst.param_types).into(),
            param_types_rparen: cst.param_types_rparen,
            return_type: (*cst.return_type.clone()).into(),
            rparen: cst.rparen,
        }
    }
}
