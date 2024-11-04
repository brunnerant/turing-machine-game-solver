pub use crate::{card::Card, code::Symbol, constraint::Constraint};

pub fn eqs(s1: Symbol, s2: Symbol) -> Constraint {
    Constraint::new(|c| c[s1] == c[s2], format!("{}={}", s1, s2))
}

pub fn lts(s1: Symbol, s2: Symbol) -> Constraint {
    Constraint::new(|c| c[s1] < c[s2], format!("{}<{}", s1, s2))
}

pub fn gts(s1: Symbol, s2: Symbol) -> Constraint {
    Constraint::new(|c| c[s1] > c[s2], format!("{}>{}", s1, s2))
}

pub fn cmps(s1: Symbol, s2: Symbol) -> Card {
    lts(s1, s2) | eqs(s1, s2) | gts(s1, s2)
}

pub fn eqv(s1: Symbol, v: u8) -> Constraint {
    Constraint::new(|c| c[s1] == v, format!("{}={}", s1, v))
}

pub fn ltv(s1: Symbol, v: u8) -> Constraint {
    Constraint::new(|c| c[s1] < v, format!("{}<{}", s1, v))
}

pub fn gtv(s1: Symbol, v: u8) -> Constraint {
    Constraint::new(|c| c[s1] > v, format!("{}>{}", s1, v))
}

pub fn cmpv(s1: Symbol, v: u8) -> Card {
    let mut card = Card::empty();
    if v > 1 {
        card = card | ltv(s1, v);
    }
    card = card | eqv(s1, v);
    if v < 5 {
        card = card | gtv(s1, v);
    }
    card
}

pub fn countv(v: u8, n: u8) -> Constraint {
    Constraint::new(|c| c.count(|x| x == v) == n, format!("#{}={}", v, n))
}

pub fn more_even() -> Constraint {
    Constraint::new(|c| c.count(|x| x % 2 == 0) > c.count(|x| x % 2 == 1), format!("#even>#odd"))
}

pub fn more_odd() -> Constraint {
    Constraint::new(|c| c.count(|x| x % 2 == 1) > c.count(|x| x % 2 == 0), format!("#odd>#even"))
}
