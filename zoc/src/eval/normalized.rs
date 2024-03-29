use crate::syntax_tree::{ast::prelude::minimal_ast::*, replace_debs::*};

use std::{hash::Hash, ops::Deref, rc::Rc};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Normalized<T>(pub(in crate::eval) T);

pub type NormalForm = Normalized<Expr>;

impl<T> Normalized<T> {
    pub fn into_raw(self) -> T {
        self.0
    }

    pub fn raw(&self) -> &T {
        &self.0
    }

    pub fn as_ref(&self) -> Normalized<&T> {
        Normalized(&self.0)
    }

    /// Shorthand for `self.as_ref().derefed()`.
    pub fn to_derefed(&self) -> Normalized<&T::Target>
    where
        T: Deref,
    {
        self.as_ref().derefed()
    }
}

impl<'a, T> Normalized<&'a T> {
    pub fn cloned(self) -> Normalized<T>
    where
        T: Clone,
    {
        Normalized(self.0.clone())
    }

    pub fn derefed(self) -> Normalized<&'a T::Target>
    where
        T: Deref,
    {
        Normalized(self.0.deref())
    }

    pub fn convert_ref<U: ?Sized>(self) -> Normalized<&'a U>
    where
        T: AsRef<U>,
    {
        Normalized(self.0.as_ref())
    }
}

impl<T> Normalized<RcHashed<T>> {
    /// Shorthand for `self.as_ref().hashee()`.
    pub fn to_hashee(&self) -> Normalized<&T> {
        self.as_ref().hashee()
    }
}
impl<'a, T> Normalized<&'a RcHashed<T>> {
    pub fn hashee(self) -> Normalized<&'a T> {
        Normalized(&self.0.hashee)
    }
}

impl<T> Normalized<T>
where
    T: Hash,
{
    pub fn into_rc_hashed(self) -> Normalized<RcHashed<T>> {
        Normalized(rc_hashed(self.0))
    }
}

impl<T> Normalized<Vec<T>> {
    pub fn new() -> Self {
        Self(vec![])
    }
}
impl<T> Normalized<[T; 0]> {
    pub fn new() -> Self {
        Self([])
    }
}
impl<T> Normalized<[T; 1]> {
    pub fn new(a: Normalized<T>) -> Self {
        Self([a.0])
    }
}

impl<T, S> Normalized<S>
where
    S: Deref<Target = [T]>,
{
    /// Shorthand for `self.as_ref().get(index)`.
    pub fn get_ref(&self, index: usize) -> Option<Normalized<&T>> {
        self.as_ref().get(index)
    }

    /// Shorthand for `self.as_ref().index(index)`.
    pub fn index_ref(&self, index: usize) -> Normalized<&T> {
        self.as_ref().index(index)
    }

    /// Shorthand for `self.as_ref().into_vec()`.
    pub fn to_vec(&self) -> Vec<Normalized<T>>
    where
        T: Clone,
    {
        self.as_ref().into_vec()
    }
}
impl<'a, T, S> Normalized<&'a S>
where
    S: Deref<Target = [T]>,
{
    pub fn get(self, index: usize) -> Option<Normalized<&'a T>> {
        self.0.deref().get(index).map(Normalized)
    }

    /// A panicking version of `get`.
    pub fn index(self, index: usize) -> Normalized<&'a T> {
        Normalized(&self.0[index])
    }

    pub fn into_vec(self) -> Vec<Normalized<T>>
    where
        T: Clone,
    {
        self.0.iter().cloned().map(Normalized).collect()
    }
}

impl<T> Normalized<Vec<T>> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, item: Normalized<T>) {
        self.0.push(item.into_raw())
    }
}

impl<T> FromIterator<Normalized<T>> for Normalized<Vec<T>> {
    fn from_iter<I: IntoIterator<Item = Normalized<T>>>(iter: I) -> Self {
        Self(iter.into_iter().map(Normalized::into_raw).collect())
    }
}

impl NormalForm {
    pub fn universe(universe: UniverseNode) -> Self {
        Normalized(Expr::Universe(Rc::new(Hashed::new(universe))))
    }
}

