use crate::{eval::*, syntax_tree::ast::*};

#[derive(Debug, Clone)]
pub struct ConcreteSubstitution {
    pub from: NormalForm,
    pub to: NormalForm,
}

impl ConcreteSubstitution {
    pub fn upshift(&self, amount: usize) -> Self {
        Self {
            from: self.from.clone().upshift(amount),
            to: self.to.clone().upshift(amount),
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
        name: ind.value.name.clone(),
        universe_level: ind.value.universe_level,
        index_types: ind.value.index_types.clone().substitute_in_children(sub),
        vcon_defs: ind
            .value
            .vcon_defs
            .clone()
            .substitute_in_children(&sub.upshift(1)),
    }
}

impl Substitute for IndepRcSemHashedVec<Expr> {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .0
            .value
            .iter()
            .cloned()
            .map(|e| e.substitute(sub))
            .collect::<Vec<_>>();
        indep_rc_sem_hashed(subbed)
    }
}

impl Substitute for DepRcSemHashedVec<Expr> {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .0
            .value
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, e)| e.substitute(&sub.upshift(i)))
            .collect::<Vec<_>>();
        dep_rc_sem_hashed(subbed)
    }
}

impl Substitute for IndepRcSemHashedVec<VconDef> {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .value
            .iter()
            .cloned()
            .map(|def| def.substitute_in_children(sub))
            .collect::<Vec<_>>();
        indep_rc_sem_hashed(subbed)
    }
}

impl Substitute for VconDef {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let param_count = self.param_types.value.len();
        VconDef {
            param_types: self.param_types.substitute_in_children(sub),
            index_args: self
                .index_args
                .substitute_in_children(&sub.upshift(param_count)),
        }
    }
}

impl Substitute for RcSemHashed<Vcon> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        Vcon {
            ind: rc_sem_hashed(substitute_in_ind_children(self.value.ind.clone(), sub)),
            vcon_index: self.value.vcon_index,
        }
        .into()
    }
}

impl Substitute for RcSemHashed<Match> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        Match {
            matchee: self.value.matchee.clone().substitute(sub),
            return_type: self.value.return_type.clone().substitute(sub),
            cases: self.value.cases.clone().substitute_in_children(sub),
        }
        .into()
    }
}

impl Substitute for IndepRcSemHashedVec<MatchCase> {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        let subbed = self
            .value
            .iter()
            .cloned()
            .map(|case| case.substitute_in_children(sub))
            .collect::<Vec<_>>();
        indep_rc_sem_hashed(subbed)
    }
}

impl Substitute for MatchCase {
    type Output = Self;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        MatchCase {
            arity: self.arity,
            return_val: self.return_val.substitute(&sub.upshift(self.arity)),
        }
    }
}

impl Substitute for RcSemHashed<Fun> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        Fun {
            decreasing_index: self.value.decreasing_index,
            param_types: self.value.param_types.clone().substitute_in_children(sub),
            return_type: self
                .value
                .return_type
                .clone()
                .substitute(&sub.upshift(self.value.param_types.value.len())),
            return_val: self
                .value
                .return_val
                .clone()
                .substitute(&sub.upshift(self.value.param_types.value.len() + 1)),
        }
        .into()
    }
}

impl Substitute for RcSemHashed<App> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        App {
            callee: self.value.callee.clone().substitute(sub),
            args: self.value.args.clone().substitute_in_children(sub),
        }
        .into()
    }
}

impl Substitute for RcSemHashed<For> {
    type Output = Expr;

    fn substitute_in_children(self, sub: &ConcreteSubstitution) -> Self::Output {
        For {
            param_types: self.value.param_types.clone().substitute_in_children(sub),
            return_type: self
                .value
                .return_type
                .clone()
                .substitute(&sub.upshift(self.value.param_types.value.len())),
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
