use std::rc::Rc;

use crate::ast::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Normalized<T>(pub(in crate::eval) T);

pub type NormalForm = Normalized<Expr>;

impl From<NormalForm> for Expr {
    fn from(nf: NormalForm) -> Self {
        nf.0
    }
}

impl<T> Normalized<T> {
    pub fn into_raw(self) -> T {
        self.0
    }

    pub fn raw(&self) -> &T {
        &self.0
    }
}

impl<T> Normalized<&T> {
    pub fn cloned(self) -> Normalized<T>
    where
        T: Clone,
    {
        Normalized(self.0.clone())
    }
}

impl<T> Normalized<&[T]> {
    pub fn get(&self, index: usize) -> Option<Normalized<&T>> {
        self.0.get(index).map(Normalized)
    }

    /// A panicking version of `get`.
    pub fn index(&self, index: usize) -> Normalized<&T> {
        Normalized(&self.0[index])
    }
}

impl<T> Normalized<Vec<T>> {
    pub fn get(&self, index: usize) -> Option<Normalized<&T>> {
        self.0.get(index).map(Normalized)
    }

    /// A panicking version of `get`.
    pub fn index(&self, index: usize) -> Normalized<&T> {
        Normalized(&self.0[index])
    }
}

impl<T> FromIterator<Normalized<T>> for Normalized<Vec<T>> {
    fn from_iter<I: IntoIterator<Item = Normalized<T>>>(iter: I) -> Self {
        Normalized(iter.into_iter().map(Normalized::into_raw).collect())
    }
}

impl<T> Normalized<Vec<T>> {
    pub fn as_slice(&self) -> Normalized<&[T]> {
        Normalized(&self.0)
    }

    pub fn transpose_from_vec(v: Vec<Normalized<T>>) -> Normalized<Vec<T>> {
        Normalized(v.into_iter().map(Normalized::into_raw).collect())
    }

    pub fn push(&mut self, item: Normalized<T>) {
        self.0.push(item.into_raw())
    }
}

impl NormalForm {
    pub fn universe(universe: UniverseNode) -> Self {
        Normalized(Expr::Universe(Rc::new(Hashed::new(universe))))
    }
}

impl Normalized<App> {
    pub fn ind_app(
        callee: Normalized<RcHashed<Ind>>,
        args: Normalized<RcHashed<Box<[Expr]>>>,
    ) -> Self {
        Normalized(App {
            callee: Expr::Ind(callee.into_raw()),
            args: args.into_raw(),
        })
    }

    pub fn collapse_if_nullary(self) -> NormalForm {
        Normalized(self.0.collapse_if_nullary())
    }
}
