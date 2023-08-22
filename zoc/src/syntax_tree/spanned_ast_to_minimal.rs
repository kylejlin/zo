use crate::{
    hash::*,
    syntax_tree::{ast::prelude::*, minimal_ast, spanned_ast},
};

#[derive(Debug, Clone, Default)]
pub struct SpanRemover {
    ind_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Ind>>,
    vcon_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Vcon>>,
    match_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Match>>,
    fun_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::Fun>>,
    app_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::App>>,
    for_cache: NoHashHashMap<Digest, RcHashed<minimal_ast::For>>,
}

impl SpanRemover {
    pub fn new() -> Self {
        Default::default()
    }
}

impl SpanRemover {
    pub fn convert(&mut self, ist: spanned_ast::Expr) -> minimal_ast::Expr {
        match ist {
            spanned_ast::Expr::Ind(e) => self.convert_ind(e).into(),
            spanned_ast::Expr::Vcon(e) => self.convert_vcon(e).into(),
            spanned_ast::Expr::Match(e) => self.convert_match(e).into(),
            spanned_ast::Expr::Fun(e) => self.convert_fun(e).into(),
            spanned_ast::Expr::App(e) => self.convert_app(e).into(),
            spanned_ast::Expr::For(e) => self.convert_for(e).into(),
            spanned_ast::Expr::Deb(e) => minimal_ast::DebNode {
                deb: e.hashee.deb,
                aux_data: (),
            }
            .into(),
            spanned_ast::Expr::Universe(e) => minimal_ast::UniverseNode {
                universe: e.hashee.universe,
                aux_data: (),
            }
            .into(),
        }
    }

    pub fn convert_ind(&mut self, ist: RcHashed<spanned_ast::Ind>) -> RcHashed<minimal_ast::Ind> {
        if let Some(ind) = self.ind_cache.get(&ist.digest) {
            return ind.clone();
        }

        self.convert_and_cache_unseen_ind(ist)
    }

    fn convert_and_cache_unseen_ind(
        &mut self,
        ist: RcHashed<spanned_ast::Ind>,
    ) -> RcHashed<minimal_ast::Ind> {
        let digest = ist.digest.clone();
        let ind = self.convert_unseen_ind(ist);
        self.ind_cache.insert(digest, ind.clone());
        ind
    }

    fn convert_unseen_ind(
        &mut self,
        ist: RcHashed<spanned_ast::Ind>,
    ) -> RcHashed<minimal_ast::Ind> {
        rc_hashed(minimal_ast::Ind {
            name: ist.hashee.name.clone(),
            universe: ist.hashee.universe,
            index_types: self.convert_expressions(&ist.hashee.index_types.hashee),
            vcon_defs: self.convert_vcon_defs(ist.hashee.vcon_defs.clone()),
            aux_data: (),
        })
    }

