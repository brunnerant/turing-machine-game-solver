use std::{fmt::{Debug, Display}, ops::BitAnd};
use itertools::Itertools;
use crate::code::*;


#[derive(Clone)]
pub struct Constraint {
    value: u128,
    name: String,
    disabled: bool,
}

impl Constraint {
    pub fn none() -> Self {
        Constraint { value: !0, name: "none".into(), disabled: false }
    }

    pub fn new<F: Fn(Code) -> bool>(f: F, name: String) -> Self {
        let mut value = 0;
        for (i, code) in Code::all().enumerate() {
            if f(code) {
                value |= 1 << i
            }
        }
        Constraint { value, name, disabled: false }
    }

    pub fn accepts(&self, code: Code) -> bool {
        self.value & (1 << (25 * code.triangle + 5 * code.square + code.circle)) != 0
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn select(&mut self) {
        self.name = format!("✅{}", self.name);
    }

    pub fn num_solutions(&self) -> u32 {
        self.value.count_ones()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
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
            Some(Code::new((idx / 25) as Digit, ((idx / 5) % 5) as Digit, (idx % 5) as Digit))
        } else {
            None
        }
    }
}

impl BitAnd for Constraint {
    type Output = Constraint;

    fn bitand(self, rhs: Self) -> Self::Output {
        Constraint { value: self.value & rhs.value, name: format!("{} & {}", self, rhs), disabled: false }
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.disabled { "⛔" } else { "" };
        write!(f, "{}{}", prefix, self.name)
    }
}

impl Debug for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.solutions().map(|c| format!("{}", c)).join(", "))
    }
}
