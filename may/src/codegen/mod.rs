use zoc::syntax_tree::ast as znode;

mod mnode {
    pub use crate::{cst::*, token::*};
}

use zoc::syntax_tree::replace_debs::{DebUpshifter, ReplaceDebs};
use zoc::{
    hash::{Digest, GetDigest, NoHashHashMap},
    syntax_tree::ast::{
        rc_hashed as bypass_cache_and_rc_hash, Deb, RcHashed, RcHashedVec, UniverseLevel,
    },
};

pub mod error;
pub use error::*;

pub mod context;
pub use context::*;

pub fn may_to_zo(expr: &mnode::Expr) -> Result<znode::Expr, SemanticError> {
    MayConverter::default().convert(expr, Context::empty())
}

#[derive(Debug, Default)]
struct MayConverter {
    znode_cache: NoHashHashMap<Digest, znode::Expr>,
    znode_vec_cache: NoHashHashMap<Digest, RcHashedVec<znode::Expr>>,
}

impl MayConverter {
    fn convert(
        &mut self,
        expr: &mnode::Expr,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::Expr::Let(e) => self.convert_let(e, context),
            mnode::Expr::Ind(e) => self.convert_ind(e, context),
            mnode::Expr::Fun(e) => self.convert_fun(e, context),
            mnode::Expr::Aind(e) => self.convert_aind(e, context),
            mnode::Expr::Vcon(e) => self.convert_vcon(e, context),
            mnode::Expr::Match(e) => self.convert_match(e, context),
            mnode::Expr::Afun(e) => self.convert_afun(e, context),
            mnode::Expr::For(e) => self.convert_for(e, context),
            mnode::Expr::VarOrApp(e) => self.convert_var_or_app(e, context),
            mnode::Expr::Universe(e) => self.convert_universe(e),
        }
    }

    fn convert_let(
        &mut self,
        expr: &mnode::Let,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_ind(
        &mut self,
        expr: &mnode::Ind,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_fun(
        &mut self,
        expr: &mnode::Fun,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_aind(
        &mut self,
        expr: &mnode::Aind,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_vcon(
        &mut self,
        expr: &mnode::Vcon,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_match(
        &mut self,
        expr: &mnode::Match,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_afun(
        &mut self,
        expr: &mnode::Afun,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_for(
        &mut self,
        expr: &mnode::For,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_var_or_app(
        &mut self,
        expr: &mnode::VarOrApp,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::VarOrApp::Var(e) => self.convert_var(e, context),
            mnode::VarOrApp::App(e) => self.convert_app(e, context),
        }
    }

    fn convert_var(
        &mut self,
        expr: &mnode::Ident,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let Some((entry, Distance(dist))) = context.get(&expr.value) else {
            return Err(SemanticError::VarNotDefined(expr.clone()));
        };
        let val = entry.val.clone().replace_debs(&DebUpshifter(dist), 0);
        Ok(self.cache_expr(val))
    }

    fn convert_app(
        &mut self,
        expr: &mnode::App,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let callee = self.convert_var_or_app(&expr.callee, context)?;
        let args = self.convert_exprs(&expr.args, context)?;
        Ok(self.cache_app(znode::App { callee, args }))
    }

    fn convert_universe(
        &mut self,
        expr: &mnode::UniverseLiteral,
    ) -> Result<znode::Expr, SemanticError> {
        Ok(self.cache_universe(znode::UniverseNode {
            level: UniverseLevel(expr.level),
        }))
    }
}

impl MayConverter {
    fn convert_exprs(
        &mut self,
        exprs: &mnode::CommaSeparatedExprs,
        context: Context,
    ) -> Result<RcHashedVec<znode::Expr>, SemanticError> {
        let v = self.convert_exprs_without_hashing(exprs, context)?;
        Ok(self.cache_expr_vec(v))
    }

    fn convert_exprs_without_hashing(
        &mut self,
        exprs: &mnode::CommaSeparatedExprs,
        context: Context,
    ) -> Result<Vec<znode::Expr>, SemanticError> {
        match exprs {
            mnode::CommaSeparatedExprs::One(e) => {
                let e = self.convert(e, context)?;
                Ok(vec![e])
            }
            mnode::CommaSeparatedExprs::Snoc(rdc, rac) => {
                let mut rdc = self.convert_exprs_without_hashing(rdc, context)?;
                let rac = self.convert(rac, context)?;
                rdc.push(rac);
                Ok(rdc)
            }
        }
    }
}

impl MayConverter {
    fn cache_expr(&mut self, node: znode::Expr) -> znode::Expr {
        let digest = node.digest();
        if let Some(existing) = self.znode_cache.get(digest) {
            return existing.clone();
        }

        self.znode_cache.insert(digest.clone(), node.clone());
        node
    }

    fn cache_app(&mut self, node: znode::App) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::App(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    fn cache_universe(&mut self, node: znode::UniverseNode) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Universe(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    fn cache_expr_vec(&mut self, node: Vec<znode::Expr>) -> RcHashedVec<znode::Expr> {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_vec_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = hashed;
        self.znode_vec_cache.insert(digest, out.clone());
        out
    }
}
