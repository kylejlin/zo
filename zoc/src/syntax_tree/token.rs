use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

impl Add for ByteIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ByteIndex(self.0 + rhs.0)
    }
}

impl Sub for ByteIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ByteIndex(self.0 - rhs.0)
    }
}

pub type Span = (ByteIndex, ByteIndex);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct NumberLiteral {
    pub value: usize,
    pub span: (ByteIndex, ByteIndex),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringLiteral {
    pub value: String,
    pub span: (ByteIndex, ByteIndex),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct UniverseLiteral {
    pub level: usize,
    // TODO: Replace this with a `span` field.
    pub start: ByteIndex,
    /// This is true if the literal is a `Prop`
    /// and false if the literal is a `Set`.
    pub erasable: bool,
}

pub use crate::syntax_tree::parser::Token;

impl UniverseLiteral {
    pub fn span(&self) -> Span {
        let kw_len = if self.erasable {
            "Prop".len()
        } else {
            "Set".len()
        };
        let end = ByteIndex(self.start.0 + kw_len + get_digit_count(self.level));
        (self.start, end)
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
