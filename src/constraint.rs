use std::{fmt::Debug, ops::BitAnd};
use itertools::Itertools;
use crate::code::*;


#[derive(Clone, Copy)]
pub struct Constraint {
    value: u128,
}

impl Constraint {
    pub fn none() -> Self {
        Constraint { value: !0 }
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

    pub fn set_group(&mut self, group: u8) {
        self.value = (self.value & ((1 << 125) - 1)) | (group as u128) << 125;
    }

    pub fn group(&self) -> u8 {
        (self.value >> 125) as u8
    }

    pub fn accepts(&self, code: Code) -> bool {
        self.value & (1 << code.index()) != 0
    }

    pub fn num_solutions(&self) -> u32 {
        self.value.count_ones()
    }

    pub fn is_sufficient(&self) -> bool {
        self.num_solutions() == 1
    }

    pub fn solutions(&self) -> impl Iterator<Item = Code> {
        let v = self.value;
        Code::all().enumerate().filter_map(move |(i, c)| if v & (1 << i) != 0 { Some(c) } else { None })
    }

    pub fn solution(&self) -> Option<Code> {
        if self.is_sufficient() {
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
