pub mod ast;
pub mod cst;
pub mod cst_to_ast;
pub mod eval;
pub mod lexer;
pub mod nohash_hashmap;
pub mod replace_debs;
pub mod semantic_hash;
pub mod token;
pub mod typecheck;

mod parser;
/// Since the `parser` module
/// is generated by Kiki,
/// we cannot include a `tests` submodule in it.
/// Thus, we depart from the custom,
/// and create an external `parser_tests` module.
#[cfg(test)]
mod parser_tests;
