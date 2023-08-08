use crate::syntax_tree::token::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LexError(ByteIndex, ByteIndex);

pub fn lex(s: &str) -> Result<Vec<Token>, LexError> {
    Lexer::new(s).lex()
}

struct Lexer<'a> {
    src: &'a str,
    out: Vec<Token>,
    state: State,
}

#[derive(Clone, Copy)]
enum State {
    Main,
    Word { start: ByteIndex, byte_len: usize },
    String { start: ByteIndex, byte_len: usize },
    Slash(ByteIndex),
    SingleLineComment,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Lexer {
            src,
            out: Vec::new(),
            state: State::Main,
        }
    }
}

impl Lexer<'_> {
    fn lex(mut self) -> Result<Vec<Token>, LexError> {
        for (current_index, curent) in self.src.char_indices() {
            self.handle_char(ByteIndex(current_index), curent)?;
        }
        self.finish_pending_state_and_reset()?;
        Ok(self.out)
    }

    fn handle_char(&mut self, current_index: ByteIndex, current: char) -> Result<(), LexError> {
        match self.state {
            State::Main => self.handle_char_assuming_state_is_main(current_index, current),
            State::Word { start, byte_len } => {
                self.handle_char_assuming_state_is_word(current_index, current, start, byte_len)
            }
            State::String { start, byte_len } => {
                self.handle_char_assuming_state_is_string(current, start, byte_len)
            }
            State::Slash(_) => self.handle_char_assuming_state_is_slash(current_index, current),
            State::SingleLineComment => {
                self.handle_char_assuming_state_is_single_line_comment(current)
            }
        }
    }

    fn handle_char_assuming_state_is_main(
        &mut self,
        current_index: ByteIndex,
        current: char,
    ) -> Result<(), LexError> {
        match current {
            ' ' | '\t' | '\n' => {}
            '"' => {
                self.state = State::String {
                    start: current_index,
                    byte_len: '"'.len_utf8(),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                self.state = State::Word {
                    start: current_index,
                    byte_len: 1,
                }
            }
            '(' => self.out.push(Token::LParen(current_index)),
            ')' => self.out.push(Token::RParen(current_index)),
            '/' => self.state = State::Slash(current_index),
            _ => {
                return Err(LexError(
                    current_index,
                    ByteIndex(current_index.0 + current.len_utf8()),
                ))
            }
        }
        Ok(())
    }

    fn handle_char_assuming_state_is_word(
        &mut self,
        current_index: ByteIndex,
        current: char,
        start: ByteIndex,
        byte_len: usize,
    ) -> Result<(), LexError> {
        match current {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                self.state = State::Word {
                    start,
                    byte_len: byte_len + 1,
                }
            }
            _ => {
                self.finish_pending_state_and_reset()?;
                self.handle_char(current_index, current)?;
            }
        }
        Ok(())
    }

    fn handle_char_assuming_state_is_string(
        &mut self,
        current: char,
        start: ByteIndex,
        byte_len: usize,
    ) -> Result<(), LexError> {
        self.state = State::String {
            start,
            byte_len: byte_len + current.len_utf8(),
        };

        if current == '"' {
            self.finish_pending_state_and_reset()?;
        }

        Ok(())
    }

    fn handle_char_assuming_state_is_slash(
        &mut self,
        current_index: ByteIndex,
        current: char,
    ) -> Result<(), LexError> {
        if current == '/' {
            self.state = State::SingleLineComment;
            return Ok(());
        }

        self.finish_pending_state_and_reset()?;
        self.handle_char(current_index, current)
    }

    fn handle_char_assuming_state_is_single_line_comment(
        &mut self,
        current: char,
    ) -> Result<(), LexError> {
        if current == '\n' {
            self.state = State::Main;
        }
        Ok(())
    }

    fn finish_pending_state_and_reset(&mut self) -> Result<(), LexError> {
        self.finish_pending_state()?;
        self.state = State::Main;
        Ok(())
    }

    fn finish_pending_state(&mut self) -> Result<(), LexError> {
        match self.state {
            State::Main => Ok(()),

            State::Word { start, byte_len } => {
                let word_src = &self.src[start.0..start.0 + byte_len];
                let Some(word) = parse_word(word_src, start) else {
                    return Err(LexError(start, ByteIndex(start.0 + byte_len)));
                };
                self.out.push(word);
                Ok(())
            }

            State::String { start, byte_len } => {
                let end = ByteIndex(start.0 + byte_len);
                let quote_exclusive_start = ByteIndex(start.0 + '"'.len_utf8());
                let quote_exclusive_end = ByteIndex(start.0 + byte_len - '"'.len_utf8());
                let quote_exclusive_string_src =
                    &self.src[quote_exclusive_start.0..quote_exclusive_end.0];
                let value = match get_string_value(quote_exclusive_string_src) {
                    Ok(v) => v,
                    Err(local_span) => {
                        return Err(LexError(
                            quote_exclusive_start + local_span.0,
                            quote_exclusive_start + local_span.1,
                        ));
                    }
                };
                self.out.push(Token::String(StringLiteral {
                    value,
                    span: (start, end),
                }));
                Ok(())
            }
            State::Slash(start) => Err(LexError(start, ByteIndex(start.0 + '/'.len_utf8()))),
            State::SingleLineComment => Ok(()),
        }
    }
}

