use crate::{
    eval::Normalized, pretty_print::PrettyPrint, test_utils::*, typecheck::LazyTypeContext,
};

mod fun_recursion;

// General tests
mod should_fail;
mod should_succeed;

// TODO: Reorganize this module into "topics" instead of
// "should succeed" and "should fail".
