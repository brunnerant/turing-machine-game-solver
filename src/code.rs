use std::{fmt::Display, ops::Index};
use itertools::{iproduct, Itertools};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Code {
    pub triangle: Digit,
    pub square: Digit,
    pub circle: Digit,
}

impl Code {
    pub fn new(triangle: u8, square: u8, circle: u8) -> Code {
        Code { triangle, square, circle }
    }
    
    pub fn count<F: Fn(Digit) -> bool>(&self, pred: F) -> u8 {
        pred(self.triangle) as u8 + pred(self.square) as u8 + pred(self.circle) as u8
    }

    pub fn all() -> impl Iterator<Item = Code> {
        iproduct!(1..=5, 1..=5, 1..=5).map(|(a, b, c)| Self::new(a, b, c))
    }
}

impl Index<Symbol> for Code {
    type Output = u8;

    fn index(&self, index: Symbol) -> &Self::Output {
        match index {
            Symbol::Triangle => &self.triangle,
            Symbol::Square => &self.square,
            Symbol::Circle => &self.circle,
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.triangle, self.square, self.circle)
    }
}
