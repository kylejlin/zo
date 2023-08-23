// TODO: I think we can make this module generic
// over all AST families.
// However, it's a hassle, and not a high priority.
// So I'll do it later.

use crate::syntax_tree::ast::prelude::ast::*;

use std::{hash::Hash, rc::Rc};

pub trait DebReplacer<A: AstFamily> {
    fn replace_deb(&self, original: RcHashed<DebNode<A>>, cutoff: usize) -> Expr<A>;
}

// TODO: Make replace_deb take an access token.
// The current implementation is not sound,
// since someone could implement `DebReplacer`
// but not require a token.

/// We allow AST families to opt-in to deb replacement.
/// Sometimes, an AST family wants to opt-in to deb replacement,
/// but only within a certain module.
/// However, Rust's `pub` system does not let us implement a trait
/// only for a certain module.
///
/// To get around this, we parameterize this trait by
/// an "access token" type `K`.
/// In order to perform deb replacement, you need to pass
/// an access token of the given type.
/// If you make `K` a type that is local to some module,
/// then that module effectively has control over deb replacement.
pub trait DebReplacableAstFamily<K: Copy>: AstFamily {}

/// Replaces `0` with the last element of in `new_exprs`,
/// `1` with the second to last element,
/// and so on.
/// Free variables that are not replaced by an element of
/// `new_exprs` will be downshifted by the length of `new_exprs`.
pub struct DebDownshiftSubstituter<'a, A: AstFamily> {
    pub new_exprs: &'a [Expr<A>],
}

impl<A: AstFamily> DebReplacer<A> for DebDownshiftSubstituter<'_, A> {
    fn replace_deb(&self, original: RcHashed<DebNode<A>>, cutoff: usize) -> Expr<A> {
        if original.hashee.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        let adjusted = original.hashee.deb.0 - cutoff;
        let new_exprs_len = self.new_exprs.len();
        if adjusted < new_exprs_len {
            let unshifted_new_expr = self.new_exprs[new_exprs_len - 1 - adjusted].clone();
            // TODO: This is a hack.
            // We should call the public `replace_debs`
            // instead of relying on this private method.
            // However, that would requires the shifter
            // to carry an access token, which is a small hassle.
            // We can refactor later.
            return unshifted_new_expr.replace_debs_impl(&DebUpshifter(cutoff), 0);
        }

        let shifted = Deb(original.hashee.deb.0 - new_exprs_len);
        Expr::Deb(Rc::new(Hashed::new(DebNode {
            deb: shifted,
            aux_data: original.hashee.aux_data.clone(),
        })))
    }
}

pub struct DebUpshifter(pub usize);

impl<A: AstFamily> DebReplacer<A> for DebUpshifter {
    fn replace_deb(&self, original: RcHashed<DebNode<A>>, cutoff: usize) -> Expr<A> {
        if original.hashee.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        Expr::Deb(Rc::new(Hashed::new(DebNode {
            deb: Deb(original.hashee.deb.0 + self.0),
            aux_data: original.hashee.aux_data.clone(),
        })))
    }
}

pub trait ReplaceDebs<K: Copy, A: DebReplacableAstFamily<K>> {
    type Output;

    fn replace_debs<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output
    where
        K: Default;

    fn replace_debs_using_access_token<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
        _token: K,
    ) -> Self::Output;
}

impl<K, A, T> ReplaceDebs<K, A> for T
where
    K: Copy,
    A: DebReplacableAstFamily<K>,
    T: ReplaceDebsImpl<A>,
{
    type Output = <T as ReplaceDebsImpl<A>>::Output;

    fn replace_debs<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output
    where
        K: Default,
    {
        self.replace_debs_impl(replacer, cutoff)
    }

    fn replace_debs_using_access_token<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
        _token: K,
    ) -> Self::Output {
        self.replace_debs_impl(replacer, cutoff)
    }
}

/// This trait should not be implemented outside the defining module.
pub trait ReplaceDebsImpl<A: AstFamily> {
    type Output;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output;
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
pub trait ReplaceDebsInEachItem<A: AstFamily> {
    fn replace_debs_with_constant_cutoff<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self;

