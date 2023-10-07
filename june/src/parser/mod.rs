pub use generated_parser::parse;
mod generated_parser;

pub mod cst {
    pub use super::generated_parser::*;
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