impl NormalForm {
    pub fn ind_or_ind_app(
        self,
    ) -> Option<(Normalized<RcHashed<Ind>>, Normalized<RcHashedVec<Expr>>)> {
        match self.0 {
            Expr::Ind(ind) => Some((Normalized(ind), Normalized(Rc::new(Hashed::new(vec![]))))),

            Expr::App(app) => match &app.hashee.callee {
                Expr::Ind(ind) => {
                    Some((Normalized(ind.clone()), Normalized(app.hashee.args.clone())))
                }
                _other_callee => None,
            },

            _ => None,
        }
    }

    /// If `self` is a `for` expression,
    /// this function returns the parameter types of said `for` expression.
    /// Otherwise, it returns `None`.
    pub fn for_param_types(self) -> Option<Normalized<RcHashedVec<Expr>>> {
        match self.0 {
            Expr::For(for_) => Some(Normalized(for_.hashee.param_types.clone())),
            _ => None,
        }
    }

    /// If `self` is a `for` expression,
    /// this function returns the parameter types of said `for` expression.
    /// Otherwise, it returns an empty (RC hashed) vector.
    pub fn for_param_types_or_empty_vec(self) -> Normalized<RcHashedVec<Expr>> {
        self.for_param_types()
            .unwrap_or_else(|| Normalized::<Vec<_>>::new().into_rc_hashed())
    }

    /// If `self` is a `for` expression,
    /// this function returns the return type of said `for` expression.
    /// Otherwise, it returns `None`.
    pub fn for_return_type(self) -> Option<NormalForm> {
        match self.0 {
            Expr::For(for_) => Some(Normalized(for_.hashee.return_type.clone())),
            _ => None,
        }
    }

    /// If `self` is a `for` expression,
    /// this function returns the return type of said `for` expression.
    /// Otherwise, it returns `self`.
    pub fn for_return_type_or_self(self) -> NormalForm {
        self.clone().for_return_type().unwrap_or(self)
    }

    /// If `self` is a `app` expression,
    /// this function returns the arguments of said `app` expression.
    /// Otherwise, it returns `None`.
    pub fn app_args(self) -> Option<Normalized<RcHashedVec<Expr>>> {
        match self.0 {
            Expr::App(app) => Some(Normalized(app.hashee.args.clone())),
            _ => None,
        }
    }

    /// If `self` is a `app` expression,
    /// this function returns the arguments of said `app` expression.
    /// Otherwise, it returns an empty (RC hashed) vector.
    pub fn app_args_or_empty_vec(self) -> Normalized<RcHashedVec<Expr>> {
        self.app_args()
            .unwrap_or_else(|| Normalized::<Vec<_>>::new().into_rc_hashed())
    }
}

impl<'a> Normalized<&'a Ind> {
    pub fn index_types(self) -> Normalized<&'a RcHashedVec<Expr>> {
        Normalized(&self.0.index_types)
    }

    pub fn vcon_defs(self) -> Normalized<&'a RcHashedVec<VconDef>> {
        Normalized(&self.0.vcon_defs)
    }
}

impl<'a> Normalized<&'a VconDef> {
    pub fn param_types(self) -> Normalized<&'a RcHashedVec<Expr>> {
        Normalized(&self.0.param_types)
    }

    pub fn index_args(self) -> Normalized<&'a RcHashedVec<Expr>> {
        Normalized(&self.0.index_args)
    }
}

impl Normalized<App> {
    pub fn app_with_ind_callee(
        callee: Normalized<RcHashed<Ind>>,
        args: Normalized<RcHashedVec<Expr>>,
    ) -> Self {
        Normalized(App {
            callee: Expr::Ind(callee.into_raw()),
            args: args.into_raw(),
            aux_data: (),
        })
    }

    pub fn collapse_if_nullary(self) -> NormalForm {
        Normalized(self.0.collapse_if_nullary())
    }
}

