use std::{fmt::Debug, ops::BitAnd};
use itertools::Itertools;
use crate::code::*;


#[derive(Clone, Copy)]
pub struct Constraint {
    value: u128,
}

impl Constraint {
    const GROUP_MASK: u128 = (1 << 125) - 1;

    pub fn none() -> Self {
        Constraint { value: !0 }
    }

    pub fn inter(cs: impl Iterator<Item = Constraint>) -> Constraint {
        cs.fold(Self::none(), |a, b| a & b)
    }

    pub fn is_superset_of(&self, other: &Constraint) -> bool {
        let v = other.with_group(0).value;
        self.value & v == v
    }

    pub fn new<F: Fn(Code) -> bool>(f: F) -> Self {
        let mut value = 0;
        for (i, code) in Code::all().enumerate() {
            if f(code) {
                value |= 1 << i
            }
        }
        Constraint { value }
    }

    pub fn with_group(&self, group: u8) -> Self {
        Constraint { value: (self.value & Self::GROUP_MASK) | (group as u128) << 125 }
    }

    pub fn group(&self) -> u8 {
        (self.value >> 125) as u8
    }

    pub fn accepts(&self, code: Code) -> bool {
        self.value & (1 << code.index()) != 0
    }

    pub fn num_solutions(&self) -> u32 {
        (self.value & Self::GROUP_MASK).count_ones()
    }

    pub fn has_unique_solution(&self) -> bool {
        self.num_solutions() == 1
    }

    pub fn solutions(&self) -> impl Iterator<Item = Code> {
        let v = self.value;
        Code::all().enumerate().filter_map(move |(i, c)| if v & (1 << i) != 0 { Some(c) } else { None })
    }

    pub fn solution(&self) -> Option<Code> {
        if self.has_unique_solution() {
            let idx = self.value.trailing_zeros();
            Some(Code::new((idx / 25 + 1) as Digit, ((idx / 5) % 5 + 1) as Digit, (idx % 5 + 1) as Digit))
        } else {
            None
        }
    }
}

impl BitAnd for Constraint {
    type Output = Constraint;

    fn bitand(self, rhs: Self) -> Self::Output {
        Constraint { value: self.value & rhs.value }
    }
}

impl Debug for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.solutions().map(|c| format!("{}", c)).join(", "))
    }
}