    fn replace_debs_with_increasing_cutoff<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self;
}

impl<T, A: AstFamily> ReplaceDebsInEachItem<A> for RcHashedVec<T>
where
    T: ReplaceDebsImpl<A, Output = T> + Clone,
    Vec<T>: Hash,
{
    fn replace_debs_with_constant_cutoff<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        let shifted: Vec<T> = self
            .hashee
            .iter()
            .map(|item| item.clone().replace_debs_impl(replacer, cutoff))
            .collect();
        Rc::new(Hashed::new(shifted))
    }

    fn replace_debs_with_increasing_cutoff<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        let shifted: Vec<T> = self
            .hashee
            .iter()
            .enumerate()
            .map(|(index, item)| item.clone().replace_debs_impl(replacer, cutoff + index))
            .collect();
        Rc::new(Hashed::new(shifted))
    }
}

impl<T, A: AstFamily> ReplaceDebsInEachItem<A> for Vec<T>
where
    T: ReplaceDebsImpl<A, Output = T> + Clone,
{
    fn replace_debs_with_constant_cutoff<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        self.into_iter()
            .map(|item| item.replace_debs_impl(replacer, cutoff))
            .collect()
    }

    fn replace_debs_with_increasing_cutoff<R: DebReplacer<A>>(
        self,
        replacer: &R,
        cutoff: usize,
    ) -> Self {
        self.into_iter()
            .enumerate()
            .map(|(index, item)| item.replace_debs_impl(replacer, cutoff + index))
            .collect()
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for Expr<A> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        match self {
            Expr::Ind(o) => Expr::Ind(o.replace_debs_impl(replacer, cutoff)),
            Expr::Vcon(o) => Expr::Vcon(o.replace_debs_impl(replacer, cutoff)),
            Expr::Match(o) => Expr::Match(o.replace_debs_impl(replacer, cutoff)),
            Expr::Fun(o) => Expr::Fun(o.replace_debs_impl(replacer, cutoff)),
            Expr::App(o) => Expr::App(o.replace_debs_impl(replacer, cutoff)),
            Expr::For(o) => Expr::For(o.replace_debs_impl(replacer, cutoff)),
            Expr::Deb(o) => replacer.replace_deb(o, cutoff),
            Expr::Universe(_) => self,
        }
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<Ind<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
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

impl<A: AstFamily> ReplaceDebsImpl<A> for VconDef<A> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
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

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<Vcon<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(Vcon {
            ind: original.ind.clone().replace_debs_impl(replacer, cutoff),
            vcon_index: original.vcon_index,
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<Match<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(Match {
            matchee: original.matchee.clone().replace_debs_impl(replacer, cutoff),
            return_type_arity: original.return_type_arity,
            return_type: original
                .return_type
                .clone()
                .replace_debs_impl(replacer, cutoff + original.return_type_arity),
            cases: original
                .cases
                .clone()
                .replace_debs_with_constant_cutoff(replacer, cutoff),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for MatchCase<A> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        MatchCase {
            arity: self.arity,
            return_val: self
                .return_val
                .replace_debs_impl(replacer, cutoff + self.arity),
            aux_data: self.aux_data.clone(),
        }
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<Fun<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
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
                .replace_debs_impl(replacer, cutoff + original.param_types.hashee.len()),
            return_val: original
                .return_val
                .clone()
                .replace_debs_impl(replacer, cutoff + original.param_types.hashee.len() + 1),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<App<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(App {
            callee: original.callee.clone().replace_debs_impl(replacer, cutoff),
            args: original
                .args
                .clone()
                .replace_debs_with_constant_cutoff(replacer, cutoff),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<For<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        let original = &self.hashee;
        Rc::new(Hashed::new(For {
            param_types: original
                .param_types
                .clone()
                .replace_debs_with_increasing_cutoff(replacer, cutoff),
            return_type: original
                .return_type
                .clone()
                .replace_debs_impl(replacer, cutoff + original.param_types.hashee.len()),
            aux_data: original.aux_data.clone(),
        }))
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<DebNode<A>> {
    type Output = Expr<A>;

    fn replace_debs_impl<R: DebReplacer<A>>(self, replacer: &R, cutoff: usize) -> Self::Output {
        replacer.replace_deb(self, cutoff)
    }
}

impl<A: AstFamily> ReplaceDebsImpl<A> for RcHashed<UniverseNode<A>> {
    type Output = Self;

    fn replace_debs_impl<R: DebReplacer<A>>(self, _: &R, _: usize) -> Self::Output {
        self
    }
}
