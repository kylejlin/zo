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
