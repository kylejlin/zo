use crate::token::*;

use string_parser::get_string_value;
mod string_parser;

#[cfg(test)]
mod tests;

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
    Dash(ByteIndex),
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
            State::Dash(_) => self.handle_char_assuming_state_is_dash(current_index, current),
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
            '[' => self.out.push(Token::LSquare(current_index)),
            ']' => self.out.push(Token::RSquare(current_index)),
            '=' => self.out.push(Token::Eq(current_index)),
            ':' => self.out.push(Token::Colon(current_index)),
            ',' => self.out.push(Token::Comma(current_index)),
            '^' => self.out.push(Token::Caret(current_index)),
            '-' => self.state = State::Dash(current_index),
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

    fn handle_char_assuming_state_is_dash(
        &mut self,
        current_index: ByteIndex,
        current: char,
    ) -> Result<(), LexError> {
        if current == '>' {
            let dash_index = ByteIndex(current_index.0 - '-'.len_utf8());
            self.out.push(Token::ThinArrow(dash_index));
            self.state = State::Main;
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
                self.out.push(Token::StringLiteral(StringLiteral {
                    value,
                    span: (start, end),
                }));
                Ok(())
            }
            State::Slash(start) => Err(LexError(start, ByteIndex(start.0 + '/'.len_utf8()))),
            State::Dash(start) => {
                self.out.push(Token::Dash(start));
                Ok(())
            }
            State::SingleLineComment => Ok(()),
        }
    }
}

fn parse_word(s: &str, start: ByteIndex) -> Option<Token> {
    if let Ok(val) = s.parse::<usize>() {
        return Some(Token::NumberLiteral(NumberLiteral {
            value: val,
            span: (start, ByteIndex(start.0 + s.len())),
        }));
    }

    match s {
        "_" => return Some(Token::Underscore(start)),

        "def" => return Some(Token::DefKw(start)),

        "match" => return Some(Token::MatchKw(start)),
        "fun" => return Some(Token::FunKw(start)),
        "For" => return Some(Token::ForKw(start)),

        "case" => return Some(Token::CaseKw(start)),
        "use" => return Some(Token::UseKw(start)),
        "end" => return Some(Token::EndKw(start)),
        "dec" => return Some(Token::DecKw(start)),

        _ => {}
    }

    if s.starts_with("Set") {
        let level = get_nonzero_number_after_prefix_but_return_zero_if_empty_string(s, "Set")?;
        return Some(Token::CapitalizedUniverseLiteral(
            CapitalizedUniverseLiteral {
                level,
                start,
                erasable: false,
            },
        ));
    }

    if s.starts_with("Prop") {
        let level = get_nonzero_number_after_prefix_but_return_zero_if_empty_string(s, "Prop")?;
        return Some(Token::CapitalizedUniverseLiteral(
            CapitalizedUniverseLiteral {
                level,
                start,
                erasable: true,
            },
        ));
    }

    if s.starts_with("set") {
        let level = get_nonzero_number_after_prefix_but_return_zero_if_empty_string(s, "set")?;
        return Some(Token::LowercaseUniverseLiteral(LowercaseUniverseLiteral {
            level,
            start,
            erasable: false,
        }));
    }

    if s.starts_with("prop") {
        let level = get_nonzero_number_after_prefix_but_return_zero_if_empty_string(s, "prop")?;
        return Some(Token::LowercaseUniverseLiteral(LowercaseUniverseLiteral {
            level,
            start,
            erasable: true,
        }));
    }

    if s.is_empty() {
        return None;
    }

    match s.chars().next().unwrap() {
        'a'..='z' | 'A'..='Z' | '_' => {
            return Some(Token::Ident(Ident {
                value: s.to_owned(),
                start,
            }))
        }
        _ => {}
    }

    None
}

/// - If `&s[prefix.len()..]` is empty, this function returns `Some(0)`.
///
/// - If `&s[prefix.len()..]` is a nonzero number `n` with no leading zeros,
///   this function returns `Some(n)`.
///
/// - Otherwise, this function returns `None`.
///
///   - As a corollary, if `&s[prefix.len()..]` is a number with extraneous leading zeros,
///     this function returns `None`.
fn get_nonzero_number_after_prefix_but_return_zero_if_empty_string(
    s: &str,
    prefix: &str,
) -> Option<usize> {
    let level_src = &s[prefix.len()..];
    if level_src.is_empty() {
        return Some(0);
    }

    if level_src.starts_with('0') {
        return None;
    }

    let Ok(level) = level_src.parse::<usize>() else {
        return None;
    };

    Some(level)
}
