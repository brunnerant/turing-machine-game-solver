use std::{fmt::Display, ops::Index};
use itertools::{iproduct, Itertools};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Triangle, Square, Circle
}

impl Symbol {
    pub fn all_symbols() -> impl Iterator<Item = Symbol> {
        [Self::Triangle, Self::Square, Self::Circle].into_iter()
    }

    pub fn all_combinations() -> impl Iterator<Item = (Symbol, Symbol)> {
        Self::all_symbols().combinations(2).map(|s| (s[0], s[1]))
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Triangle => write!(f, "▲"),
            Symbol::Square => write!(f, "■"),
            Symbol::Circle => write!(f, "●"),
        }
    }
}

pub type Digit = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    digits: [Digit; 3],
}

impl Code {
    pub fn new(triangle: u8, square: u8, circle: u8) -> Code {
        Code { digits: [triangle, square, circle] }
    }
    
    pub fn all() -> impl Iterator<Item = Code> {
        iproduct!(1..=5, 1..=5, 1..=5).map(|(a, b, c)| Self::new(a, b, c))
    }

    pub fn index(&self) -> u8 {
        25 * self.digits[0] + 5 * self.digits[1] + self.digits[2] - 31
    }

    pub fn count<F: Fn(Digit) -> bool>(&self, pred: F) -> u8 {
        self.digits.iter().filter(|&&d| pred(d)).count() as u8
    }

    pub fn sum(&self) -> u8 {
        self.digits.iter().sum::<u8>()
    }

    pub fn num_distinct(&self) -> u8 {
        self.digits.iter().unique().count() as u8
    }
    
    pub fn count_adj<F: Fn(Digit, Digit) -> bool>(&self, f: F) -> u8 {
        f(self[0], self[1]) as u8 + f(self[1], self[2]) as u8
    }
}

impl Index<Symbol> for Code {
    type Output = Digit;

    fn index(&self, index: Symbol) -> &Self::Output {
        &self.digits[index as usize]
    }
}

impl Index<u8> for Code {
    type Output = Digit;

    fn index(&self, index: u8) -> &Self::Output {
        &self.digits[index as usize]
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self[0], self[1], self[2])
    }
}