fn parse_word(s: &str, start: ByteIndex) -> Option<Token> {
    if let Ok(val) = s.parse::<usize>() {
        return Some(Token::Number(NumberLiteral {
            value: val,
            span: (start, ByteIndex(start.0 + s.len())),
        }));
    }

    match s {
        "ind" => return Some(Token::IndKw(start)),
        "vcon" => return Some(Token::VconKw(start)),
        "match" => return Some(Token::MatchKw(start)),
        "fun" => return Some(Token::FunKw(start)),
        "for" => return Some(Token::ForKw(start)),
        "nonrec" => return Some(Token::NonrecKw(start)),
        _ => {}
    }

    if s.starts_with("Set") {
        let level = get_number_after_prefix(s, "Set")?;
        return Some(Token::Universe(UniverseLiteral {
            level,
            start,
            erasable: false,
        }));
    }

    if s.starts_with("Prop") {
        let level = get_number_after_prefix(s, "Prop")?;
        return Some(Token::Universe(UniverseLiteral {
            level,
            start,
            erasable: true,
        }));
    }

    None
}

fn get_number_after_prefix(s: &str, prefix: &str) -> Option<usize> {
    let level_src = &s[prefix.len()..];
    if level_src.is_empty() {
        return None;
    }

    let has_extraneous_leading_zeros = level_src != "0" && level_src.starts_with('0');
    if has_extraneous_leading_zeros {
        return None;
    }

    let Ok(level) = level_src.parse::<usize>() else {
        return None;
    };

    Some(level)
}

use string_parser::get_string_value;
mod string_parser {

    use super::*;

    /// This function assumes the `quote_exclusive_string_src` argument
    /// does not have any double quotes.
    /// If `quote_exclusive_string_src` has double quotes,
    /// this function may produce an incorrect result.
    ///
    /// This function returns `Err(span)` if it encounters
    /// an invalid escape sequence or an unterminated escape sequence.
    /// - If the escape sequence is invalid,
    ///   `span` is the span of the invalid escape sequence,
    ///   **excluding** the enclosing curly braces.
    /// - If the escape sequence is unterminated,
    ///   the `span` is the span of the unterminated escape sequence,
    ///   **including** the left curly brace.
    ///   By definition, there is no right curly brace
    ///   (otherwise the escape sequence would be terminated).
    pub fn get_string_value(
        quote_exclusive_string_src: &str,
    ) -> Result<String, (ByteIndex, ByteIndex)> {
        StringParser::new(quote_exclusive_string_src).parse()
    }

