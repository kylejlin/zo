use crate::syntax_tree::ast::*;

use std::rc::Rc;

pub trait DebReplacer {
    fn replace_deb(&self, original: RcSemHashed<DebNode>, cutoff: usize) -> Expr;
}

/// Replaces `0` with the last element of in `new_exprs`,
/// `1` with the second to last element,
/// and so on.
/// Free variables that are not replaced by an element of
/// `new_exprs` will be downshifted by the length of `new_exprs`.
pub struct DebDownshiftSubstituter<'a> {
    pub new_exprs: &'a [Expr],
}

impl DebReplacer for DebDownshiftSubstituter<'_> {
    fn replace_deb(&self, original: RcSemHashed<DebNode>, cutoff: usize) -> Expr {
        if original.value.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        let adjusted = original.value.deb.0 - cutoff;
        let new_exprs_len = self.new_exprs.len();
        if adjusted < new_exprs_len {
            let unshifted_new_expr = self.new_exprs[new_exprs_len - 1 - adjusted].clone();
            return unshifted_new_expr.replace_debs(&DebUpshifter(cutoff), 0);
        }

        let shifted = Deb(original.value.deb.0 - new_exprs_len);
        Expr::Deb(Rc::new(Hashed::new(DebNode { deb: shifted })))
    }
}

pub struct DebUpshifter(pub usize);

impl DebReplacer for DebUpshifter {
    fn replace_deb(&self, original: RcSemHashed<DebNode>, cutoff: usize) -> Expr {
        if original.value.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        Expr::Deb(Rc::new(Hashed::new(DebNode {
            deb: Deb(original.value.deb.0 + self.0),
        })))
    }
}

pub trait ReplaceDebs {
    type Output;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output;
}

impl<T> ReplaceDebs for IndepRcSemHashedVec<T>
where
    T: ReplaceDebs<Output = T> + Clone,
    Vec<T>: HashWithAlgorithm<SemanticHashAlgorithm>,
{
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let vec = self
            .0
            .value
            .iter()
            .cloned()
            .map(|item| item.replace_debs(replacer, cutoff))
            .collect();
        Independent(rc_sem_hashed(vec))
    }
}
impl<T> ReplaceDebs for DepRcSemHashedVec<T>
where
    T: ReplaceDebs<Output = T> + Clone,
    Vec<T>: HashWithAlgorithm<SemanticHashAlgorithm>,
{
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let vec = self
            .0
            .value
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, item)| item.replace_debs(replacer, cutoff + index))
            .collect();
        Dependent(rc_sem_hashed(vec))
    }
}

impl ReplaceDebs for Expr {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        match self {
            Expr::Ind(o) => Expr::Ind(o.replace_debs(replacer, cutoff)),
            Expr::Vcon(o) => Expr::Vcon(o.replace_debs(replacer, cutoff)),
            Expr::Match(o) => Expr::Match(o.replace_debs(replacer, cutoff)),
            Expr::Fun(o) => Expr::Fun(o.replace_debs(replacer, cutoff)),
            Expr::App(o) => Expr::App(o.replace_debs(replacer, cutoff)),
            Expr::For(o) => Expr::For(o.replace_debs(replacer, cutoff)),
            Expr::Deb(o) => replacer.replace_deb(o, cutoff),
            Expr::Universe(_) => self,
        }
    }
}

impl ReplaceDebs for RcSemHashed<Ind> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.value;
        Rc::new(Hashed::new(Ind {
            name: original.name.clone(),
            universe_level: original.universe_level,
            index_types: original.index_types.clone().replace_debs(replacer, cutoff),
            vcon_defs: original
                .vcon_defs
                .clone()
                .replace_debs(replacer, cutoff + 1),
        }))
    }
}

impl ReplaceDebs for VconDef {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        VconDef {
            param_types: self.param_types.clone().replace_debs(replacer, cutoff),
            index_args: self
                .index_args
                .clone()
                .replace_debs(replacer, cutoff + self.param_types.value.len()),
        }
    }
}

impl ReplaceDebs for RcSemHashed<Vcon> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.value;
        Rc::new(Hashed::new(Vcon {
            ind: original.ind.clone().replace_debs(replacer, cutoff),
            vcon_index: original.vcon_index,
        }))
    }
}

impl ReplaceDebs for RcSemHashed<Match> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.value;
        Rc::new(Hashed::new(Match {
            matchee: original.matchee.clone().replace_debs(replacer, cutoff),
            return_type: original.return_type.clone().replace_debs(replacer, cutoff),
            cases: original.cases.clone().replace_debs(replacer, cutoff),
        }))
    }
}

impl ReplaceDebs for MatchCase {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        MatchCase {
            arity: self.arity,
            return_val: self.return_val.replace_debs(replacer, cutoff + self.arity),
        }
    }
}

impl ReplaceDebs for RcSemHashed<Fun> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.value;
        Rc::new(Hashed::new(Fun {
            decreasing_index: original.decreasing_index,
            param_types: original.param_types.clone().replace_debs(replacer, cutoff),
            return_type: original
                .return_type
                .clone()
                .replace_debs(replacer, cutoff + original.param_types.value.len()),
            return_val: original
                .return_val
                .clone()
                .replace_debs(replacer, cutoff + original.param_types.value.len() + 1),
        }))
    }
}

impl ReplaceDebs for RcSemHashed<App> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.value;
        Rc::new(Hashed::new(App {
            callee: original.callee.clone().replace_debs(replacer, cutoff),
            args: original.args.clone().replace_debs(replacer, cutoff),
        }))
    }
}

impl ReplaceDebs for RcSemHashed<For> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.value;
        Rc::new(Hashed::new(For {
            param_types: original.param_types.clone().replace_debs(replacer, cutoff),
            return_type: original
                .return_type
                .clone()
                .replace_debs(replacer, cutoff + original.param_types.value.len()),
        }))
    }
}

impl ReplaceDebs for RcSemHashed<DebNode> {
    type Output = Expr;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        replacer.replace_deb(self, cutoff)
    }
}

impl ReplaceDebs for RcSemHashed<UniverseNode> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, _: &R, _: usize) -> Self::Output {
        self
    }
}
