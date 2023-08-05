use super::*;

impl GetDigest for Expr {
    fn digest(&self) -> &Digest {
        match self {
            Expr::Ind(e) => &e.digest,
            Expr::Vcon(e) => &e.digest,
            Expr::Match(e) => &e.digest,
            Expr::Fun(e) => &e.digest,
            Expr::App(e) => &e.digest,
            Expr::For(e) => &e.digest,
            Expr::Deb(e) => &e.digest,
            Expr::Universe(e) => &e.digest,
        }
    }
}
impl GetDigest for RcHashed<Ind> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<Vcon> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<Match> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<Fun> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<App> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<For> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<DebNode> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcHashed<UniverseNode> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
