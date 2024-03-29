use super::*;

impl<A> Hash for Expr<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
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

impl<A> Hash for Ind<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_IND.hash(state);
        self.name.hash(state);
        self.universe.hash(state);
        self.index_types.digest.hash(state);
        self.vcon_defs.digest.hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for VconDef<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_VCON_DEF.hash(state);
        self.param_types.digest.hash(state);
        self.index_args.digest.hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for Vcon<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_VCON.hash(state);
        self.ind.digest.hash(state);
        self.vcon_index.hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for Match<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_MATCH.hash(state);
        self.matchee.digest().hash(state);
        self.return_type.digest().hash(state);
        self.cases.digest.hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for MatchCase<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_MATCH_CASE.hash(state);
        self.arity.hash(state);
        self.return_val.digest().hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for Fun<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_FUN.hash(state);
        self.decreasing_index.hash(state);
        self.param_types.digest.hash(state);
        self.return_type.digest().hash(state);
        self.return_val.digest().hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for App<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_APP.hash(state);
        self.callee.digest().hash(state);
        self.args.digest.hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for For<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_FOR.hash(state);
        self.param_types.digest.hash(state);
        self.return_type.digest().hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for DebNode<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_DEB.hash(state);
        self.deb.hash(state);
        delimiters::END.hash(state);
    }
}

impl<A> Hash for UniverseNode<A>
where
    A: AuxDataFamily,
    A::Ind: Hash,
    A::VconDef: Hash,
    A::Vcon: Hash,
    A::Match: Hash,
    A::MatchCase: Hash,
    A::Fun: Hash,
    A::App: Hash,
    A::For: Hash,
    A::Deb: Hash,
    A::Universe: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        delimiters::START_UNIVERSE.hash(state);
        self.universe.hash(state);
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
    pub const START_DEB: u8 = 8;
    pub const START_UNIVERSE: u8 = 9;

    pub const START_VCON_DEF: u8 = 10;
    pub const START_MATCH_CASE: u8 = 11;
}
