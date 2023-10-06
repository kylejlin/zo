pub use generated_parser::parse;
mod generated_parser;

pub mod cst {
    pub use super::generated_parser::*;
}

// TODO: Uncomment
// #[cfg(test)]
// mod tests;
