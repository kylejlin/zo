use crate::{
    hash::*,
    syntax_tree::{
        ast::{self, rc_hashed, Deb, RcHashed, RcHashedVec, Universe, UniverseLevel},
        ipist,
    },
};

use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct IpistToAstConverter {
    ind_cache: NoHashHashMap<Digest, RcHashed<ast::Ind>>,
    vcon_cache: NoHashHashMap<Digest, RcHashed<ast::Vcon>>,
    match_cache: NoHashHashMap<Digest, RcHashed<ast::Match>>,
    fun_cache: NoHashHashMap<Digest, RcHashed<ast::Fun>>,
    app_cache: NoHashHashMap<Digest, RcHashed<ast::App>>,
    for_cache: NoHashHashMap<Digest, RcHashed<ast::For>>,
}

impl IpistToAstConverter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl IpistToAstConverter {
    pub fn convert(&mut self, ist: ipist::Expr) -> ast::Expr {
        match ist {
            ipist::Expr::Ind(e) => self.convert_ind(e).into(),
            ipist::Expr::Vcon(e) => self.convert_vcon(e).into(),
            ipist::Expr::Match(e) => self.convert_match(e).into(),
            ipist::Expr::Fun(e) => self.convert_fun(e).into(),
            ipist::Expr::App(e) => self.convert_app(e).into(),
            ipist::Expr::For(e) => self.convert_for(e).into(),
            ipist::Expr::Deb(e) => ast::DebNode {
                deb: Deb(e.hashee.value),
            }
            .into(),
            ipist::Expr::Universe(e) => ast::UniverseNode {
                universe: Universe {
                    level: UniverseLevel(e.hashee.level),
                    erasable: e.hashee.erasable,
                },
            }
            .into(),
        }
    }

    pub fn convert_ind(&mut self, ist: RcHashed<ipist::Ind>) -> RcHashed<ast::Ind> {
        if let Some(ind) = self.ind_cache.get(&ist.digest) {
            return ind.clone();
        }

        self.convert_and_cache_unseen_ind(ist)
    }

    fn convert_and_cache_unseen_ind(&mut self, ist: RcHashed<ipist::Ind>) -> RcHashed<ast::Ind> {
        let digest = ist.digest.clone();
        let ind = self.convert_unseen_ind(ist);
        self.ind_cache.insert(digest, ind.clone());
        ind
    }

    fn convert_unseen_ind(&mut self, ist: RcHashed<ipist::Ind>) -> RcHashed<ast::Ind> {
        rc_hashed(ast::Ind {
            name: Rc::new(ast::StringValue(ist.hashee.name.value.to_owned())),
            universe: Universe {
                level: UniverseLevel(ist.hashee.type_.level),
                erasable: ist.hashee.type_.erasable,
            },
            index_types: self.convert_expressions(&ist.hashee.index_types),
            vcon_defs: self.convert_vcon_defs(ist.hashee.vcon_defs.clone()),
        })
    }

