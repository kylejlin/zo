use crate::{
    hash::*,
    syntax_tree::{ast::prelude::*, minimal_ast},
};

#[derive(Debug, Clone, Default)]
pub struct AuxDataRemover {
    ind_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Ind>>,
    vcon_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Vcon>>,
    match_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Match>>,
    fun_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Fun>>,
    app_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::App>>,
    for_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::For>>,
}

impl AuxDataRemover {
    pub fn new() -> Self {
        Default::default()
    }
}

impl AuxDataRemover {
    pub fn convert<A: AuxDataFamily>(&mut self, ist: ast::Expr<A>) -> minimal_ast::Expr {
        match ist {
            ast::Expr::Ind(e) => self.convert_ind(e).into(),
            ast::Expr::Vcon(e) => self.convert_vcon(e).into(),
            ast::Expr::Match(e) => self.convert_match(e).into(),
            ast::Expr::Fun(e) => self.convert_fun(e).into(),
            ast::Expr::App(e) => self.convert_app(e).into(),
            ast::Expr::For(e) => self.convert_for(e).into(),
            ast::Expr::Deb(e) => minimal_ast::DebNode {
                deb: e.hashee.deb,
                aux_data: (),
            }
            .into(),
            ast::Expr::Universe(e) => minimal_ast::UniverseNode {
                universe: e.hashee.universe,
                aux_data: (),
            }
            .into(),
        }
    }

