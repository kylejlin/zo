use super::*;

impl<A: AuxDataFamily> From<RcHashed<Ind<A>>> for Expr<A> {
    fn from(ind: RcHashed<Ind<A>>) -> Self {
        Expr::Ind(ind)
    }
}
impl<A: AuxDataFamily> From<RcHashed<Vcon<A>>> for Expr<A> {
    fn from(vcon: RcHashed<Vcon<A>>) -> Self {
        Expr::Vcon(vcon)
    }
}
impl<A: AuxDataFamily> From<RcHashed<Match<A>>> for Expr<A> {
    fn from(match_: RcHashed<Match<A>>) -> Self {
        Expr::Match(match_)
    }
}
impl<A: AuxDataFamily> From<RcHashed<Fun<A>>> for Expr<A> {
    fn from(fun: RcHashed<Fun<A>>) -> Self {
        Expr::Fun(fun)
    }
}
impl<A: AuxDataFamily> From<RcHashed<App<A>>> for Expr<A> {
    fn from(app: RcHashed<App<A>>) -> Self {
        Expr::App(app)
    }
}
impl<A: AuxDataFamily> From<RcHashed<For<A>>> for Expr<A> {
    fn from(for_: RcHashed<For<A>>) -> Self {
        Expr::For(for_)
    }
}
impl<A: AuxDataFamily> From<RcHashed<DebNode<A>>> for Expr<A> {
    fn from(deb: RcHashed<DebNode<A>>) -> Self {
        Expr::Deb(deb)
    }
}
impl<A: AuxDataFamily> From<RcHashed<UniverseNode<A>>> for Expr<A> {
    fn from(universe: RcHashed<UniverseNode<A>>) -> Self {
        Expr::Universe(universe)
    }
}

impl<A: AuxDataFamily> From<Ind<A>> for Expr<A> {
    fn from(ind: Ind<A>) -> Self {
        rc_hashed(ind).into()
    }
}
impl<A: AuxDataFamily> From<Vcon<A>> for Expr<A> {
    fn from(vcon: Vcon<A>) -> Self {
        rc_hashed(vcon).into()
    }
}
impl<A: AuxDataFamily> From<Match<A>> for Expr<A> {
    fn from(match_: Match<A>) -> Self {
        rc_hashed(match_).into()
    }
}
impl<A: AuxDataFamily> From<Fun<A>> for Expr<A> {
    fn from(fun: Fun<A>) -> Self {
        rc_hashed(fun).into()
    }
}
impl<A: AuxDataFamily> From<App<A>> for Expr<A> {
    fn from(app: App<A>) -> Self {
        rc_hashed(app).into()
    }
}
impl<A: AuxDataFamily> From<For<A>> for Expr<A> {
    fn from(for_: For<A>) -> Self {
        rc_hashed(for_).into()
    }
}
impl<A: AuxDataFamily> From<DebNode<A>> for Expr<A> {
    fn from(deb: DebNode<A>) -> Self {
        rc_hashed(deb).into()
    }
}
impl<A: AuxDataFamily> From<UniverseNode<A>> for Expr<A> {
    fn from(universe: UniverseNode<A>) -> Self {
        rc_hashed(universe).into()
    }
}

impl<A: AuxDataFamily> Expr<A> {
    pub fn try_into_ind(self) -> Result<RcHashed<Ind<A>>, Self> {
        match self {
            Expr::Ind(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_vcon(self) -> Result<RcHashed<Vcon<A>>, Self> {
        match self {
            Expr::Vcon(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_match(self) -> Result<RcHashed<Match<A>>, Self> {
        match self {
            Expr::Match(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_fun(self) -> Result<RcHashed<Fun<A>>, Self> {
        match self {
            Expr::Fun(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_app(self) -> Result<RcHashed<App<A>>, Self> {
        match self {
            Expr::App(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_for(self) -> Result<RcHashed<For<A>>, Self> {
        match self {
            Expr::For(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_deb(self) -> Result<RcHashed<DebNode<A>>, Self> {
        match self {
            Expr::Deb(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<RcHashed<UniverseNode<A>>, Self> {
        match self {
            Expr::Universe(e) => Ok(e),
            _ => Err(self),
        }
    }
}