    pub fn convert_vcon_defs(
        &mut self,
        ist: RcHashedVec<spanned_ast::VconDef>,
    ) -> RcHashedVec<minimal_ast::VconDef> {
        let v = ist
            .hashee
            .iter()
            .cloned()
            .map(|def| self.convert_vcon_def(def))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_vcon_def(&mut self, ist: spanned_ast::VconDef) -> minimal_ast::VconDef {
        minimal_ast::VconDef {
            param_types: self.convert_expressions(&ist.param_types.hashee),
            index_args: self.convert_expressions(&ist.index_args.hashee),
            aux_data: (),
        }
    }

    pub fn convert_vcon(
        &mut self,
        ist: RcHashed<spanned_ast::Vcon>,
    ) -> RcHashed<minimal_ast::Vcon> {
        if let Some(vcon) = self.vcon_cache.get(&ist.digest) {
            return vcon.clone();
        }

        self.convert_and_cache_unseen_vcon(ist)
    }

    fn convert_and_cache_unseen_vcon(
        &mut self,
        ist: RcHashed<spanned_ast::Vcon>,
    ) -> RcHashed<minimal_ast::Vcon> {
        let digest = ist.digest.clone();
        let vcon = self.convert_unseen_vcon(ist);
        self.vcon_cache.insert(digest, vcon.clone());
        vcon
    }

    fn convert_unseen_vcon(
        &mut self,
        ist: RcHashed<spanned_ast::Vcon>,
    ) -> RcHashed<minimal_ast::Vcon> {
        rc_hashed(minimal_ast::Vcon {
            ind: self.convert_ind(ist.hashee.ind.clone()),
            vcon_index: ist.hashee.vcon_index,
            aux_data: (),
        })
    }

    pub fn convert_match(
        &mut self,
        ist: RcHashed<spanned_ast::Match>,
    ) -> RcHashed<minimal_ast::Match> {
        if let Some(match_) = self.match_cache.get(&ist.digest) {
            return match_.clone();
        }

        self.convert_and_cache_unseen_match(ist)
    }

    fn convert_and_cache_unseen_match(
        &mut self,
        ist: RcHashed<spanned_ast::Match>,
    ) -> RcHashed<minimal_ast::Match> {
        let digest = ist.digest.clone();
        let match_ = self.convert_unseen_match(ist);
        self.match_cache.insert(digest, match_.clone());
        match_
    }

    fn convert_unseen_match(
        &mut self,
        ist: RcHashed<spanned_ast::Match>,
    ) -> RcHashed<minimal_ast::Match> {
        rc_hashed(minimal_ast::Match {
            matchee: self.convert(ist.hashee.matchee.clone()),
            return_type_arity: ist.hashee.return_type_arity,
            return_type: self.convert(ist.hashee.return_type.clone()),
            cases: self.convert_match_cases(ist.hashee.cases.clone()),
            aux_data: (),
        })
    }

    fn convert_match_cases(
        &mut self,
        ist: RcHashedVec<spanned_ast::MatchCase>,
    ) -> RcHashedVec<minimal_ast::MatchCase> {
        let v = ist
            .hashee
            .iter()
            .cloned()
            .map(|case| self.convert_match_case(case))
            .collect();
        rc_hashed(v)
    }

    pub fn convert_match_case(&mut self, ist: spanned_ast::MatchCase) -> minimal_ast::MatchCase {
        minimal_ast::MatchCase {
            arity: ist.arity,
            return_val: self.convert(ist.return_val),
            aux_data: (),
        }
    }

    pub fn convert_fun(&mut self, ist: RcHashed<spanned_ast::Fun>) -> RcHashed<minimal_ast::Fun> {
        if let Some(fun) = self.fun_cache.get(&ist.digest) {
            return fun.clone();
        }

        self.convert_and_cache_unseen_fun(ist)
    }

    fn convert_and_cache_unseen_fun(
        &mut self,
        ist: RcHashed<spanned_ast::Fun>,
    ) -> RcHashed<minimal_ast::Fun> {
        let digest = ist.digest.clone();
        let fun = self.convert_unseen_fun(ist);
        self.fun_cache.insert(digest, fun.clone());
        fun
    }

    fn convert_unseen_fun(
        &mut self,
        ist: RcHashed<spanned_ast::Fun>,
    ) -> RcHashed<minimal_ast::Fun> {
        rc_hashed(minimal_ast::Fun {
            decreasing_index: ist.hashee.decreasing_index,
            param_types: self.convert_expressions(&ist.hashee.param_types.hashee),
            return_type: self.convert(ist.hashee.return_type.clone()),
            return_val: self.convert(ist.hashee.return_val.clone()),
            aux_data: (),
        })
    }

    pub fn convert_app(&mut self, ist: RcHashed<spanned_ast::App>) -> RcHashed<minimal_ast::App> {
        if let Some(app) = self.app_cache.get(&ist.digest) {
            return app.clone();
        }

        self.convert_and_cache_unseen_app(ist)
    }

    fn convert_and_cache_unseen_app(
        &mut self,
        ist: RcHashed<spanned_ast::App>,
    ) -> RcHashed<minimal_ast::App> {
        let digest = ist.digest.clone();
        let app = self.convert_unseen_app(ist);
        self.app_cache.insert(digest, app.clone());
        app
    }

    fn convert_unseen_app(
        &mut self,
        ist: RcHashed<spanned_ast::App>,
    ) -> RcHashed<minimal_ast::App> {
        rc_hashed(minimal_ast::App {
            callee: self.convert(ist.hashee.callee.clone()),
            args: self.convert_expressions(&ist.hashee.args.hashee),
            aux_data: (),
        })
    }

    pub fn convert_for(&mut self, ist: RcHashed<spanned_ast::For>) -> RcHashed<minimal_ast::For> {
        if let Some(for_) = self.for_cache.get(&ist.digest) {
            return for_.clone();
        }

        self.convert_and_cache_unseen_for(ist)
    }

    fn convert_and_cache_unseen_for(
        &mut self,
        ist: RcHashed<spanned_ast::For>,
    ) -> RcHashed<minimal_ast::For> {
        let digest = ist.digest.clone();
        let for_ = self.convert_unseen_for(ist);
        self.for_cache.insert(digest, for_.clone());
        for_
    }

    fn convert_unseen_for(
        &mut self,
        ist: RcHashed<spanned_ast::For>,
    ) -> RcHashed<minimal_ast::For> {
        rc_hashed(minimal_ast::For {
            param_types: self.convert_expressions(&ist.hashee.param_types.hashee),
            return_type: self.convert(ist.hashee.return_type.clone()),
            aux_data: (),
        })
    }

    pub fn convert_expressions(
        &mut self,
        ist: &[spanned_ast::Expr],
    ) -> RcHashedVec<minimal_ast::Expr> {
        let v = ist
            .into_iter()
            .map(|expr| self.convert(expr.clone()))
            .collect();
        rc_hashed(v)
    }
}
