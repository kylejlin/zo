// TODO: I think we can make this module generic
// over all AST families.
// However, it's a hassle, and not a high priority.
// So I'll do it later.

use crate::syntax_tree::minimal_ast::*;

use std::{hash::Hash, rc::Rc};

pub trait DebReplacer {
    fn replace_deb(&self, original: RcHashed<DebNode>, cutoff: usize) -> Expr;
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
    fn replace_deb(&self, original: RcHashed<DebNode>, cutoff: usize) -> Expr {
        if original.hashee.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        let adjusted = original.hashee.deb.0 - cutoff;
        let new_exprs_len = self.new_exprs.len();
        if adjusted < new_exprs_len {
            let unshifted_new_expr = self.new_exprs[new_exprs_len - 1 - adjusted].clone();
            return unshifted_new_expr.replace_debs(&DebUpshifter(cutoff), 0);
        }

        let shifted = Deb(original.hashee.deb.0 - new_exprs_len);
        Expr::Deb(Rc::new(Hashed::new(DebNode {
            deb: shifted,
            aux_data: original.hashee.aux_data.clone(),
        })))
    }
}

pub struct DebUpshifter(pub usize);

impl DebReplacer for DebUpshifter {
    fn replace_deb(&self, original: RcHashed<DebNode>, cutoff: usize) -> Expr {
        if original.hashee.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        Expr::Deb(Rc::new(Hashed::new(DebNode {
            deb: Deb(original.hashee.deb.0 + self.0),
            aux_data: original.hashee.aux_data.clone(),
        })))
    }
}

pub trait ReplaceDebs {
    type Output;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output;
}

/// Some collections represent independent items,
/// whereas others represent a dependent items.
/// This requires shifting each item with a constant cutoff
/// and increasing cutoff, respectively.
///
/// We could represent this semantic difference at the type level
/// (e.g. `RcHashedIndependentVec` vs `RcHashedDependentVec`),
/// but this requires a lot of boilerplate.
/// So instead, we leave it up to the user to call the correct method.
pub trait ReplaceDebsInEachItem {
    fn replace_debs_with_constant_cutoff<R: DebReplacer>(self, replacer: &R, cutoff: usize)
        -> Self;

    fn replace_debs_with_increasing_cutoff<R: DebReplacer>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self;
}

impl<T> ReplaceDebsInEachItem for RcHashedVec<T>
where
    T: ReplaceDebs<Output = T> + Clone,
    Vec<T>: Hash,
{
    fn replace_debs_with_constant_cutoff<R: DebReplacer>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        let shifted: Vec<T> = self
            .hashee
            .iter()
            .map(|item| item.clone().replace_debs(replacer, cutoff))
            .collect();
        Rc::new(Hashed::new(shifted))
    }

    fn replace_debs_with_increasing_cutoff<R: DebReplacer>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        let shifted: Vec<T> = self
            .hashee
            .iter()
            .enumerate()
            .map(|(index, item)| item.clone().replace_debs(replacer, cutoff + index))
            .collect();
        Rc::new(Hashed::new(shifted))
    }
}

impl<T> ReplaceDebsInEachItem for Vec<T>
where
    T: ReplaceDebs<Output = T> + Clone,
{
    fn replace_debs_with_constant_cutoff<R: DebReplacer>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        self.into_iter()
            .map(|item| item.replace_debs(replacer, cutoff))
            .collect()
    }

    fn replace_debs_with_increasing_cutoff<R: DebReplacer>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        self.into_iter()
            .enumerate()
            .map(|(index, item)| item.replace_debs(replacer, cutoff + index))
            .collect()
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

impl ReplaceDebs for RcHashed<Ind> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(Ind {
            name: original.name.clone(),
            universe: original.universe,
            index_types: original
                .index_types
                .clone()
                .replace_debs_with_increasing_cutoff(replacer, cutoff),
            vcon_defs: original
                .vcon_defs
                .clone()
                .replace_debs_with_constant_cutoff(replacer, cutoff + 1),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl ReplaceDebs for VconDef {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        VconDef {
            param_types: self
                .param_types
                .clone()
                .replace_debs_with_increasing_cutoff(replacer, cutoff),
            index_args: self.index_args.clone().replace_debs_with_constant_cutoff(
                replacer,
                cutoff + self.param_types.hashee.len(),
            ),
            aux_data: self.aux_data.clone(),
        }
    }
}

impl ReplaceDebs for RcHashed<Vcon> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(Vcon {
            ind: original.ind.clone().replace_debs(replacer, cutoff),
            vcon_index: original.vcon_index,
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl ReplaceDebs for RcHashed<Match> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(Match {
            matchee: original.matchee.clone().replace_debs(replacer, cutoff),
            return_type_arity: original.return_type_arity,
            return_type: original
                .return_type
                .clone()
                .replace_debs(replacer, cutoff + original.return_type_arity),
            cases: original
                .cases
                .clone()
                .replace_debs_with_constant_cutoff(replacer, cutoff),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl ReplaceDebs for MatchCase {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        MatchCase {
            arity: self.arity,
            return_val: self.return_val.replace_debs(replacer, cutoff + self.arity),
            aux_data: self.aux_data.clone(),
        }
    }
}

impl ReplaceDebs for RcHashed<Fun> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(Fun {
            decreasing_index: original.decreasing_index,
            param_types: original
                .param_types
                .clone()
                .replace_debs_with_increasing_cutoff(replacer, cutoff),
            return_type: original
                .return_type
                .clone()
                .replace_debs(replacer, cutoff + original.param_types.hashee.len()),
            return_val: original
                .return_val
                .clone()
                .replace_debs(replacer, cutoff + original.param_types.hashee.len() + 1),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl ReplaceDebs for RcHashed<App> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(App {
            callee: original.callee.clone().replace_debs(replacer, cutoff),
            args: original
                .args
                .clone()
                .replace_debs_with_constant_cutoff(replacer, cutoff),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl ReplaceDebs for RcHashed<For> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(For {
            param_types: original
                .param_types
                .clone()
                .replace_debs_with_increasing_cutoff(replacer, cutoff),
            return_type: original
                .return_type
                .clone()
                .replace_debs(replacer, cutoff + original.param_types.hashee.len()),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl ReplaceDebs for RcHashed<DebNode> {
    type Output = Expr;

    fn replace_debs<R: DebReplacer>(self, replacer: &R, cutoff: usize) -> Self::Output {
        replacer.replace_deb(self, cutoff)
    }
}

impl ReplaceDebs for RcHashed<UniverseNode> {
    type Output = Self;

    fn replace_debs<R: DebReplacer>(self, _: &R, _: usize) -> Self::Output {
        self
    }
}
