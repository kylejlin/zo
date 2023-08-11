use super::*;

impl From<Ind> for Expr {
    fn from(ind: Ind) -> Self {
        Expr::Ind(rc_hashed(ind))
    }
}
impl From<Vcon> for Expr {
    fn from(vcon: Vcon) -> Self {
        Expr::Vcon(rc_hashed(vcon))
    }
}
impl From<Match> for Expr {
    fn from(match_: Match) -> Self {
        Expr::Match(rc_hashed(match_))
    }
}
impl From<Fun> for Expr {
    fn from(fun: Fun) -> Self {
        Expr::Fun(rc_hashed(fun))
    }
}
impl From<App> for Expr {
    fn from(app: App) -> Self {
        Expr::App(rc_hashed(app))
    }
}
impl From<For> for Expr {
    fn from(for_: For) -> Self {
        Expr::For(rc_hashed(for_))
    }
}
impl From<NumberLiteral> for Expr {
    fn from(deb: NumberLiteral) -> Self {
        Expr::Deb(rc_hashed(deb))
    }
}
impl From<UniverseLiteral> for Expr {
    fn from(universe: UniverseLiteral) -> Self {
        Expr::Universe(rc_hashed(universe))
    }
}

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
impl From<RcHashed<NumberLiteral>> for Expr {
    fn from(deb: RcHashed<NumberLiteral>) -> Self {
        Expr::Deb(deb)
    }
}
impl From<RcHashed<UniverseLiteral>> for Expr {
    fn from(universe: RcHashed<UniverseLiteral>) -> Self {
        Expr::Universe(universe)
    }
}
