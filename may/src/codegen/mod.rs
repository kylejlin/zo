use zoc::syntax_tree::ast as znode;

mod mnode {
    pub use crate::{cst::*, token::*};
}

use zoc::{
    hash::{Digest, NoHashHashMap},
    syntax_tree::ast::{rc_hashed as bypass_cache_and_rc_hash, Deb, RcHashed, UniverseLevel},
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
}

impl MayConverter {
    fn convert(&mut self, expr: &mnode::Expr, con: Context) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::Expr::Let(e) => self.convert_let(e, con),
            mnode::Expr::Ind(e) => self.convert_ind(e, con),
            mnode::Expr::Fun(e) => self.convert_fun(e, con),
            mnode::Expr::Aind(e) => self.convert_aind(e, con),
            mnode::Expr::Vcon(e) => self.convert_vcon(e, con),
            mnode::Expr::Match(e) => self.convert_match(e, con),
            mnode::Expr::Afun(e) => self.convert_afun(e, con),
            mnode::Expr::For(e) => self.convert_for(e, con),
            mnode::Expr::VarOrApp(e) => self.convert_var_or_app(e, con),
            mnode::Expr::Universe(e) => self.convert_universe(e),
        }
    }

    fn convert_let(
        &mut self,
        expr: &mnode::Let,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_ind(
        &mut self,
        expr: &mnode::Ind,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_fun(
        &mut self,
        expr: &mnode::Fun,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_aind(
        &mut self,
        expr: &mnode::Aind,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_vcon(
        &mut self,
        expr: &mnode::Vcon,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_match(
        &mut self,
        expr: &mnode::Match,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_afun(
        &mut self,
        expr: &mnode::Afun,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_for(
        &mut self,
        expr: &mnode::For,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_var_or_app(
        &mut self,
        expr: &mnode::VarOrApp,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::VarOrApp::Var(e) => self.convert_var(e, con),
            mnode::VarOrApp::App(e) => self.convert_app(e, con),
        }
    }

    fn convert_var(
        &mut self,
        expr: &mnode::Ident,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_app(
        &mut self,
        expr: &mnode::App,
        con: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
    }

    fn convert_universe(
        &mut self,
        expr: &mnode::UniverseLiteral,
    ) -> Result<znode::Expr, SemanticError> {
        Ok(self.make_universe(znode::UniverseNode {
            level: UniverseLevel(expr.level),
        }))
    }
}

impl MayConverter {
    fn make_universe(&mut self, node: znode::UniverseNode) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        znode::Expr::Universe(hashed)
    }
}
