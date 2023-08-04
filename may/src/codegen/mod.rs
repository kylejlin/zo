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

pub mod context;
pub use context::*;

mod convert_param_defs_to_context_extension;
use convert_param_defs_to_context_extension::*;

pub mod error;
pub use error::*;

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
        let val = self.convert(&expr.val, context)?;

        let val_singleton = [UnshiftedEntry {
            key: &expr.name.value,
            val,
            defines_deb: false,
        }];
        let extended_context = Context::Snoc(&context, &val_singleton);

        self.convert(&expr.next_val, extended_context)
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
        let matchee = self.convert(&expr.matchee, context)?;

        let extension =
            self.convert_return_arity_clause_to_context_extension(&expr.return_arity)?;
        let context_with_return_params = Context::Snoc(&context, &extension);
        let return_type = self.convert(&expr.return_type, context_with_return_params)?;

        let cases = self.convert_match_cases(&expr.cases, context)?;

        Ok(self.cache_match(znode::Match {
            matchee,
            return_type,
            cases,
        }))
    }

    fn convert_match_cases(
        &mut self,
        cases: &mnode::ZeroOrMoreMatchCases,
        context: Context,
    ) -> Result<RcHashedVec<znode::MatchCase>, SemanticError> {
        let mut cases = convert_match_case_snoc_to_vec(cases);
        cases.sort_unstable_by(|a, b| a.name.value.cmp(&b.name.value));
        self.convert_ordered_match_cases(&cases, context)
    }

    fn convert_ordered_match_cases(
        &mut self,
        cases: &[&mnode::MatchCase],
        context: Context,
    ) -> Result<RcHashedVec<znode::MatchCase>, SemanticError> {
        let v: Vec<znode::MatchCase> = cases
            .into_iter()
            .map(|case| self.convert_match_case(case, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(bypass_cache_and_rc_hash(v))
    }

    fn convert_match_case(
        &mut self,
        case: &mnode::MatchCase,
        context: Context,
    ) -> Result<znode::MatchCase, SemanticError> {
        let arity = case.params.len();

        let extension = self.convert_match_case_params_to_context_extension(&case.params);
        let context_with_params = Context::Snoc(&context, &extension);

        let return_val = self.convert(&case.return_val, context_with_params)?;

        Ok(znode::MatchCase { arity, return_val })
    }

    fn convert_afun(
        &mut self,
        expr: &mnode::Afun,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, decreasing_index) = self
            .convert_param_defs_to_context_extension(
                &expr.innards.params.params,
                context,
                AtMostOneDash::default(),
            )?;
        let context_with_params = Context::Snoc(&context, &extension);

        let return_type = self.convert(&expr.innards.return_type, context_with_params)?;

        let recursive_fun_param_singleton =
            [self.get_deb_defining_entry(expr.name.val_or_underscore())];

        let context_with_recursive_fun_param =
            Context::Snoc(&context_with_params, &recursive_fun_param_singleton);

        let return_val =
            self.convert(&expr.innards.return_val, context_with_recursive_fun_param)?;

        let param_types = self.cache_expr_vec(param_types);

        Ok(self.cache_fun(znode::Fun {
            decreasing_index,
            param_types,
            return_type,
            return_val,
        }))
    }

    fn convert_for(
        &mut self,
        expr: &mnode::For,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, ()) =
            self.convert_param_defs_to_context_extension(&expr.params.params, context, ForbidDash)?;
        let extended_context = Context::Snoc(&context, &extension);
        let return_type = self.convert(&expr.return_type, extended_context)?;

        let param_types = self.cache_expr_vec(param_types);

        Ok(self.cache_for(znode::For {
            param_types,
            return_type,
        }))
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

    fn cache_match(&mut self, node: znode::Match) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Match(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    fn cache_fun(&mut self, node: znode::Fun) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Fun(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
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

    fn cache_for(&mut self, node: znode::For) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::For(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    fn cache_deb(&mut self, node: znode::DebNode) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Deb(hashed);
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

impl mnode::IdentOrUnderscore {
    /// If `self` is an identifier,
    /// its value is returned.
    /// Otherwise, `self` is an underscore,
    /// in which case the string `"_"` is returned.
    fn val(&self) -> &str {
        match self {
            Self::Ident(ident) => &ident.value,
            Self::Underscore(_) => "_",
        }
    }
}

impl mnode::OptIdent {
    /// If `self` is `Some(ident)`,
    /// then `ident` is returned.
    /// Otherwise, `"_"` is returned.
    fn val_or_underscore(&self) -> &str {
        match self {
            Self::Some(ident) => &ident.value,
            Self::None => "_",
        }
    }
}

impl mnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores {
    fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Some(parenthesized) => parenthesized.idents.len(),
        }
    }
}

impl mnode::CommaSeparatedIdentsOrUnderscores {
    fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Snoc(rdc, _) => rdc.len() + 1,
        }
    }
}

fn convert_match_case_snoc_to_vec(cases: &mnode::ZeroOrMoreMatchCases) -> Vec<&mnode::MatchCase> {
    match cases {
        mnode::ZeroOrMoreMatchCases::Nil => vec![],
        mnode::ZeroOrMoreMatchCases::Snoc(rdc, rac) => {
            let mut rdc = convert_match_case_snoc_to_vec(rdc);
            rdc.push(rac);
            rdc
        }
    }
}
