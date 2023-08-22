use crate::syntax_tree::{
    ast::prelude::*,
    cst,
    spanned_ast::{
        self, ForSpans, FunSpans, IndSpans, MatchCaseSpans, MatchSpans, VconDefSpans, VconSpans,
    },
};

impl From<cst::Expr> for spanned_ast::Expr {
    fn from(cst: cst::Expr) -> Self {
        match cst {
            cst::Expr::Ind(cst) => spanned_ast::Ind::from(*cst).into(),

            cst::Expr::Vcon(cst) => spanned_ast::Vcon::from(*cst).into(),

            cst::Expr::Match(cst) => spanned_ast::Match::from(*cst).into(),

            cst::Expr::Fun(cst) => spanned_ast::Fun::from(*cst).into(),

            cst::Expr::App(cst) => spanned_ast::App::from(*cst).into(),

            cst::Expr::For(cst) => spanned_ast::For::from(*cst).into(),

            cst::Expr::Deb(cst) => spanned_ast::DebNode::from(cst).into(),

            cst::Expr::Universe(cst) => spanned_ast::UniverseNode::from(cst).into(),
        }
    }
}

impl From<cst::Ind> for spanned_ast::Ind {
    fn from(cst: cst::Ind) -> Self {
        spanned_ast::Ind {
            universe: Universe {
                level: UniverseLevel(cst.type_.level),
                erasable: cst.type_.erasable,
            },
            name: Rc::new(StringValue(cst.name.value.clone())),
            index_types: rc_hashed((*cst.index_types).into()),
            vcon_defs: rc_hashed((*cst.vcon_defs).into()),
            aux_data: IndSpans {
                span: (cst.lparen, cst.rparen),
                universe_span: cst.type_.span,
                name_span: cst.name.span,
                index_types_span: (cst.index_types_lparen, cst.index_types_rparen),
                vcon_defs_span: (cst.vcon_defs_lparen, cst.vcon_defs_rparen),
            },
        }
    }
}

impl From<cst::ZeroOrMoreExprs> for Vec<spanned_ast::Expr> {
    fn from(cst: cst::ZeroOrMoreExprs) -> Self {
        match cst {
            cst::ZeroOrMoreExprs::Nil => vec![],
            cst::ZeroOrMoreExprs::Snoc(rdc, rac) => {
                let mut rdc: Vec<spanned_ast::Expr> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<cst::ZeroOrMoreVconDefs> for Vec<spanned_ast::VconDef> {
    fn from(cst: cst::ZeroOrMoreVconDefs) -> Self {
        match cst {
            cst::ZeroOrMoreVconDefs::Nil => vec![],
            cst::ZeroOrMoreVconDefs::Snoc(rdc, rac) => {
                let mut rdc: Vec<spanned_ast::VconDef> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<cst::VconDef> for spanned_ast::VconDef {
    fn from(cst: cst::VconDef) -> Self {
        spanned_ast::VconDef {
            param_types: rc_hashed((*cst.param_types).into()),
            index_args: rc_hashed((*cst.index_args).into()),
            aux_data: VconDefSpans {
                span: (cst.lparen, cst.rparen),
                param_types_span: (cst.param_types_lparen, cst.param_types_rparen),
                index_args_span: (cst.index_args_lparen, cst.index_args_rparen),
            },
        }
    }
}

impl From<cst::Vcon> for spanned_ast::Vcon {
    fn from(cst: cst::Vcon) -> Self {
        spanned_ast::Vcon {
            ind: rc_hashed((*cst.ind).into()),
            vcon_index: cst.vcon_index.value,
            aux_data: VconSpans {
                span: (cst.lparen, cst.rparen),
                vcon_index_span: cst.vcon_index.span,
            },
        }
    }
}

impl From<cst::Match> for spanned_ast::Match {
    fn from(cst: cst::Match) -> Self {
        spanned_ast::Match {
            matchee: (*cst.matchee).into(),
            return_type_arity: cst.return_type_arity.value,
            return_type: (*cst.return_type).into(),
            cases: rc_hashed((*cst.cases).into()),
            aux_data: MatchSpans {
                span: (cst.lparen, cst.rparen),
                return_type_arity_span: cst.return_type_arity.span,
                cases_span: (cst.cases_lparen, cst.cases_rparen),
            },
        }
    }
}

impl From<cst::ZeroOrMoreMatchCases> for Vec<spanned_ast::MatchCase> {
    fn from(cst: cst::ZeroOrMoreMatchCases) -> Self {
        match cst {
            cst::ZeroOrMoreMatchCases::Nil => vec![],
            cst::ZeroOrMoreMatchCases::Snoc(rdc, rac) => {
                let mut rdc: Vec<spanned_ast::MatchCase> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<cst::MatchCase> for spanned_ast::MatchCase {
    fn from(cst: cst::MatchCase) -> Self {
        spanned_ast::MatchCase {
            arity: cst.arity.value,
            return_val: (*cst.return_val.clone()).into(),
            aux_data: MatchCaseSpans {
                span: (cst.lparen, cst.rparen),
                arity_span: cst.arity.span,
            },
        }
    }
}

impl From<cst::Fun> for spanned_ast::Fun {
    fn from(cst: cst::Fun) -> Self {
        spanned_ast::Fun {
            decreasing_index: match *cst.decreasing_index {
                cst::NumberOrNonrecKw::NonrecKw(_) => None,
                cst::NumberOrNonrecKw::Number(n) => Some(n.value),
            },
            param_types: rc_hashed((*cst.param_types).into()),
            return_type: (*cst.return_type.clone()).into(),
            return_val: (*cst.return_val).into(),
            aux_data: FunSpans {
                span: (cst.lparen, cst.rparen),
                decreasing_index_span: match *cst.decreasing_index {
                    cst::NumberOrNonrecKw::NonrecKw(start) => {
                        (start, ByteIndex(start.0 + "nonrec".len()))
                    }
                    cst::NumberOrNonrecKw::Number(n) => n.span,
                },
                param_types_span: (cst.param_types_lparen, cst.param_types_rparen),
            },
        }
    }
}

impl From<cst::App> for spanned_ast::App {
    fn from(cst: cst::App) -> Self {
        spanned_ast::App {
            callee: (*cst.callee).into(),
            args: rc_hashed((*cst.args).into()),
            aux_data: (cst.lparen, cst.rparen),
        }
    }
}

impl From<cst::For> for spanned_ast::For {
    fn from(cst: cst::For) -> Self {
        spanned_ast::For {
            param_types: rc_hashed((*cst.param_types).into()),
            return_type: (*cst.return_type.clone()).into(),
            aux_data: ForSpans {
                span: (cst.lparen, cst.rparen),
                param_types_span: (cst.param_types_lparen, cst.param_types_rparen),
            },
        }
    }
}

impl From<cst::NumberLiteral> for spanned_ast::DebNode {
    fn from(cst: cst::NumberLiteral) -> Self {
        spanned_ast::DebNode {
            deb: Deb(cst.value),
            aux_data: cst.span,
        }
    }
}

impl From<cst::UniverseLiteral> for spanned_ast::UniverseNode {
    fn from(cst: cst::UniverseLiteral) -> Self {
        spanned_ast::UniverseNode {
            universe: Universe {
                level: UniverseLevel(cst.level),
                erasable: cst.erasable,
            },
            aux_data: cst.span,
        }
    }
}