    struct StringParser<'a> {
        quote_exclusive_string_src: &'a str,
        out: String,
        state: State,
    }

    #[derive(Clone, Copy)]
    enum State {
        Main,
        Escape { start: ByteIndex, byte_len: usize },
    }

    impl<'a> StringParser<'a> {
        fn new(quote_exclusive_string_src: &'a str) -> Self {
            Self {
                quote_exclusive_string_src,
                out: String::new(),
                state: State::Main,
            }
        }
    }

    impl StringParser<'_> {
        fn parse(mut self) -> Result<String, (ByteIndex, ByteIndex)> {
            for (current_index, current) in self.quote_exclusive_string_src.char_indices() {
                self.handle_char(ByteIndex(current_index), current)?;
            }

            match self.state {
                State::Escape { start, .. } => {
                    Err((start, ByteIndex(self.quote_exclusive_string_src.len())))
                }
                State::Main => Ok(self.out),
            }
        }

        fn handle_char(
            &mut self,
            current_index: ByteIndex,
            current: char,
        ) -> Result<(), (ByteIndex, ByteIndex)> {
            match self.state {
                State::Main => self.handle_char_assuming_state_is_main(current_index, current),
                State::Escape { start, byte_len } => self.handle_char_assuming_state_is_escape(
                    current_index,
                    current,
                    start,
                    byte_len,
                ),
            }
        }

        fn handle_char_assuming_state_is_main(
            &mut self,
            current_index: ByteIndex,
            current: char,
        ) -> Result<(), (ByteIndex, ByteIndex)> {
            match current {
                '{' => {
                    self.state = State::Escape {
                        start: current_index,
                        byte_len: '{'.len_utf8(),
                    };
                    Ok(())
                }

                '}' => Err((current_index, ByteIndex(current_index.0 + '}'.len_utf8()))),

                _ => {
                    self.out.push(current);
                    Ok(())
                }
            }
        }

        fn handle_char_assuming_state_is_escape(
            &mut self,
            current_index: ByteIndex,
            current: char,
            start: ByteIndex,
            byte_len: usize,
        ) -> Result<(), (ByteIndex, ByteIndex)> {
            if current == '}' {
                let brace_exclusive_start = ByteIndex(start.0 + '{'.len_utf8());
                let brace_exclusive_end = current_index;
                self.push_escape_sequence(brace_exclusive_start, brace_exclusive_end)?;
                self.state = State::Main;
                return Ok(());
            }

            self.state = State::Escape {
                start,
                byte_len: byte_len + current.len_utf8(),
            };
            Ok(())
        }

        fn push_escape_sequence(
            &mut self,
            brace_exclusive_start: ByteIndex,
            brace_exclusive_end: ByteIndex,
        ) -> Result<(), (ByteIndex, ByteIndex)> {
            let invalid_escape_sequence_err = Err((brace_exclusive_start, brace_exclusive_end));

            let byte_len = brace_exclusive_end.0 - brace_exclusive_start.0;
            if byte_len < 3 {
                return invalid_escape_sequence_err;
            }

            let brace_exclusive_src =
                &self.quote_exclusive_string_src[brace_exclusive_start.0..brace_exclusive_end.0];
            if !brace_exclusive_src.starts_with("0x") {
                return invalid_escape_sequence_err;
            }

            let hex_start = ByteIndex(brace_exclusive_start.0 + "0x".len());
            let hex_src = &self.quote_exclusive_string_src[hex_start.0..brace_exclusive_end.0];
            let Ok(val) = u32::from_str_radix(hex_src, 16) else {
                return invalid_escape_sequence_err;
            };
            let Ok(val) = char::try_from(val) else {
                return invalid_escape_sequence_err;
            };
            self.out.push(val);
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use pretty_assertions::assert_eq;

        #[test]
        fn empty() {
            let actual = get_string_value("");
            let expected = Ok("".to_owned());
            assert_eq!(expected, actual);
        }

        #[test]
        fn hello_world() {
            let actual = get_string_value("hello world");
            let expected = Ok("hello world".to_owned());
            assert_eq!(expected, actual);
        }

        #[test]
        fn hello_unescaped_newline_world() {
            let actual = get_string_value("hello\nworld");
            let expected = Ok("hello\nworld".to_owned());
            assert_eq!(expected, actual);
        }

        #[test]
        fn hello_escaped_newline_world() {
            let actual = get_string_value("hello{0xA}world");
            let expected = Ok("hello\nworld".to_owned());
            assert_eq!(expected, actual);
        }

        #[test]
        fn hello_lcurly_world_rcurly() {
            let actual = get_string_value("hello{0x7B}world{0x7D}");
            let expected = Ok("hello{world}".to_owned());
            assert_eq!(expected, actual);
        }

        #[test]
        fn hello_double_quote_world_double_quote() {
            let actual = get_string_value("hello {0x22}world{0x22}");
            let expected = Ok("hello \"world\"".to_owned());
            assert_eq!(expected, actual);
        }

        #[test]
        fn unterminated_invalid_escape() {
            let actual = get_string_value("hello {world");
            let expected = Err((ByteIndex(6), ByteIndex(12)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn unterminated_but_otherwise_valid_escape() {
            let actual = get_string_value("hello {0x22");
            let expected = Err((ByteIndex(6), ByteIndex(11)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn empty_escape() {
            let actual = get_string_value("hello {} world");
            let expected = Err((ByteIndex(7), ByteIndex(7)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn too_short_escape_1_char() {
            let actual = get_string_value("hello {0} world");
            let expected = Err((ByteIndex(7), ByteIndex(8)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn too_short_escape_2_char() {
            let actual = get_string_value("hello {0x} world");
            let expected = Err((ByteIndex(7), ByteIndex(9)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn bad_prefix_escape() {
            let actual = get_string_value("hello {BEEF} world");
            let expected = Err((ByteIndex(7), ByteIndex(11)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn curly_in_escape() {
            let actual = get_string_value("hello {0x{A}} world");
            let expected = Err((ByteIndex(7), ByteIndex(11)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn non_hex_escape() {
            let actual = get_string_value("hello {0xG} world");
            let expected = Err((ByteIndex(7), ByteIndex(10)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn capital_x_escape() {
            let actual = get_string_value("hello {0XA} world");
            let expected = Err((ByteIndex(7), ByteIndex(10)));
            assert_eq!(expected, actual);
        }

        #[test]
        fn unescaped_rcurly() {
            let actual = get_string_value("hello } world");
            let expected = Err((ByteIndex(6), ByteIndex(7)));
            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let actual = lex("");
        let expected = Ok(vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn just_whitespace() {
        let actual = lex("   \n  \t\t \n ");
        let expected = Ok(vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn ind() {
        let actual = lex(r#"(ind Set0 "Nat" () ((() ()) ((0) ())))"#);
        let expected = Ok(vec![
            Token::LParen(ByteIndex(0)),
            Token::IndKw(ByteIndex(1)),
            Token::Universe(UniverseLiteral {
                level: 0,
                start: ByteIndex(5),
                erasable: false,
            }),
            Token::String(StringLiteral {
                value: "Nat".to_owned(),
                span: (ByteIndex(10), ByteIndex(15)),
            }),
            // Begin parenthesized constructor definitions
            Token::LParen(ByteIndex(16)),
            Token::RParen(ByteIndex(17)),
            Token::LParen(ByteIndex(19)),
            Token::LParen(ByteIndex(20)),
            Token::LParen(ByteIndex(21)),
            Token::RParen(ByteIndex(22)),
            Token::LParen(ByteIndex(24)),
            Token::RParen(ByteIndex(25)),
            Token::RParen(ByteIndex(26)),
            Token::LParen(ByteIndex(28)),
            Token::LParen(ByteIndex(29)),
            Token::Number(NumberLiteral {
                value: 0,
                span: (ByteIndex(30), ByteIndex(31)),
            }),
            Token::RParen(ByteIndex(31)),
            Token::LParen(ByteIndex(33)),
            Token::RParen(ByteIndex(34)),
            Token::RParen(ByteIndex(35)),
            Token::RParen(ByteIndex(36)),
            // End parenthesized constructor definitions
            Token::RParen(ByteIndex(37)),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn keywords() {
        let src = r#"ind vcon match fun for nonrec Set0 Set1 Set33 Prop0 Prop1 Prop33"#;
        let actual = lex(src);
        let expected = Ok(vec![
            Token::IndKw(ByteIndex(src.find("ind").unwrap())),
            Token::VconKw(ByteIndex(src.find("vcon").unwrap())),
            Token::MatchKw(ByteIndex(src.find("match").unwrap())),
            Token::FunKw(ByteIndex(src.find("fun").unwrap())),
            Token::ForKw(ByteIndex(src.find("for").unwrap())),
            Token::NonrecKw(ByteIndex(src.find("nonrec").unwrap())),
            Token::Universe(UniverseLiteral {
                level: 0,
                start: ByteIndex(src.find("Set0").unwrap()),
                erasable: false,
            }),
            Token::Universe(UniverseLiteral {
                level: 1,
                start: ByteIndex(src.find("Set1").unwrap()),
                erasable: false,
            }),
            Token::Universe(UniverseLiteral {
                level: 33,
                start: ByteIndex(src.find("Set33").unwrap()),
                erasable: false,
            }),
            Token::Universe(UniverseLiteral {
                level: 0,
                start: ByteIndex(src.find("Prop0").unwrap()),
                erasable: true,
            }),
            Token::Universe(UniverseLiteral {
                level: 1,
                start: ByteIndex(src.find("Prop1").unwrap()),
                erasable: true,
            }),
            Token::Universe(UniverseLiteral {
                level: 33,
                start: ByteIndex(src.find("Prop33").unwrap()),
                erasable: true,
            }),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_whitespace() {
        let src = r#"(ind)"#;
        let actual = lex(src);
        let expected = Ok(vec![
            Token::LParen(ByteIndex(src.find("(").unwrap())),
            Token::IndKw(ByteIndex(src.find("ind").unwrap())),
            Token::RParen(ByteIndex(src.find(")").unwrap())),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn comments() {
        let src = r#"(// Hello world!
// You can write comments on their own line.
ind // You can also write them at the end of a line 
nonrec)"#;
        let actual = lex(src);
        let expected = Ok(vec![
            Token::LParen(ByteIndex(src.find("(").unwrap())),
            Token::IndKw(ByteIndex(src.find("ind").unwrap())),
            Token::NonrecKw(ByteIndex(src.find("nonrec").unwrap())),
            Token::RParen(ByteIndex(src.find(")").unwrap())),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_zero_zero() {
        let src = r#"Set00"#;
        let actual = lex(src);
        let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_zero_one() {
        let src = r#"Set01"#;
        let actual = lex(src);
        let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
        assert_eq!(expected, actual);
    }

    #[test]
    fn prop_zero_zero() {
        let src = r#"Prop00"#;
        let actual = lex(src);
        let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
        assert_eq!(expected, actual);
    }

    #[test]
    fn prop_zero_one() {
        let src = r#"Prop01"#;
        let actual = lex(src);
        let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
        assert_eq!(expected, actual);
    }
}
