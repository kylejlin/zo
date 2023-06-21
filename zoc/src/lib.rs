pub mod cst;
pub mod lexer;
pub mod token;

mod parser;
/// Since the `parser` module
/// is generated by Kiki,
/// we cannot include a `tests` submodule in it.
/// Thus, we depart from the custom,
/// and create an external `parser_tests` module.
#[cfg(test)]
mod parser_tests;
