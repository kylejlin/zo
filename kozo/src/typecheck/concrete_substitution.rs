use crate::{
    eval::*,
    syntax_tree::{ast::*, is_subexpression::*},
};

/// `ConcreteSubstitution::from()` is guaranteed
/// to return a value that is **not** a
/// strict subexpression of `ConcreteSubstitution::to()`.
#[derive(Debug, Clone)]
pub struct ConcreteSubstitution {
    from: NormalForm,
    to: NormalForm,
}

impl ConcreteSubstitution {
    /// The `tentative_from` and `tentative_to`
    /// params are called "tentative" because
    /// they will be swapped iff `tentative_from`
    /// is a strict subexpression of `tentative_to`.
    pub fn new(tentative_from: NormalForm, tentative_to: NormalForm) -> Self {
        let (from, to) = if tentative_from
            .raw()
            .is_strict_subexpression_of(tentative_to.raw())
        {
            (tentative_to, tentative_from)
        } else {
            (tentative_from, tentative_to)
        };
        Self { from, to }
    }
}

impl ConcreteSubstitution {
    pub fn from(&self) -> &NormalForm {
        &self.from
    }

    pub fn to(&self) -> &NormalForm {
        &self.to
    }
}

impl PartialEq for ConcreteSubstitution {
    fn eq(&self, other: &Self) -> bool {
        self.from.raw().digest() == other.from.raw().digest()
            && self.to.raw().digest() == other.to.raw().digest()
    }
}

impl ConcreteSubstitution {
    pub fn upshift(&self, amount: usize) -> Self {
        Self {
            from: self.from.clone().upshift(amount, 0),
            to: self.to.clone().upshift(amount, 0),
        }
    }
}

pub trait Substitute: Sized {
    type Output;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output;

    fn substitute(self, sub: &ConcreteSubstitution) -> Self::Output
    where
        Self: GetDigest,
        Self::Output: From<Expr>,
        Self::Output: Into<Expr>,
    {
        if self.digest() == sub.from.raw().digest() {
            return sub.to.raw().clone().into();
        }

        self.substitute_in_children(sub)
    }
}

impl Substitute for Expr {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        match self {
            Expr::Ind(e) => e.substitute_in_children(sub),
            Expr::Vcon(e) => e.substitute_in_children(sub),
            Expr::Match(e) => e.substitute_in_children(sub),
            Expr::Fun(e) => e.substitute_in_children(sub),
            Expr::App(e) => e.substitute_in_children(sub),
            Expr::For(e) => e.substitute_in_children(sub),
            Expr::Deb(e) => e.substitute_in_children(sub),
            Expr::Universe(e) => e.substitute_in_children(sub),
        }
    }
}

impl Substitute for RcSemHashed<Ind> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        substitute_in_ind_children(self, sub).into()
    }
}

fn substitute_in_ind_children(ind: RcSemHashed<Ind>, sub: &ConcreteSubstitution) -> Ind {
    Ind {
        name: ind.hashee.name.clone(),
        universe_level: ind.hashee.universe_level,
        index_types: DependentExprs(&ind.hashee.index_types.hashee).substitute_in_children(sub),
        vcon_defs: ind
            .hashee
            .vcon_defs
            .clone()
            .substitute_in_children(&sub.upshift(1)),
    }
}

struct DependentExprs<'a>(&'a [Expr]);
struct IndependentExprs<'a>(&'a [Expr]);

impl Substitute for DependentExprs<'_> {
    type Output = RcSemHashedVec<Expr>;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .0
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, e)| e.substitute(&sub.upshift(i)))
            .collect::<Vec<_>>();
        rc_sem_hashed(subbed)
    }
}

impl Substitute for IndependentExprs<'_> {
    type Output = RcSemHashedVec<Expr>;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .0
            .iter()
            .cloned()
            .map(|e| e.substitute(sub))
            .collect::<Vec<_>>();
        rc_sem_hashed(subbed)
    }
}

impl Substitute for RcSemHashedVec<VconDef> {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .hashee
            .iter()
            .cloned()
            .map(|def| def.substitute_in_children(sub))
            .collect::<Vec<_>>();
        rc_sem_hashed(subbed)
    }
}

impl Substitute for VconDef {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        VconDef {
            param_types: DependentExprs(&self.param_types.hashee).substitute_in_children(sub),
            index_args: IndependentExprs(&self.index_args.hashee)
                .substitute_in_children(&sub.upshift(self.param_types.hashee.len())),
        }
    }
}

impl Substitute for RcSemHashed<Vcon> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        Vcon {
            ind: rc_sem_hashed(substitute_in_ind_children(self.hashee.ind.clone(), sub)),
            vcon_index: self.hashee.vcon_index,
        }
        .into()
    }
}

impl Substitute for RcSemHashed<Match> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        Match {
            matchee: self.hashee.matchee.clone().substitute(sub),
            return_type: self.hashee.return_type.clone().substitute(sub),
            cases: self.hashee.cases.clone().substitute_in_children(sub),
        }
        .into()
    }
}

impl Substitute for RcSemHashedVec<MatchCase> {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .hashee
            .iter()
            .cloned()
            .map(|case| case.substitute_in_children(sub))
            .collect::<Vec<_>>();
        rc_sem_hashed(subbed)
    }
}

impl Substitute for MatchCase {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        match self {
            MatchCase::Dismissed => self,
            MatchCase::Nondismissed(case) => {
                MatchCase::Nondismissed(case.substitute_in_children(sub))
            }
        }
    }
}

impl Substitute for NondismissedMatchCase {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        NondismissedMatchCase {
            arity: self.arity,
            return_val: self.return_val.substitute(&sub.upshift(self.arity)),
        }
    }
}

impl Substitute for RcSemHashed<Fun> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        Fun {
            decreasing_index: self.hashee.decreasing_index,
            param_types: DependentExprs(&self.hashee.param_types.hashee)
                .substitute_in_children(sub),
            return_type: self
                .hashee
                .return_type
                .clone()
                .substitute(&sub.upshift(self.hashee.param_types.hashee.len())),
            return_val: self
                .hashee
                .return_val
                .clone()
                .substitute(&sub.upshift(self.hashee.param_types.hashee.len() + 1)),
        }
        .into()
    }
}

impl Substitute for RcSemHashed<App> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        App {
            callee: self.hashee.callee.clone().substitute(sub),
            args: IndependentExprs(&self.hashee.args.hashee).substitute_in_children(sub),
        }
        .into()
    }
}

impl Substitute for RcSemHashed<For> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        For {
            param_types: DependentExprs(&self.hashee.param_types.hashee)
                .substitute_in_children(sub),
            return_type: self
                .hashee
                .return_type
                .clone()
                .substitute(&sub.upshift(self.hashee.param_types.hashee.len())),
        }
        .into()
    }
}

impl Substitute for RcSemHashed<DebNode> {
    type Output = Expr;

    fn substitute_in_children(self, _: &ConcreteSubstitution) -> Self::Output {
        self.into()
    }
}

impl Substitute for RcSemHashed<UniverseNode> {
    type Output = Expr;

    fn substitute_in_children(self, _: &ConcreteSubstitution) -> Self::Output {
        self.into()
    }
}

// TODO: Test `Substitute` implementations.
