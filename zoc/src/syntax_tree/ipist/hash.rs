use super::*;

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expr::Ind(e) => e.hash(state),
            Expr::Vcon(e) => e.hash(state),
            Expr::Match(e) => e.hash(state),
            Expr::Fun(e) => e.hash(state),
            Expr::App(e) => e.hash(state),
            Expr::For(e) => e.hash(state),
            Expr::Deb(e) => e.hash(state),
            Expr::Universe(e) => e.hash(state),
        }
    }
}

impl Hash for Ind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_IND.hash(state);
        self.name.hash(state);
        self.type_.hash(state);
        self.index_types.hash(state);
        self.vcon_defs.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for VconDef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_VCON_DEF.hash(state);
        self.param_types.hash(state);
        self.index_args.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for Vcon {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_VCON.hash(state);
        self.ind.digest.hash(state);
        self.vcon_index.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for Match {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_MATCH.hash(state);
        self.matchee.hash(state);
        self.return_type.hash(state);
        self.cases.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for MatchCase {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_MATCH_CASE.hash(state);
        self.arity.hash(state);
        self.return_val.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for Fun {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_FUN.hash(state);
        self.decreasing_index.hash(state);
        self.param_types.hash(state);
        self.return_type.hash(state);
        self.return_val.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for App {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_APP.hash(state);
        self.callee.hash(state);
        self.args.hash(state);
        delimiters::END.hash(state);
    }
}

impl Hash for For {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_FOR.hash(state);
        self.param_types.hash(state);
        self.return_type.hash(state);
        delimiters::END.hash(state);
    }
}

mod delimiters {
    pub const END: u8 = 1;

    pub const START_IND: u8 = 2;
    pub const START_VCON: u8 = 3;
    pub const START_MATCH: u8 = 4;
    pub const START_FUN: u8 = 5;
    pub const START_APP: u8 = 6;
    pub const START_FOR: u8 = 7;
    pub const _START_DEB: u8 = 8;
    pub const _START_UNIVERSE: u8 = 9;

    pub const START_VCON_DEF: u8 = 10;
    pub const START_MATCH_CASE: u8 = 11;
}
