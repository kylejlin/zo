use crate::{
    hash::{sha256::*, *},
    syntax_tree::{
        ast::{self, rc_sem_hashed, Deb, RcSemHashed, RcSemHashedVec, UniverseLevel},
        rch_cst::{self as cst, RcHashed},
    },
};

use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct RchCstToAstConverter {
    ind_cache: NoHashHashMap<Digest, RcSemHashed<ast::Ind>>,
    vcon_cache: NoHashHashMap<Digest, RcSemHashed<ast::Vcon>>,
    match_cache: NoHashHashMap<Digest, RcSemHashed<ast::Match>>,
    fun_cache: NoHashHashMap<Digest, RcSemHashed<ast::Fun>>,
    app_cache: NoHashHashMap<Digest, RcSemHashed<ast::App>>,
    for_cache: NoHashHashMap<Digest, RcSemHashed<ast::For>>,
}

impl RchCstToAstConverter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl RchCstToAstConverter {
    pub fn convert(&mut self, cst: cst::Expr) -> ast::Expr {
        match cst {
            cst::Expr::Ind(e) => self.convert_ind(e).into(),
            cst::Expr::Vcon(e) => self.convert_vcon(e).into(),
            cst::Expr::Match(e) => self.convert_match(e).into(),
            cst::Expr::Fun(e) => self.convert_fun(e).into(),
            cst::Expr::App(e) => self.convert_app(e).into(),
            cst::Expr::For(e) => self.convert_for(e).into(),
            cst::Expr::Deb(e) => ast::DebNode {
                deb: Deb(e.value.value),
            }
            .into(),
            cst::Expr::Universe(e) => ast::UniverseNode {
                level: UniverseLevel(e.value.level),
            }
            .into(),
        }
    }

    pub fn convert_ind(&mut self, cst: RcHashed<cst::Ind>) -> RcSemHashed<ast::Ind> {
        if let Some(ind) = self.ind_cache.get(&cst.digest) {
            return ind.clone();
        }

        self.convert_and_cache_unseen_ind(cst)
    }

    fn convert_and_cache_unseen_ind(&mut self, cst: RcHashed<cst::Ind>) -> RcSemHashed<ast::Ind> {
        let digest = cst.digest.clone();
        let ind = self.convert_unseen_ind(cst);
        self.ind_cache.insert(digest, ind.clone());
        ind
    }

    fn convert_unseen_ind(&mut self, cst: RcHashed<cst::Ind>) -> RcSemHashed<ast::Ind> {
        rc_sem_hashed(ast::Ind {
            name: Rc::new(ast::StringValue(cst.value.name.value.to_owned())),
            universe_level: UniverseLevel(cst.value.type_.level),
            index_types: self.convert_expressions(cst.value.index_types.clone()),
            vcon_defs: self.convert_vcon_defs(cst.value.vcon_defs.clone()),
        })
    }

    pub fn convert_vcon_defs(
        &mut self,
        cst: cst::ZeroOrMoreVconDefs,
    ) -> RcSemHashedVec<ast::VconDef> {
        let v = self.convert_vcon_defs_to_vec(cst);
        rc_sem_hashed(v)
    }

    fn convert_vcon_defs_to_vec(&mut self, cst: cst::ZeroOrMoreVconDefs) -> Vec<ast::VconDef> {
        match cst {
            cst::ZeroOrMoreVconDefs::Nil => vec![],
            cst::ZeroOrMoreVconDefs::Cons(left, right) => {
                let mut left = self.convert_vcon_defs_to_vec(*left);
                let right = self.convert_vcon_def(right);
                left.push(right);
                left
            }
        }
    }

    fn convert_vcon_def(&mut self, cst: cst::VconDef) -> ast::VconDef {
        ast::VconDef {
            param_types: self.convert_expressions(cst.param_types),
            index_args: self.convert_expressions(cst.index_args),
        }
    }

    pub fn convert_vcon(&mut self, cst: RcHashed<cst::Vcon>) -> RcSemHashed<ast::Vcon> {
        if let Some(vcon) = self.vcon_cache.get(&cst.digest) {
            return vcon.clone();
        }

        self.convert_and_cache_unseen_vcon(cst)
    }

    fn convert_and_cache_unseen_vcon(
        &mut self,
        cst: RcHashed<cst::Vcon>,
    ) -> RcSemHashed<ast::Vcon> {
        let digest = cst.digest.clone();
        let vcon = self.convert_unseen_vcon(cst);
        self.vcon_cache.insert(digest, vcon.clone());
        vcon
    }

    fn convert_unseen_vcon(&mut self, cst: RcHashed<cst::Vcon>) -> RcSemHashed<ast::Vcon> {
        rc_sem_hashed(ast::Vcon {
            ind: self.convert_ind(cst.value.ind.clone()),
            vcon_index: cst.value.vcon_index.value,
        })
    }

    pub fn convert_match(&mut self, cst: RcHashed<cst::Match>) -> RcSemHashed<ast::Match> {
        if let Some(match_) = self.match_cache.get(&cst.digest) {
            return match_.clone();
        }

        self.convert_and_cache_unseen_match(cst)
    }

