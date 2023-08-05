use super::*;

use crate::syntax_tree::ost::Span;

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Ind(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Vcon(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Match(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Fun(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::App(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::For(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Deb(e) => e.hashee.span,
            Self::Universe(e) => (
                e.hashee.start,
                ByteIndex(e.hashee.start.0 + "Type".len() + get_digit_count(e.hashee.level)),
            ),
        }
    }
}

impl Ind {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl VconDef {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl Vcon {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl Match {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl MatchCase {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl Fun {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl App {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

impl For {
    pub fn span(&self) -> Span {
        (self.lparen, self.rparen)
    }
}

fn get_digit_count(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _0_has_1_digit() {
        assert_eq!(1, get_digit_count(0));
    }

    #[test]
    fn _1_has_1_digit() {
        assert_eq!(1, get_digit_count(1));
    }

    #[test]
    fn _9_has_1_digit() {
        assert_eq!(1, get_digit_count(9));
    }

    #[test]
    fn _10_has_2_digits() {
        assert_eq!(2, get_digit_count(10));
    }

    #[test]
    fn _11_has_2_digits() {
        assert_eq!(2, get_digit_count(11));
    }

    #[test]
    fn _19_has_2_digits() {
        assert_eq!(2, get_digit_count(19));
    }

    #[test]
    fn _99_has_2_digits() {
        assert_eq!(2, get_digit_count(99));
    }

    #[test]
    fn _100_has_3_digits() {
        assert_eq!(3, get_digit_count(100));
    }

    #[test]
    fn _101_has_3_digits() {
        assert_eq!(3, get_digit_count(101));
    }

    #[test]
    fn _999_has_3_digits() {
        assert_eq!(3, get_digit_count(999));
    }

    #[test]
    fn _1000_has_4_digits() {
        assert_eq!(4, get_digit_count(1000));
    }

    #[test]
    fn _1001_has_4_digits() {
        assert_eq!(4, get_digit_count(1001));
    }

    #[test]
    fn _9999_has_4_digits() {
        assert_eq!(4, get_digit_count(9999));
    }
}
