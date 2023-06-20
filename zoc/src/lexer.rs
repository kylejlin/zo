use crate::token::*;

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
        "forall" => return Some(Token::ForallKw(start)),
        "nonrec" => return Some(Token::NonrecKw(start)),
        _ => {}
    }

    if s.starts_with("Type") {
        let level_src = &s["Type".len()..];
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

        return Some(Token::Universe(UniverseLiteral { level, start }));
    }

    None
}

use string_parser::get_string_value;
mod string_parser {

    use super::*;

    /// This function assumes the `quote_exclusive_string_src` argument
    /// does not have any double quotes.
    /// If `quote_exclusive_string_src` has double quotes,
    /// this function may produce an incorrect result.
    ///
    /// This function returns `Err(Some(span))` if it encounters
    /// an invalid escape sequence, where `span` is the location
    /// of the invalid escape sequence.
    ///
    /// This function returns `Err(None)` if it encounters
    /// an unterminated escape sequence.
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
                State::Escape { start, .. } => Err((
                    start,
                    ByteIndex(start.0 + self.quote_exclusive_string_src.len()),
                )),
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
                start: current_index,
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
}