    pub fn convert_ind<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Ind<A>>,
    ) -> RcHashed<minimal_ast::Ind> {
        if let Some(ind) = self.ind_cache.get(&ist.digest) {
            return ind.clone();
        }

        self.convert_and_cache_unseen_ind(ist)
    }

    fn convert_and_cache_unseen_ind<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Ind<A>>,
    ) -> RcHashed<minimal_ast::Ind> {
        let digest = ist.digest.clone();
        let ind = self.convert_unseen_ind(ist);
        self.ind_cache.insert(digest, ind.clone());
        ind
    }

    fn convert_unseen_ind<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Ind<A>>,
    ) -> RcHashed<minimal_ast::Ind> {
        rc_hashed(minimal_ast::Ind {
            name: ist.hashee.name.clone(),
            universe: ist.hashee.universe,
            index_types: self.convert_expressions(&ist.hashee.index_types.hashee),
            vcon_defs: self.convert_vcon_defs(ist.hashee.vcon_defs.clone()),
            aux_data: (),
        })
    }

    pub fn convert_vcon_defs<A: AuxDataFamily>(
        &mut self,
        ist: RcHashedVec<ast::VconDef<A>>,
    ) -> RcHashedVec<minimal_ast::VconDef> {
        let v = ist
            .hashee
            .iter()
            .cloned()
            .map(|def| self.convert_vcon_def(def))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_vcon_def<A: AuxDataFamily>(
        &mut self,
        ist: ast::VconDef<A>,
    ) -> minimal_ast::VconDef {
        minimal_ast::VconDef {
            param_types: self.convert_expressions(&ist.param_types.hashee),
            index_args: self.convert_expressions(&ist.index_args.hashee),
            aux_data: (),
        }
    }

    pub fn convert_vcon<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Vcon<A>>,
    ) -> RcHashed<minimal_ast::Vcon> {
        if let Some(vcon) = self.vcon_cache.get(&ist.digest) {
            return vcon.clone();
        }

        self.convert_and_cache_unseen_vcon(ist)
    }

    fn convert_and_cache_unseen_vcon<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Vcon<A>>,
    ) -> RcHashed<minimal_ast::Vcon> {
        let digest = ist.digest.clone();
        let vcon = self.convert_unseen_vcon(ist);
        self.vcon_cache.insert(digest, vcon.clone());
        vcon
    }

    fn convert_unseen_vcon<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Vcon<A>>,
    ) -> RcHashed<minimal_ast::Vcon> {
        rc_hashed(minimal_ast::Vcon {
            ind: self.convert_ind(ist.hashee.ind.clone()),
            vcon_index: ist.hashee.vcon_index,
            aux_data: (),
        })
    }

    pub fn convert_match<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Match<A>>,
    ) -> RcHashed<minimal_ast::Match> {
        if let Some(match_) = self.match_cache.get(&ist.digest) {
            return match_.clone();
        }

        self.convert_and_cache_unseen_match(ist)
    }

    fn convert_and_cache_unseen_match<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Match<A>>,
    ) -> RcHashed<minimal_ast::Match> {
        let digest = ist.digest.clone();
        let match_ = self.convert_unseen_match(ist);
        self.match_cache.insert(digest, match_.clone());
        match_
    }

    fn convert_unseen_match<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Match<A>>,
    ) -> RcHashed<minimal_ast::Match> {
        rc_hashed(minimal_ast::Match {
            matchee: self.convert(ist.hashee.matchee.clone()),
            return_type_arity: ist.hashee.return_type_arity,
            return_type: self.convert(ist.hashee.return_type.clone()),
            cases: self.convert_match_cases(ist.hashee.cases.clone()),
            aux_data: (),
        })
    }

    fn convert_match_cases<A: AuxDataFamily>(
        &mut self,
        ist: RcHashedVec<ast::MatchCase<A>>,
    ) -> RcHashedVec<minimal_ast::MatchCase> {
        let v = ist
            .hashee
            .iter()
            .cloned()
            .map(|case| self.convert_match_case(case))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_match_case<A: AuxDataFamily>(
        &mut self,
        ist: ast::MatchCase<A>,
    ) -> minimal_ast::MatchCase {
        minimal_ast::MatchCase {
            arity: ist.arity,
            return_val: self.convert(ist.return_val),
            aux_data: (),
        }
    }

    pub fn convert_fun<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Fun<A>>,
    ) -> RcHashed<minimal_ast::Fun> {
        if let Some(fun) = self.fun_cache.get(&ist.digest) {
            return fun.clone();
        }

        self.convert_and_cache_unseen_fun(ist)
    }

    fn convert_and_cache_unseen_fun<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Fun<A>>,
    ) -> RcHashed<minimal_ast::Fun> {
        let digest = ist.digest.clone();
        let fun = self.convert_unseen_fun(ist);
        self.fun_cache.insert(digest, fun.clone());
        fun
    }

    fn convert_unseen_fun<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::Fun<A>>,
    ) -> RcHashed<minimal_ast::Fun> {
        rc_hashed(minimal_ast::Fun {
            decreasing_index: ist.hashee.decreasing_index,
            param_types: self.convert_expressions(&ist.hashee.param_types.hashee),
            return_type: self.convert(ist.hashee.return_type.clone()),
            return_val: self.convert(ist.hashee.return_val.clone()),
            aux_data: (),
        })
    }

    pub fn convert_app<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::App<A>>,
    ) -> RcHashed<minimal_ast::App> {
        if let Some(app) = self.app_cache.get(&ist.digest) {
            return app.clone();
        }

        self.convert_and_cache_unseen_app(ist)
    }

    fn convert_and_cache_unseen_app<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::App<A>>,
    ) -> RcHashed<minimal_ast::App> {
        let digest = ist.digest.clone();
        let app = self.convert_unseen_app(ist);
        self.app_cache.insert(digest, app.clone());
        app
    }

    fn convert_unseen_app<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::App<A>>,
    ) -> RcHashed<minimal_ast::App> {
        rc_hashed(minimal_ast::App {
            callee: self.convert(ist.hashee.callee.clone()),
            args: self.convert_expressions(&ist.hashee.args.hashee),
            aux_data: (),
        })
    }

    pub fn convert_for<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::For<A>>,
    ) -> RcHashed<minimal_ast::For> {
        if let Some(for_) = self.for_cache.get(&ist.digest) {
            return for_.clone();
        }

        self.convert_and_cache_unseen_for(ist)
    }

    fn convert_and_cache_unseen_for<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::For<A>>,
    ) -> RcHashed<minimal_ast::For> {
        let digest = ist.digest.clone();
        let for_ = self.convert_unseen_for(ist);
        self.for_cache.insert(digest, for_.clone());
        for_
    }

    fn convert_unseen_for<A: AuxDataFamily>(
        &mut self,
        ist: RcHashed<ast::For<A>>,
    ) -> RcHashed<minimal_ast::For> {
        rc_hashed(minimal_ast::For {
            param_types: self.convert_expressions(&ist.hashee.param_types.hashee),
            return_type: self.convert(ist.hashee.return_type.clone()),
            aux_data: (),
        })
    }

    pub fn convert_expressions<A: AuxDataFamily>(
        &mut self,
        ist: &[ast::Expr<A>],
    ) -> RcHashedVec<minimal_ast::Expr> {
        let v = ist
            .into_iter()
            .map(|expr| self.convert(expr.clone()))
            .collect();
        rc_hashed(v)
    }
}