    fn convert_and_cache_unseen_match(
        &mut self,
        cst: RcHashed<cst::Match>,
    ) -> RcSemHashed<ast::Match> {
        let digest = cst.digest.clone();
        let match_ = self.convert_unseen_match(cst);
        self.match_cache.insert(digest, match_.clone());
        match_
    }

    fn convert_unseen_match(&mut self, cst: RcHashed<cst::Match>) -> RcSemHashed<ast::Match> {
        rc_sem_hashed(ast::Match {
            matchee: self.convert(cst.value.matchee.clone()),
            return_type: self.convert(cst.value.return_type.clone()),
            cases: self.convert_match_cases(cst.value.cases.clone()),
        })
    }

    fn convert_match_cases(
        &mut self,
        cst: cst::ZeroOrMoreMatchCases,
    ) -> RcSemHashedVec<ast::MatchCase> {
        let v = self.convert_match_cases_to_vec(cst);
        rc_sem_hashed(v)
    }

    fn convert_match_cases_to_vec(
        &mut self,
        cst: cst::ZeroOrMoreMatchCases,
    ) -> Vec<ast::MatchCase> {
        match cst {
            cst::ZeroOrMoreMatchCases::Nil => vec![],
            cst::ZeroOrMoreMatchCases::Cons(left, right) => {
                let mut left = self.convert_match_cases_to_vec(*left);
                let right = self.convert_match_case(right);
                left.push(right);
                left
            }
        }
    }

    fn convert_match_case(&mut self, cst: cst::MatchCase) -> ast::MatchCase {
        ast::MatchCase {
            arity: cst.arity.value,
            return_val: self.convert(cst.return_val),
        }
    }

    pub fn convert_fun(&mut self, cst: RcHashed<cst::Fun>) -> RcSemHashed<ast::Fun> {
        if let Some(fun) = self.fun_cache.get(&cst.digest) {
            return fun.clone();
        }

        self.convert_and_cache_unseen_fun(cst)
    }

    fn convert_and_cache_unseen_fun(&mut self, cst: RcHashed<cst::Fun>) -> RcSemHashed<ast::Fun> {
        let digest = cst.digest.clone();
        let fun = self.convert_unseen_fun(cst);
        self.fun_cache.insert(digest, fun.clone());
        fun
    }

    fn convert_unseen_fun(&mut self, cst: RcHashed<cst::Fun>) -> RcSemHashed<ast::Fun> {
        rc_sem_hashed(ast::Fun {
            decreasing_index: match cst.value.decreasing_index {
                cst::NumberOrNonrecKw::NonrecKw(_) => None,
                cst::NumberOrNonrecKw::Number(n) => Some(n.value),
            },
            param_types: self.convert_expressions(cst.value.param_types.clone()),
            return_type: self.convert(cst.value.return_type.clone()),
            return_val: self.convert(cst.value.return_val.clone()),
        })
    }

    pub fn convert_app(&mut self, cst: RcHashed<cst::App>) -> RcSemHashed<ast::App> {
        if let Some(app) = self.app_cache.get(&cst.digest) {
            return app.clone();
        }

        self.convert_and_cache_unseen_app(cst)
    }

    fn convert_and_cache_unseen_app(&mut self, cst: RcHashed<cst::App>) -> RcSemHashed<ast::App> {
        let digest = cst.digest.clone();
        let app = self.convert_unseen_app(cst);
        self.app_cache.insert(digest, app.clone());
        app
    }

    fn convert_unseen_app(&mut self, cst: RcHashed<cst::App>) -> RcSemHashed<ast::App> {
        rc_sem_hashed(ast::App {
            callee: self.convert(cst.value.callee.clone()),
            args: self.convert_expressions(cst.value.args.clone()),
        })
    }

    pub fn convert_for(&mut self, cst: RcHashed<cst::For>) -> RcSemHashed<ast::For> {
        if let Some(for_) = self.for_cache.get(&cst.digest) {
            return for_.clone();
        }

        self.convert_and_cache_unseen_for(cst)
    }

    fn convert_and_cache_unseen_for(&mut self, cst: RcHashed<cst::For>) -> RcSemHashed<ast::For> {
        let digest = cst.digest.clone();
        let for_ = self.convert_unseen_for(cst);
        self.for_cache.insert(digest, for_.clone());
        for_
    }

    fn convert_unseen_for(&mut self, cst: RcHashed<cst::For>) -> RcSemHashed<ast::For> {
        rc_sem_hashed(ast::For {
            param_types: self.convert_expressions(cst.value.param_types.clone()),
            return_type: self.convert(cst.value.return_type.clone()),
        })
    }

    pub fn convert_expressions(&mut self, cst: cst::ZeroOrMoreExprs) -> RcSemHashedVec<ast::Expr> {
        let v = self.convert_expressions_to_vec(cst);
        rc_sem_hashed(v)
    }

    fn convert_expressions_to_vec(&mut self, cst: cst::ZeroOrMoreExprs) -> Vec<ast::Expr> {
        match cst {
            cst::ZeroOrMoreExprs::Nil => vec![],
            cst::ZeroOrMoreExprs::Cons(left, right) => {
                let mut left = self.convert_expressions_to_vec(*left);
                let right = self.convert(right);
                left.push(right);
                left
            }
        }
    }
}
