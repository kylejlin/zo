use crate::{
    syntax_tree::ipist::{self, rc_hashed},
    syntax_tree::ost,
};

impl From<ost::Expr> for ipist::Expr {
    fn from(ost: ost::Expr) -> Self {
        match ost {
            ost::Expr::Ind(ost) => ipist::Ind::from(*ost).into(),

            ost::Expr::Vcon(ost) => ipist::Vcon::from(*ost).into(),

            ost::Expr::Match(ost) => ipist::Match::from(*ost).into(),

            ost::Expr::Retype(ost) => ipist::Retype::from(*ost).into(),

            ost::Expr::Fun(ost) => ipist::Fun::from(*ost).into(),

            ost::Expr::App(ost) => ipist::App::from(*ost).into(),

            ost::Expr::For(ost) => ipist::For::from(*ost).into(),

            ost::Expr::Deb(ost) => ipist::Expr::Deb(rc_hashed(ost)),

            ost::Expr::Universe(ost) => ipist::Expr::Universe(rc_hashed(ost)),
        }
    }
}

impl From<ost::Ind> for ipist::Ind {
    fn from(ost: ost::Ind) -> Self {
        ipist::Ind {
            lparen: ost.lparen,
            type_: ost.type_,
            name: ost.name,
            index_types_lparen: ost.index_types_lparen,
            index_types: (*ost.index_types).into(),
            index_types_rparen: ost.index_types_rparen,
            vcon_defs_lparen: ost.vcon_defs_lparen,
            vcon_defs: (*ost.vcon_defs).into(),
            vcon_defs_rparen: ost.vcon_defs_rparen,
            rparen: ost.rparen,
        }
    }
}

impl From<ost::ZeroOrMoreExprs> for Vec<ipist::Expr> {
    fn from(ost: ost::ZeroOrMoreExprs) -> Self {
        match ost {
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
    fn from(ost: ost::ZeroOrMoreVconDefs) -> Self {
        match ost {
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
    fn from(ost: ost::VconDef) -> Self {
        ipist::VconDef {
            lparen: ost.lparen,
            param_types_lparen: ost.param_types_lparen,
            param_types: (*ost.param_types).into(),
            param_types_rparen: ost.param_types_rparen,
            index_args_lparen: ost.index_args_lparen,
            index_args: (*ost.index_args).into(),
            index_args_rparen: ost.index_args_rparen,
            rparen: ost.rparen,
        }
    }
}

impl From<ost::Vcon> for ipist::Vcon {
    fn from(ost: ost::Vcon) -> Self {
        ipist::Vcon {
            lparen: ost.lparen,
            ind: rc_hashed((*ost.ind).into()),
            vcon_index: ost.vcon_index,
            rparen: ost.rparen,
        }
    }
}

impl From<ost::Match> for ipist::Match {
    fn from(ost: ost::Match) -> Self {
        ipist::Match {
            lparen: ost.lparen,
            matchee: (*ost.matchee).into(),
            econ_extension_len: ost.econ_extension_len,
            return_type: (*ost.return_type).into(),
            cases_lparen: ost.cases_lparen,
            cases: (*ost.cases).into(),
            cases_rparen: ost.cases_rparen,
            rparen: ost.rparen,
        }
    }
}

impl From<ost::ZeroOrMoreMatchCases> for Vec<ipist::MatchCase> {
    fn from(ost: ost::ZeroOrMoreMatchCases) -> Self {
        match ost {
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
    fn from(ost: ost::MatchCase) -> Self {
        match ost {
            ost::MatchCase::Dismissed(kw_index) => ipist::MatchCase::Dismissed(kw_index),
            ost::MatchCase::Nondismissed(ost) => ipist::MatchCase::Nondismissed((*ost).into()),
        }
    }
}

impl From<ost::NondismissedMatchCase> for ipist::NondismissedMatchCase {
    fn from(ost: ost::NondismissedMatchCase) -> Self {
        ipist::NondismissedMatchCase {
            lparen: ost.lparen,
            arity: ost.arity,
            return_val: (*ost.return_val.clone()).into(),
            rparen: ost.rparen,
        }
    }
}

impl From<ost::Retype> for ipist::Retype {
    fn from(ost: ost::Retype) -> Self {
        ipist::Retype {
            lparen: ost.lparen,
            in_term: (*ost.in_term).into(),
            in_type: (*ost.in_type).into(),
            out_type: (*ost.out_type).into(),
            in_rewrites_lparen: ost.in_rewrites_lparen,
            in_rewrites: (*ost.in_rewrites).into(),
            in_rewrites_rparen: ost.in_rewrites_rparen,
            out_rewrites_lparen: ost.out_rewrites_lparen,
            out_rewrites: (*ost.out_rewrites).into(),
            out_rewrites_rparen: ost.out_rewrites_rparen,
            rparen: ost.rparen,
        }
    }
}

impl From<ost::ZeroOrMoreRewrites> for Vec<ipist::RewriteLiteral> {
    fn from(ost: ost::ZeroOrMoreRewrites) -> Self {
        match ost {
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
    fn from(ost: ost::Fun) -> Self {
        ipist::Fun {
            lparen: ost.lparen,
            decreasing_index: *ost.decreasing_index,
            param_types_lparen: ost.param_types_lparen,
            param_types: (*ost.param_types).into(),
            param_types_rparen: ost.param_types_rparen,
            return_type: (*ost.return_type.clone()).into(),
            return_val: (*ost.return_val).into(),
            rparen: ost.rparen,
        }
    }
}

impl From<ost::App> for ipist::App {
    fn from(ost: ost::App) -> Self {
        ipist::App {
            lparen: ost.lparen,
            callee: (*ost.callee).into(),
            args: (*ost.args).into(),
            rparen: ost.rparen,
        }
    }
}

impl From<ost::For> for ipist::For {
    fn from(ost: ost::For) -> Self {
        ipist::For {
            lparen: ost.lparen,
            param_types_lparen: ost.param_types_lparen,
            param_types: (*ost.param_types).into(),
            param_types_rparen: ost.param_types_rparen,
            return_type: (*ost.return_type.clone()).into(),
            rparen: ost.rparen,
        }
    }
}
