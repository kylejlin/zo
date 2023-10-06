use super::*;

impl MayConverter {
    pub(crate) fn cache_expr(&mut self, node: znode::Expr) -> znode::Expr {
        let digest = node.digest();
        if let Some(existing) = self.znode_cache.get(digest) {
            return existing.clone();
        }

        self.znode_cache.insert(digest.clone(), node.clone());
        node
    }

    pub(crate) fn cache_ind(&mut self, node: znode::Ind) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Ind(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_vcon(&mut self, node: znode::Vcon) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Vcon(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_match(&mut self, node: znode::Match) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Match(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_fun(&mut self, node: znode::Fun) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Fun(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_app(&mut self, node: znode::App) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::App(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_for(&mut self, node: znode::For) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::For(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_deb(&mut self, node: znode::DebNode) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Deb(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }

    pub(crate) fn cache_universe(&mut self, node: znode::UniverseNode) -> znode::Expr {
        let hashed = bypass_cache_and_rc_hash(node);

        if let Some(existing) = self.znode_cache.get(&hashed.digest) {
            return existing.clone();
        }

        let digest = hashed.digest.clone();
        let out = znode::Expr::Universe(hashed);
        self.znode_cache.insert(digest, out.clone());
        out
    }
}

impl MayConverter {
    pub(crate) fn cache_expr_vec(&mut self, node: Vec<znode::Expr>) -> RcHashedVec<znode::Expr> {
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

impl MayConverter {
    pub(crate) fn cache_string_value(&mut self, val: StringValue) -> Rc<StringValue> {
        let val = Rc::new(val);

        if let Some(existing) = self.str_val_cache.get(&val) {
            return existing.clone();
        }

        self.str_val_cache.insert(val.clone());
        val
    }
}
