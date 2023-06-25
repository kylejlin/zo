use crate::{ast::*, replace_debs::*};

use std::{ops::Deref, rc::Rc};

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

    pub fn to_derefed(&self) -> Normalized<&T::Target>
    where
        T: Deref,
    {
        Normalized(self.0.deref())
    }
}

impl<T> Normalized<&T> {
    pub fn cloned(self) -> Normalized<T>
    where
        T: Clone,
    {
        Normalized(self.0.clone())
    }

    pub fn derefed(&self) -> Normalized<&T::Target>
    where
        T: Deref,
    {
        Normalized(self.0.deref())
    }
}

impl<T> Normalized<RcHashed<T>> {
    pub fn without_digest(&self) -> Normalized<&T> {
        Normalized(&self.0.value)
    }
}

impl<T, S> Normalized<S>
where
    S: Deref<Target = [T]>,
{
    pub fn get(&self, index: usize) -> Option<Normalized<&T>> {
        self.0.deref().get(index).map(Normalized)
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
    pub fn from_vec_normalized(v: Vec<Normalized<T>>) -> Self {
        Normalized(v.into_iter().map(Normalized::into_raw).collect())
    }

    pub fn into_vec_normalized(self) -> Vec<Normalized<T>> {
        self.0.into_iter().map(Normalized).collect()
    }
}

impl<T> Normalized<Vec<T>> {
    pub fn push(&mut self, item: Normalized<T>) {
        self.0.push(item.into_raw())
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
    ) -> Option<(Normalized<RcHashed<Ind>>, Normalized<RcHashed<Box<[Expr]>>>)> {
        match self.0 {
            Expr::Ind(ind) => Some((
                Normalized(ind),
                Normalized(Rc::new(Hashed::new(Box::new([])))),
            )),

            Expr::App(app) => match &app.value.callee {
                Expr::Ind(ind) => {
                    Some((Normalized(ind.clone()), Normalized(app.value.args.clone())))
                }
                _other_callee => None,
            },

            _ => None,
        }
    }
}

impl Normalized<&Ind> {
    pub fn vcon_defs(self) -> Normalized<RcHashed<Box<[VconDef]>>> {
        Normalized(self.0.vcon_defs.clone())
    }
}

impl Normalized<&VconDef> {
    pub fn index_args(self) -> Normalized<RcHashed<Box<[Expr]>>> {
        Normalized(self.0.index_args.clone())
    }
}

impl Normalized<App> {
    pub fn app_with_ind_callee(
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

impl Normalized<&For> {
    pub fn param_types(self) -> Normalized<RcHashed<Box<[Expr]>>> {
        Normalized(self.0.param_types.clone())
    }
}

impl Normalized<For> {
    pub fn for_(param_types: Normalized<RcHashed<Box<[Expr]>>>, return_type: NormalForm) -> Self {
        Normalized(For {
            param_types: param_types.into_raw(),
            return_type: return_type.into_raw(),
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
    pub fn vcon_capp(
        ind: Normalized<RcHashed<Ind>>,
        vcon_index: usize,
        arg_count: usize,
    ) -> NormalForm {
        let vcon = Vcon {
            ind: ind.into_raw(),
            vcon_index,
        };
        let args: Vec<Expr> = (0..arg_count)
            .into_iter()
            .rev()
            .map(|i| DebNode { deb: Deb(i) }.into())
            .collect();
        let capp = App {
            callee: vcon.into(),
            args: rc_hash(args.into_boxed_slice()),
        }
        .collapse_if_nullary();
        Normalized(capp)
    }
}

impl NormalForm {
    pub fn upshift(self, amount: usize) -> Self {
        Normalized(DebUpshifter(amount).replace_debs(self.0, 0))
    }
}

impl Normalized<RcHashed<Box<[Expr]>>> {
    pub fn upshift_expressions_with_constant_cutoff(self, amount: usize) -> Self {
        Normalized(DebUpshifter(amount).replace_debs_in_expressions_with_constant_cutoff(self.0, 0))
    }

    pub fn replace_deb0_with_ind_with_increasing_cutoff(
        self,
        ind: Normalized<RcHashed<Ind>>,
    ) -> Self {
        let ind_singleton: [Expr; 1] = [ind.raw().clone().into()];
        let ind_singleton_deb_substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };
        Normalized(
            ind_singleton_deb_substituter
                .replace_debs_in_expressions_with_increasing_cutoff(self.0, 0),
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
        T: SemanticHash,
    {
        fn rc_hash_and_wrap_in_normalized(self) -> Normalized<RcHashed<Self>> {
            Normalized(rc_hash(self))
        }
    }
}