impl<'a> Normalized<&'a For> {
    pub fn param_types(self) -> Normalized<&'a RcHashedVec<Expr>> {
        Normalized(&self.0.param_types)
    }

    pub fn return_type(self) -> Normalized<&'a Expr> {
        Normalized(&self.0.return_type)
    }
}

impl Normalized<For> {
    pub fn for_(param_types: Normalized<RcHashedVec<Expr>>, return_type: NormalForm) -> Self {
        Normalized(For {
            param_types: param_types.into_raw(),
            return_type: return_type.into_raw(),
            aux_data: (),
        })
    }

    pub fn collapse_if_nullary(self) -> NormalForm {
        Normalized(self.0.collapse_if_nullary())
    }
}

impl NormalForm {
    /// Returns an expression of the form
    /// ```zolike
    /// (@capp (vcon <ind> <vcon_index>) (
    ///     <arg_count - 1>
    ///     <arg_count - 2>
    ///     ...
    ///     0
    /// ))
    /// ```
    /// where `arg_count` is the number of params
    /// that the `vcon_index`th vcon def has.
    pub fn vcon_capp_of_descending_debs(
        ind: Normalized<RcHashed<Ind>>,
        vcon_index: usize,
    ) -> NormalForm {
        let arg_count = ind.raw().hashee.vcon_defs.hashee[vcon_index]
            .param_types
            .hashee
            .len();

        let vcon = Vcon {
            ind: ind.into_raw(),
            vcon_index,
            aux_data: (),
        };
        let args: Vec<Expr> = (0..arg_count)
            .into_iter()
            .rev()
            .map(|i| {
                DebNode {
                    deb: Deb(i),
                    aux_data: (),
                }
                .into()
            })
            .collect();
        let capp = App {
            callee: vcon.into(),
            args: rc_hashed(args),
            aux_data: (),
        }
        .collapse_if_nullary();
        Normalized(capp)
    }

    /// Returns an expression of the form
    /// ```zolike
    /// (@capp <ind> (
    ///     <index_count - 1>
    ///     <index_count - 2>
    ///     ...
    ///     0
    /// ))
    /// ```
    /// where `index_count` is the number of indices
    /// that the `ind` has.
    pub fn ind_capp_of_descending_debs(ind: Normalized<RcHashed<Ind>>) -> NormalForm {
        let index_count = ind.raw().hashee.index_types.hashee.len();

        let args: Vec<Expr> = (0..index_count)
            .into_iter()
            .rev()
            .map(|i| {
                DebNode {
                    deb: Deb(i),
                    aux_data: (),
                }
                .into()
            })
            .collect();
        let capp = App {
            callee: ind.into_raw().into(),
            args: rc_hashed(args),
            aux_data: (),
        }
        .collapse_if_nullary();
        Normalized(capp)
    }
}

impl<T: ReplaceDebs> Normalized<T> {
    pub fn upshift(self, amount: usize, cutoff: usize) -> Normalized<T::Output> {
        Normalized(self.0.replace_debs(&DebUpshifter(amount), cutoff))
    }
}

impl<T: ReplaceDebsInEachItem> Normalized<T> {
    pub fn upshift_with_constant_cutoff(self, amount: usize) -> Self {
        Normalized(
            self.0
                .replace_debs_with_constant_cutoff(&DebUpshifter(amount), 0),
        )
    }

    pub fn upshift_with_increasing_cutoff(self, amount: usize) -> Self {
        Normalized(
            self.0
                .replace_debs_with_increasing_cutoff(&DebUpshifter(amount), 0),
        )
    }

    pub fn replace_deb0_with_ind_with_constant_cutoff(
        self,
        ind: Normalized<RcHashed<Ind>>,
        cutoff: usize,
    ) -> Self {
        let ind_singleton: [Expr; 1] = [ind.raw().clone().into()];
        let substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };
        Normalized(
            self.0
                .replace_debs_with_constant_cutoff(&substituter, cutoff),
        )
    }

    pub fn replace_deb0_with_ind_with_increasing_cutoff(
        self,
        ind: Normalized<RcHashed<Ind>>,
        cutoff: usize,
    ) -> Self {
        let ind_singleton: [Expr; 1] = [ind.raw().clone().into()];
        let substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };
        Normalized(
            self.0
                .replace_debs_with_increasing_cutoff(&substituter, cutoff),
        )
    }
}

