use super::*;

impl<A: AuxDataFamily> GetDigest for Expr<A> {
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
impl<A: AuxDataFamily> GetDigest for RcHashed<Ind<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<Vcon<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<Match<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<Fun<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<App<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<For<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<DebNode<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl<A: AuxDataFamily> GetDigest for RcHashed<UniverseNode<A>> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
