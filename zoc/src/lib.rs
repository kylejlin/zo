pub mod eval;
pub mod hash;
pub mod pretty_print;
pub mod syntax_tree;
pub mod typecheck;

#[cfg(test)]
pub mod test_utils;

// TODO: Add recursion checker.
// We must check arg decrease before
// typechecking, otherwise we might
// end up in infinite recursion.
