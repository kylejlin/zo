use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    rc::Rc,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrettyPrint<'a, T>(pub &'a T);

impl<'a, T> Debug for PrettyPrint<'a, T>
where
    PrettyPrint<'a, T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

mod impl_for_ast;
mod impl_for_type_error;

pub trait PrettyPrinted {
    fn pretty_printed(&self) -> PrettyPrint<'_, Self>
    where
        Self: Sized;
}

impl<T> PrettyPrinted for T {
    fn pretty_printed(&self) -> PrettyPrint<'_, Self>
    where
        Self: Sized,
    {
        PrettyPrint(self)
    }
}

pub trait PrettyUnwrap {
    type Output;

    fn pretty_unwrap(self) -> Self::Output;
}

impl<T, E> PrettyUnwrap for Result<T, E>
where
    for<'a> PrettyPrint<'a, E>: Display,
{
    type Output = T;

    fn pretty_unwrap(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                // We use the "#" flag so that if the
                // Display impl of `PrettyPrint<E>` uses
                // `Formatter:debug_struct` (or `debug_tuple`, etc.),
                // the output will be printed with easy-to-read
                // indentation.
                panic!(
                    "called `Result::unwrap()` on an `Err` value:\n{:#}",
                    PrettyPrint(&e)
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
