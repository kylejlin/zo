pub use generated_parser::parse;
mod generated_parser;

pub mod cst {

    pub use super::generated_parser::*;

    impl IdentOrUnderscore {
        pub fn val(&self) -> &str {
            match self {
                IdentOrUnderscore::Ident(s) => &s.value,
                IdentOrUnderscore::Underscore(_) => "_",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::lex;

    #[test]
    fn parses_valid_input_correctly() {
        let src = include_str!("./parser_test_sample_code.jn");
        let tokens = lex(src).unwrap();
        parse(tokens).unwrap();
    }
}
