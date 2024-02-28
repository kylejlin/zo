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

    impl ZeroOrMoreEnumCases {
        pub fn to_vec(&self) -> Vec<&EnumCase> {
            match self {
                ZeroOrMoreEnumCases::Nil => vec![],
                ZeroOrMoreEnumCases::Snoc(rdc, rac) => {
                    let mut v = rdc.to_vec();
                    v.push(rac);
                    v
                }
            }
        }
    }

    impl OptCaretParenthesizedParamDefs {
        pub fn to_std_option(&self) -> Option<&CommaSeparatedNonfunParamDefs> {
            match self {
                OptCaretParenthesizedParamDefs::None => None,
                OptCaretParenthesizedParamDefs::Some(p) => Some(&p.params),
            }
        }
    }

    impl OptParenthesizedNonfunParamDefs {
        pub fn to_std_option(&self) -> Option<&CommaSeparatedNonfunParamDefs> {
            match self {
                OptParenthesizedNonfunParamDefs::None => None,
                OptParenthesizedNonfunParamDefs::Some(p) => Some(&p.params),
            }
        }
    }

    impl OptCaretParenthesizedExprs {
        pub fn to_std_option(&self) -> Option<&CommaSeparatedExprs> {
            match self {
                OptCaretParenthesizedExprs::None => None,
                OptCaretParenthesizedExprs::Some(p) => Some(&p.exprs),
            }
        }
    }

    impl ZeroOrMoreMatchCases {
        pub fn to_vec(&self) -> Vec<&MatchCase> {
            match self {
                ZeroOrMoreMatchCases::Nil => vec![],
                ZeroOrMoreMatchCases::Snoc(rdc, rac) => {
                    let mut v = rdc.to_vec();
                    v.push(rac);
                    v
                }
            }
        }
    }

    impl OptParenthesizedCommaSeparatedIdentsOrUnderscores {
        pub fn len(&self) -> usize {
            match self {
                OptParenthesizedCommaSeparatedIdentsOrUnderscores::None => 0,
                OptParenthesizedCommaSeparatedIdentsOrUnderscores::Some(p) => p.len(),
            }
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl ParenthesizedCommaSeparatedIdentsOrUnderscores {
        pub fn len(&self) -> usize {
            self.idents.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl CommaSeparatedIdentsOrUnderscores {
        pub fn len(&self) -> usize {
            match self {
                CommaSeparatedIdentsOrUnderscores::One(_) => 1,
                CommaSeparatedIdentsOrUnderscores::Snoc(rdc, _) => 1 + rdc.len(),
            }
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn to_vec(&self) -> Vec<&IdentOrUnderscore> {
            match self {
                CommaSeparatedIdentsOrUnderscores::One(i) => vec![i],
                CommaSeparatedIdentsOrUnderscores::Snoc(rdc, rac) => {
                    let mut v = rdc.to_vec();
                    v.push(rac);
                    v
                }
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
