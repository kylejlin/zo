use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    rc::Rc,
};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrettyPrinted<'a, T>(pub &'a T);

mod impl_for_ast;

pub trait PrettyUnwrap {
    type Output;

    fn pretty_unwrap(self) -> Self::Output;
}

impl<T, E> PrettyUnwrap for Result<T, E>
where
    for<'a> PrettyPrinted<'a, E>: Display,
{
    type Output = T;

    fn pretty_unwrap(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "called `Result::unwrap()` on an `Err` value:\n{}",
                    PrettyPrinted(&e)
                );
            }
        }
    }
}

pub const SOFT_TAB: &str = "    ";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Indentation {
    pub soft_tab_count: usize,
}

impl Indentation {
    pub fn incremented(self) -> Self {
        Self {
            soft_tab_count: self.soft_tab_count + 1,
        }
    }
}

impl Display for Indentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for _ in 0..self.soft_tab_count {
            f.write_str(SOFT_TAB)?;
        }
        Ok(())
    }
}
