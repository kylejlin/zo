use super::*;

impl From<RcHashed<Ind>> for Expr {
    fn from(ind: RcHashed<Ind>) -> Self {
        Expr::Ind(ind)
    }
}
impl From<RcHashed<Vcon>> for Expr {
    fn from(vcon: RcHashed<Vcon>) -> Self {
        Expr::Vcon(vcon)
    }
}
impl From<RcHashed<Match>> for Expr {
    fn from(match_: RcHashed<Match>) -> Self {
        Expr::Match(match_)
    }
}
impl From<RcHashed<Fun>> for Expr {
    fn from(fun: RcHashed<Fun>) -> Self {
        Expr::Fun(fun)
    }
}
impl From<RcHashed<App>> for Expr {
    fn from(app: RcHashed<App>) -> Self {
        Expr::App(app)
    }
}
impl From<RcHashed<For>> for Expr {
    fn from(for_: RcHashed<For>) -> Self {
        Expr::For(for_)
    }
}
impl From<RcHashed<DebNode>> for Expr {
    fn from(deb: RcHashed<DebNode>) -> Self {
        Expr::Deb(deb)
    }
}
impl From<RcHashed<UniverseNode>> for Expr {
    fn from(universe: RcHashed<UniverseNode>) -> Self {
        Expr::Universe(universe)
    }
}

impl From<Ind> for Expr {
    fn from(ind: Ind) -> Self {
        rc_hashed(ind).into()
    }
}
impl From<Vcon> for Expr {
    fn from(vcon: Vcon) -> Self {
        rc_hashed(vcon).into()
    }
}
impl From<Match> for Expr {
    fn from(match_: Match) -> Self {
        rc_hashed(match_).into()
    }
}
impl From<Fun> for Expr {
    fn from(fun: Fun) -> Self {
        rc_hashed(fun).into()
    }
}
impl From<App> for Expr {
    fn from(app: App) -> Self {
        rc_hashed(app).into()
    }
}
impl From<For> for Expr {
    fn from(for_: For) -> Self {
        rc_hashed(for_).into()
    }
}
impl From<DebNode> for Expr {
    fn from(deb: DebNode) -> Self {
        rc_hashed(deb).into()
    }
}
impl From<UniverseNode> for Expr {
    fn from(universe: UniverseNode) -> Self {
        rc_hashed(universe).into()
    }
}

impl Expr {
    pub fn try_into_ind(self) -> Result<RcHashed<Ind>, Self> {
        match self {
            Expr::Ind(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_vcon(self) -> Result<RcHashed<Vcon>, Self> {
        match self {
            Expr::Vcon(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_match(self) -> Result<RcHashed<Match>, Self> {
        match self {
            Expr::Match(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_fun(self) -> Result<RcHashed<Fun>, Self> {
        match self {
            Expr::Fun(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_app(self) -> Result<RcHashed<App>, Self> {
        match self {
            Expr::App(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_for(self) -> Result<RcHashed<For>, Self> {
        match self {
            Expr::For(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_deb(self) -> Result<RcHashed<DebNode>, Self> {
        match self {
            Expr::Deb(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<RcHashed<UniverseNode>, Self> {
        match self {
            Expr::Universe(e) => Ok(e),
            _ => Err(self),
        }
    }
}
