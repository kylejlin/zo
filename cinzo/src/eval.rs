use crate::{ast::*, deb_shift_cache::DebShiftCache, nohash_hashmap::NoHashHashMap};

#[derive(Clone, Debug, Default)]
pub struct Caches {
    pub eval_cache: NoHashHashMap<Digest, Result<Expr, EvalError>>,
    pub deb_shift_cache: DebShiftCache,
}

#[derive(Clone, Debug)]
pub enum EvalError {}

pub fn eval_with_caches(expr: Expr, caches: &mut Caches) -> Result<Expr, EvalError> {
    todo!()
}