impl NormalForm {
    pub fn try_into_ind(self) -> Result<Normalized<RcHashed<Ind>>, NormalForm> {
        match self.0 {
            Expr::Ind(ind) => Ok(Normalized(ind)),
            _ => Err(self),
        }
    }

    pub fn try_into_vcon(self) -> Result<Normalized<RcHashed<Vcon>>, NormalForm> {
        match self.0 {
            Expr::Vcon(vcon) => Ok(Normalized(vcon)),
            _ => Err(self),
        }
    }

    pub fn try_into_match(self) -> Result<Normalized<RcHashed<Match>>, NormalForm> {
        match self.0 {
            Expr::Match(m) => Ok(Normalized(m)),
            _ => Err(self),
        }
    }

    pub fn try_into_fun(self) -> Result<Normalized<RcHashed<Fun>>, NormalForm> {
        match self.0 {
            Expr::Fun(f) => Ok(Normalized(f)),
            _ => Err(self),
        }
    }

    pub fn try_into_app(self) -> Result<Normalized<RcHashed<App>>, NormalForm> {
        match self.0 {
            Expr::App(a) => Ok(Normalized(a)),
            _ => Err(self),
        }
    }

    pub fn try_into_for(self) -> Result<Normalized<RcHashed<For>>, NormalForm> {
        match self.0 {
            Expr::For(f) => Ok(Normalized(f)),
            _ => Err(self),
        }
    }

    pub fn try_into_deb(self) -> Result<Normalized<RcHashed<DebNode>>, NormalForm> {
        match self.0 {
            Expr::Deb(d) => Ok(Normalized(d)),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<Normalized<RcHashed<UniverseNode>>, NormalForm> {
        match self.0 {
            Expr::Universe(u) => Ok(Normalized(u)),
            _ => Err(self),
        }
    }
}

impl From<Normalized<Ind>> for NormalForm {
    fn from(e: Normalized<Ind>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<Vcon>> for NormalForm {
    fn from(e: Normalized<Vcon>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<Match>> for NormalForm {
    fn from(e: Normalized<Match>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<Fun>> for NormalForm {
    fn from(e: Normalized<Fun>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<App>> for NormalForm {
    fn from(e: Normalized<App>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<For>> for NormalForm {
    fn from(e: Normalized<For>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<DebNode>> for NormalForm {
    fn from(e: Normalized<DebNode>) -> Self {
        Normalized(e.0.into())
    }
}
impl From<Normalized<UniverseNode>> for NormalForm {
    fn from(e: Normalized<UniverseNode>) -> Self {
        Normalized(e.0.into())
    }
}

pub(in crate::eval) use unchecked::*;
mod unchecked {
    use super::*;

    pub trait WrapInNormalized: Sized {
        fn wrap_in_normalized(self) -> Normalized<Self>;
    }

    impl<T> WrapInNormalized for T {
        fn wrap_in_normalized(self) -> Normalized<T> {
            Normalized(self)
        }
    }

    pub trait ConvertToExprAndWrapInNormalized {
        fn convert_to_expr_and_wrap_in_normalized(self) -> NormalForm;
    }

    impl<T> ConvertToExprAndWrapInNormalized for T
    where
        T: Into<Expr>,
    {
        fn convert_to_expr_and_wrap_in_normalized(self) -> NormalForm {
            Normalized(self.into())
        }
    }

    pub trait RcHashAndWrapInNormalized: Sized {
        fn rc_hash_and_wrap_in_normalized(self) -> Normalized<RcHashed<Self>>;
    }

    impl<T> RcHashAndWrapInNormalized for T
    where
        T: Hash,
    {
        fn rc_hash_and_wrap_in_normalized(self) -> Normalized<RcHashed<Self>> {
            Normalized(rc_hashed(self))
        }
    }
}
