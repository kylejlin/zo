use crate::{hash::*, syntax_tree::ast::prelude::*};

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
    pub fn convert<A: AuxDataFamily>(&mut self, original: ast::Expr<A>) -> minimal_ast::Expr {
        match original {
            ast::Expr::Ind(e) => self.convert_ind(e).into(),
            ast::Expr::Vcon(e) => self.convert_vcon(e).into(),
            ast::Expr::Match(e) => self.convert_match(e).into(),
            ast::Expr::Fun(e) => self.convert_fun(e).into(),
            ast::Expr::App(e) => self.convert_app(e).into(),
            ast::Expr::For(e) => self.convert_for(e).into(),
            ast::Expr::Deb(e) => self.convert_deb_node(&e.hashee).into(),
            ast::Expr::Universe(e) => self.convert_universe_node(&e.hashee).into(),
        }
    }

    pub fn convert_ind<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Ind<A>>,
    ) -> RcHashed<minimal_ast::Ind> {
        if let Some(ind) = self.ind_cache.get(&original.digest) {
            return ind.clone();
        }

        self.convert_and_cache_unseen_ind(original)
    }

    fn convert_and_cache_unseen_ind<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Ind<A>>,
    ) -> RcHashed<minimal_ast::Ind> {
        let digest = original.digest.clone();
        let ind = self.convert_unseen_ind(original);
        self.ind_cache.insert(digest, ind.clone());
        ind
    }

    fn convert_unseen_ind<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Ind<A>>,
    ) -> RcHashed<minimal_ast::Ind> {
        rc_hashed(minimal_ast::Ind {
            name: original.hashee.name.clone(),
            universe: original.hashee.universe,
            index_types: self.convert_expressions(&original.hashee.index_types.hashee),
            vcon_defs: self.convert_vcon_defs(original.hashee.vcon_defs.clone()),
            aux_data: (),
        })
    }

    pub fn convert_vcon_defs<A: AuxDataFamily>(
        &mut self,
        original: RcHashedVec<ast::VconDef<A>>,
    ) -> RcHashedVec<minimal_ast::VconDef> {
        let v = original
            .hashee
            .iter()
            .cloned()
            .map(|def| self.convert_vcon_def(def))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_vcon_def<A: AuxDataFamily>(
        &mut self,
        original: ast::VconDef<A>,
    ) -> minimal_ast::VconDef {
        minimal_ast::VconDef {
            param_types: self.convert_expressions(&original.param_types.hashee),
            index_args: self.convert_expressions(&original.index_args.hashee),
            aux_data: (),
        }
    }

    pub fn convert_vcon<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Vcon<A>>,
    ) -> RcHashed<minimal_ast::Vcon> {
        if let Some(vcon) = self.vcon_cache.get(&original.digest) {
            return vcon.clone();
        }

        self.convert_and_cache_unseen_vcon(original)
    }

    fn convert_and_cache_unseen_vcon<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Vcon<A>>,
    ) -> RcHashed<minimal_ast::Vcon> {
        let digest = original.digest.clone();
        let vcon = self.convert_unseen_vcon(original);
        self.vcon_cache.insert(digest, vcon.clone());
        vcon
    }

    fn convert_unseen_vcon<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Vcon<A>>,
    ) -> RcHashed<minimal_ast::Vcon> {
        rc_hashed(minimal_ast::Vcon {
            ind: self.convert_ind(original.hashee.ind.clone()),
            vcon_index: original.hashee.vcon_index,
            aux_data: (),
        })
    }

    pub fn convert_match<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Match<A>>,
    ) -> RcHashed<minimal_ast::Match> {
        if let Some(match_) = self.match_cache.get(&original.digest) {
            return match_.clone();
        }

        self.convert_and_cache_unseen_match(original)
    }

    fn convert_and_cache_unseen_match<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Match<A>>,
    ) -> RcHashed<minimal_ast::Match> {
        let digest = original.digest.clone();
        let match_ = self.convert_unseen_match(original);
        self.match_cache.insert(digest, match_.clone());
        match_
    }

    fn convert_unseen_match<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Match<A>>,
    ) -> RcHashed<minimal_ast::Match> {
        rc_hashed(minimal_ast::Match {
            matchee: self.convert(original.hashee.matchee.clone()),
            return_type_arity: original.hashee.return_type_arity,
            return_type: self.convert(original.hashee.return_type.clone()),
            cases: self.convert_match_cases(original.hashee.cases.clone()),
            aux_data: (),
        })
    }

    fn convert_match_cases<A: AuxDataFamily>(
        &mut self,
        original: RcHashedVec<ast::MatchCase<A>>,
    ) -> RcHashedVec<minimal_ast::MatchCase> {
        let v = original
            .hashee
            .iter()
            .cloned()
            .map(|case| self.convert_match_case(case))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_match_case<A: AuxDataFamily>(
        &mut self,
        original: ast::MatchCase<A>,
    ) -> minimal_ast::MatchCase {
        minimal_ast::MatchCase {
            arity: original.arity,
            return_val: self.convert(original.return_val),
            aux_data: (),
        }
    }

    pub fn convert_fun<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Fun<A>>,
    ) -> RcHashed<minimal_ast::Fun> {
        if let Some(fun) = self.fun_cache.get(&original.digest) {
            return fun.clone();
        }

        self.convert_and_cache_unseen_fun(original)
    }

    fn convert_and_cache_unseen_fun<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Fun<A>>,
    ) -> RcHashed<minimal_ast::Fun> {
        let digest = original.digest.clone();
        let fun = self.convert_unseen_fun(original);
        self.fun_cache.insert(digest, fun.clone());
        fun
    }

    fn convert_unseen_fun<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::Fun<A>>,
    ) -> RcHashed<minimal_ast::Fun> {
        rc_hashed(minimal_ast::Fun {
            decreasing_index: original.hashee.decreasing_index,
            param_types: self.convert_expressions(&original.hashee.param_types.hashee),
            return_type: self.convert(original.hashee.return_type.clone()),
            return_val: self.convert(original.hashee.return_val.clone()),
            aux_data: (),
        })
    }

    pub fn convert_app<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::App<A>>,
    ) -> RcHashed<minimal_ast::App> {
        if let Some(app) = self.app_cache.get(&original.digest) {
            return app.clone();
        }

        self.convert_and_cache_unseen_app(original)
    }

    fn convert_and_cache_unseen_app<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::App<A>>,
    ) -> RcHashed<minimal_ast::App> {
        let digest = original.digest.clone();
        let app = self.convert_unseen_app(original);
        self.app_cache.insert(digest, app.clone());
        app
    }

    fn convert_unseen_app<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::App<A>>,
    ) -> RcHashed<minimal_ast::App> {
        rc_hashed(minimal_ast::App {
            callee: self.convert(original.hashee.callee.clone()),
            args: self.convert_expressions(&original.hashee.args.hashee),
            aux_data: (),
        })
    }

    pub fn convert_for<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::For<A>>,
    ) -> RcHashed<minimal_ast::For> {
        if let Some(for_) = self.for_cache.get(&original.digest) {
            return for_.clone();
        }

        self.convert_and_cache_unseen_for(original)
    }

    fn convert_and_cache_unseen_for<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::For<A>>,
    ) -> RcHashed<minimal_ast::For> {
        let digest = original.digest.clone();
        let for_ = self.convert_unseen_for(original);
        self.for_cache.insert(digest, for_.clone());
        for_
    }

    fn convert_unseen_for<A: AuxDataFamily>(
        &mut self,
        original: RcHashed<ast::For<A>>,
    ) -> RcHashed<minimal_ast::For> {
        rc_hashed(minimal_ast::For {
            param_types: self.convert_expressions(&original.hashee.param_types.hashee),
            return_type: self.convert(original.hashee.return_type.clone()),
            aux_data: (),
        })
    }

    pub fn convert_expressions<A: AuxDataFamily>(
        &mut self,
        original: &[ast::Expr<A>],
    ) -> RcHashedVec<minimal_ast::Expr> {
        let v = original
            .into_iter()
            .map(|expr| self.convert(expr.clone()))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_deb_node<A: AuxDataFamily>(
        &mut self,
        // Since debs are leaf nodes,
        // caching the conversion result
        // will not save much time.
        // Therefore, we do not need the digest,
        // so we take a normal reference instead of a `RcHashed`.
        original: &ast::DebNode<A>,
    ) -> RcHashed<minimal_ast::DebNode> {
        rc_hashed(minimal_ast::DebNode {
            deb: original.deb,
            aux_data: (),
        })
    }

    pub fn convert_universe_node<A: AuxDataFamily>(
        &mut self,
        // Since debs are leaf nodes,
        // caching the conversion result
        // will not save much time.
        // Therefore, we do not need the digest,
        // so we take a normal reference instead of a `RcHashed`.
        original: &ast::UniverseNode<A>,
    ) -> RcHashed<minimal_ast::UniverseNode> {
        rc_hashed(minimal_ast::UniverseNode {
            universe: original.universe,
            aux_data: (),
        })
    }
}
