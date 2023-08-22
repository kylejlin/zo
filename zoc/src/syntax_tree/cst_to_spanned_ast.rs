use crate::syntax_tree::{
    ast::prelude::*,
    ost,
    spanned_ast::{
        self, ForSpans, FunSpans, IndSpans, MatchCaseSpans, MatchSpans, VconDefSpans, VconSpans,
    },
};

impl From<ost::Expr> for spanned_ast::Expr {
    fn from(ost: ost::Expr) -> Self {
        match ost {
            ost::Expr::Ind(ost) => spanned_ast::Ind::from(*ost).into(),

            ost::Expr::Vcon(ost) => spanned_ast::Vcon::from(*ost).into(),

            ost::Expr::Match(ost) => spanned_ast::Match::from(*ost).into(),

            ost::Expr::Fun(ost) => spanned_ast::Fun::from(*ost).into(),

            ost::Expr::App(ost) => spanned_ast::App::from(*ost).into(),

            ost::Expr::For(ost) => spanned_ast::For::from(*ost).into(),

            ost::Expr::Deb(ost) => spanned_ast::DebNode::from(ost).into(),

            ost::Expr::Universe(ost) => spanned_ast::UniverseNode::from(ost).into(),
        }
    }
}

impl From<ost::Ind> for spanned_ast::Ind {
    fn from(ost: ost::Ind) -> Self {
        spanned_ast::Ind {
            universe: Universe {
                level: UniverseLevel(ost.type_.level),
                erasable: ost.type_.erasable,
            },
            name: Rc::new(StringValue(ost.name.value.clone())),
            index_types: rc_hashed((*ost.index_types).into()),
            vcon_defs: rc_hashed((*ost.vcon_defs).into()),
            aux_data: IndSpans {
                span: (ost.lparen, ost.rparen),
                universe_span: ost.type_.span(),
                name_span: ost.name.span,
                index_types_span: (ost.index_types_lparen, ost.index_types_rparen),
                vcon_defs_span: (ost.vcon_defs_lparen, ost.vcon_defs_rparen),
            },
        }
    }
}

impl From<ost::ZeroOrMoreExprs> for Vec<spanned_ast::Expr> {
    fn from(ost: ost::ZeroOrMoreExprs) -> Self {
        match ost {
            ost::ZeroOrMoreExprs::Nil => vec![],
            ost::ZeroOrMoreExprs::Snoc(rdc, rac) => {
                let mut rdc: Vec<spanned_ast::Expr> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<ost::ZeroOrMoreVconDefs> for Vec<spanned_ast::VconDef> {
    fn from(ost: ost::ZeroOrMoreVconDefs) -> Self {
        match ost {
            ost::ZeroOrMoreVconDefs::Nil => vec![],
            ost::ZeroOrMoreVconDefs::Snoc(rdc, rac) => {
                let mut rdc: Vec<spanned_ast::VconDef> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<ost::VconDef> for spanned_ast::VconDef {
    fn from(ost: ost::VconDef) -> Self {
        spanned_ast::VconDef {
            param_types: rc_hashed((*ost.param_types).into()),
            index_args: rc_hashed((*ost.index_args).into()),
            aux_data: VconDefSpans {
                span: (ost.lparen, ost.rparen),
                param_types_span: (ost.param_types_lparen, ost.param_types_rparen),
                index_args_span: (ost.index_args_lparen, ost.index_args_rparen),
            },
        }
    }
}

impl From<ost::Vcon> for spanned_ast::Vcon {
    fn from(ost: ost::Vcon) -> Self {
        spanned_ast::Vcon {
            ind: rc_hashed((*ost.ind).into()),
            vcon_index: ost.vcon_index.value,
            aux_data: VconSpans {
                span: (ost.lparen, ost.rparen),
                vcon_index_span: ost.vcon_index.span,
            },
        }
    }
}

impl From<ost::Match> for spanned_ast::Match {
    fn from(ost: ost::Match) -> Self {
        spanned_ast::Match {
            matchee: (*ost.matchee).into(),
            return_type_arity: ost.return_type_arity.value,
            return_type: (*ost.return_type).into(),
            cases: rc_hashed((*ost.cases).into()),
            aux_data: MatchSpans {
                span: (ost.lparen, ost.rparen),
                return_type_arity_span: ost.return_type_arity.span,
                cases_span: (ost.cases_lparen, ost.cases_rparen),
            },
        }
    }
}

impl From<ost::ZeroOrMoreMatchCases> for Vec<spanned_ast::MatchCase> {
    fn from(ost: ost::ZeroOrMoreMatchCases) -> Self {
        match ost {
            ost::ZeroOrMoreMatchCases::Nil => vec![],
            ost::ZeroOrMoreMatchCases::Snoc(rdc, rac) => {
                let mut rdc: Vec<spanned_ast::MatchCase> = (*rdc).into();
                rdc.push((*rac).into());
                rdc
            }
        }
    }
}

impl From<ost::MatchCase> for spanned_ast::MatchCase {
    fn from(ost: ost::MatchCase) -> Self {
        spanned_ast::MatchCase {
            arity: ost.arity.value,
            return_val: (*ost.return_val.clone()).into(),
            aux_data: MatchCaseSpans {
                span: (ost.lparen, ost.rparen),
                arity_span: ost.arity.span,
            },
        }
    }
}

impl From<ost::Fun> for spanned_ast::Fun {
    fn from(ost: ost::Fun) -> Self {
        spanned_ast::Fun {
            decreasing_index: match *ost.decreasing_index {
                ost::NumberOrNonrecKw::NonrecKw(_) => None,
                ost::NumberOrNonrecKw::Number(n) => Some(n.value),
            },
            param_types: rc_hashed((*ost.param_types).into()),
            return_type: (*ost.return_type.clone()).into(),
            return_val: (*ost.return_val).into(),
            aux_data: FunSpans {
                span: (ost.lparen, ost.rparen),
                decreasing_index_span: match *ost.decreasing_index {
                    ost::NumberOrNonrecKw::NonrecKw(start) => {
                        (start, ByteIndex(start.0 + "nonrec".len()))
                    }
                    ost::NumberOrNonrecKw::Number(n) => n.span,
                },
                param_types_span: (ost.param_types_lparen, ost.param_types_rparen),
            },
        }
    }
}

impl From<ost::App> for spanned_ast::App {
    fn from(ost: ost::App) -> Self {
        spanned_ast::App {
            callee: (*ost.callee).into(),
            args: rc_hashed((*ost.args).into()),
            aux_data: (ost.lparen, ost.rparen),
        }
    }
}

impl From<ost::For> for spanned_ast::For {
    fn from(ost: ost::For) -> Self {
        spanned_ast::For {
            param_types: rc_hashed((*ost.param_types).into()),
            return_type: (*ost.return_type.clone()).into(),
            aux_data: ForSpans {
                span: (ost.lparen, ost.rparen),
                param_types_span: (ost.param_types_lparen, ost.param_types_rparen),
            },
        }
    }
}

impl From<ost::NumberLiteral> for spanned_ast::DebNode {
    fn from(ost: ost::NumberLiteral) -> Self {
        spanned_ast::DebNode {
            deb: Deb(ost.value),
            aux_data: ost.span,
        }
    }
}

impl From<ost::UniverseLiteral> for spanned_ast::UniverseNode {
    fn from(ost: ost::UniverseLiteral) -> Self {
        spanned_ast::UniverseNode {
            universe: Universe {
                level: UniverseLevel(ost.level),
                erasable: ost.erasable,
            },
            aux_data: ost.span(),
        }
    }
}