    pub fn convert_vcon_defs(&mut self, ist: Vec<ipist::VconDef>) -> RcHashedVec<ast::VconDef> {
        let v = ist
            .into_iter()
            .map(|def| self.convert_vcon_def(def))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_vcon_def(&mut self, ist: ipist::VconDef) -> ast::VconDef {
        ast::VconDef {
            param_types: self.convert_expressions(&ist.param_types),
            index_args: self.convert_expressions(&ist.index_args),
        }
    }

    pub fn convert_vcon(&mut self, ist: RcHashed<ipist::Vcon>) -> RcHashed<ast::Vcon> {
        if let Some(vcon) = self.vcon_cache.get(&ist.digest) {
            return vcon.clone();
        }

        self.convert_and_cache_unseen_vcon(ist)
    }

    fn convert_and_cache_unseen_vcon(&mut self, ist: RcHashed<ipist::Vcon>) -> RcHashed<ast::Vcon> {
        let digest = ist.digest.clone();
        let vcon = self.convert_unseen_vcon(ist);
        self.vcon_cache.insert(digest, vcon.clone());
        vcon
    }

    fn convert_unseen_vcon(&mut self, ist: RcHashed<ipist::Vcon>) -> RcHashed<ast::Vcon> {
        rc_hashed(ast::Vcon {
            ind: self.convert_ind(ist.hashee.ind.clone()),
            vcon_index: ist.hashee.vcon_index.value,
        })
    }

    pub fn convert_match(&mut self, ist: RcHashed<ipist::Match>) -> RcHashed<ast::Match> {
        if let Some(match_) = self.match_cache.get(&ist.digest) {
            return match_.clone();
        }

        self.convert_and_cache_unseen_match(ist)
    }

    fn convert_and_cache_unseen_match(
        &mut self,
        ist: RcHashed<ipist::Match>,
    ) -> RcHashed<ast::Match> {
        let digest = ist.digest.clone();
        let match_ = self.convert_unseen_match(ist);
        self.match_cache.insert(digest, match_.clone());
        match_
    }

    fn convert_unseen_match(&mut self, ist: RcHashed<ipist::Match>) -> RcHashed<ast::Match> {
        rc_hashed(ast::Match {
            matchee: self.convert(ist.hashee.matchee.clone()),
            return_type_arity: ist.hashee.return_type_arity.value,
            return_type: self.convert(ist.hashee.return_type.clone()),
            cases: self.convert_match_cases(ist.hashee.cases.clone()),
        })
    }

    fn convert_match_cases(&mut self, ist: Vec<ipist::MatchCase>) -> RcHashedVec<ast::MatchCase> {
        let v = ist
            .into_iter()
            .map(|case| self.convert_match_case(case))
            .collect();
        rc_hashed(v)
    }

    fn convert_match_case(&mut self, ist: ipist::MatchCase) -> ast::MatchCase {
        ast::MatchCase {
            arity: ist.arity.value,
            return_val: self.convert(ist.return_val),
        }
    }

    pub fn convert_fun(&mut self, ist: RcHashed<ipist::Fun>) -> RcHashed<ast::Fun> {
        if let Some(fun) = self.fun_cache.get(&ist.digest) {
            return fun.clone();
        }

        self.convert_and_cache_unseen_fun(ist)
    }

    fn convert_and_cache_unseen_fun(&mut self, ist: RcHashed<ipist::Fun>) -> RcHashed<ast::Fun> {
        let digest = ist.digest.clone();
        let fun = self.convert_unseen_fun(ist);
        self.fun_cache.insert(digest, fun.clone());
        fun
    }

    fn convert_unseen_fun(&mut self, ist: RcHashed<ipist::Fun>) -> RcHashed<ast::Fun> {
        rc_hashed(ast::Fun {
            decreasing_index: match ist.hashee.decreasing_index {
                ipist::NumberOrNonrecKw::NonrecKw(_) => None,
                ipist::NumberOrNonrecKw::Number(n) => Some(n.value),
            },
            param_types: self.convert_expressions(&ist.hashee.param_types),
            return_type: self.convert(ist.hashee.return_type.clone()),
            return_val: self.convert(ist.hashee.return_val.clone()),
        })
    }

    pub fn convert_app(&mut self, ist: RcHashed<ipist::App>) -> RcHashed<ast::App> {
        if let Some(app) = self.app_cache.get(&ist.digest) {
            return app.clone();
        }

        self.convert_and_cache_unseen_app(ist)
    }

    fn convert_and_cache_unseen_app(&mut self, ist: RcHashed<ipist::App>) -> RcHashed<ast::App> {
        let digest = ist.digest.clone();
        let app = self.convert_unseen_app(ist);
        self.app_cache.insert(digest, app.clone());
        app
    }

    fn convert_unseen_app(&mut self, ist: RcHashed<ipist::App>) -> RcHashed<ast::App> {
        rc_hashed(ast::App {
            callee: self.convert(ist.hashee.callee.clone()),
            args: self.convert_expressions(&ist.hashee.args),
        })
    }

    pub fn convert_for(&mut self, ist: RcHashed<ipist::For>) -> RcHashed<ast::For> {
        if let Some(for_) = self.for_cache.get(&ist.digest) {
            return for_.clone();
        }

        self.convert_and_cache_unseen_for(ist)
    }

    fn convert_and_cache_unseen_for(&mut self, ist: RcHashed<ipist::For>) -> RcHashed<ast::For> {
        let digest = ist.digest.clone();
        let for_ = self.convert_unseen_for(ist);
        self.for_cache.insert(digest, for_.clone());
        for_
    }

    fn convert_unseen_for(&mut self, ist: RcHashed<ipist::For>) -> RcHashed<ast::For> {
        rc_hashed(ast::For {
            param_types: self.convert_expressions(&ist.hashee.param_types),
            return_type: self.convert(ist.hashee.return_type.clone()),
        })
    }

    pub fn convert_expressions(&mut self, ist: &[ipist::Expr]) -> RcHashedVec<ast::Expr> {
        let v = ist
            .into_iter()
            .map(|expr| self.convert(expr.clone()))
            .collect();
        rc_hashed(v)
    }
}
