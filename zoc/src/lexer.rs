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

#[derive(Clone, Copy, Debug)]
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
                self.handle_char_assuming_state_is_string(current_index, current, start, byte_len)
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
        current_index: ByteIndex,
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
                let quote_exclusive_string_src = &self.src[start.0 + '"'.len_utf8()..end.0];
                let value = get_string_value(quote_exclusive_string_src)?;
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

    use std::{iter::Peekable, str::CharIndices};

    /// This function assumes the `quote_exclusive_string_src` argument
    /// does not have any double quotes.
    /// If `quote_exclusive_string_src` has double quotes,
    /// this function may produce an incorrect result.
    pub fn get_string_value(quote_exclusive_string_src: &str) -> Result<String, ByteIndex> {
        let mut out = String::new();
        let mut iter = quote_exclusive_string_src.char_indices().peekable();
        while let Some((i, c)) = iter.next() {
            if let Some(escape_range) = consume_escape_sequence_if_it_exists(&mut iter)? {
                let escape_src = &quote_exclusive_string_src[escape_range.0 .0..escape_range.1 .0];
                todo!();
                continue;
            }

            out.push(c);
        }
        Ok(out)
    }

    fn consume_escape_sequence_if_it_exists(
        iter: &mut Peekable<CharIndices>,
    ) -> Result<Option<(ByteIndex, ByteIndex)>, ByteIndex> {
        if iter.peek().map(|(_, c)| *c) != Some('{') {
            return Ok(None);
        }

        todo!()
    }
}
