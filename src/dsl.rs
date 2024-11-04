use itertools::Itertools;
use crate::{code::{Code, Digit}, problem::Problem};
pub use crate::{card::Card, code::Symbol, constraint::Constraint};

const TRI: Symbol = Symbol::Triangle;
const SQU: Symbol = Symbol::Square;
const CIR: Symbol = Symbol::Circle;

pub fn cons<F: Fn(Code) -> bool>(f: F, name: String) -> Constraint {
    Constraint::new(f, name)
}

pub fn eqs(s1: Symbol, s2: Symbol) -> Constraint {
    cons(|c| c[s1] == c[s2], format!("{}={}", s1, s2))
}

pub fn lts(s1: Symbol, s2: Symbol) -> Constraint {
    cons(|c| c[s1] < c[s2], format!("{}<{}", s1, s2))
}

pub fn gts(s1: Symbol, s2: Symbol) -> Constraint {
    cons(|c| c[s1] > c[s2], format!("{}>{}", s1, s2))
}

pub fn smallest(s1: Symbol) -> Constraint {
    let (s2, s3) = Symbol::all_symbols().filter(|s| *s != s1).collect_tuple().unwrap();
    cons(|c| c[s1] < c[s2] && c[s1] < c[s3], format!("{}<{}{}", s1, s2, s3))
}

pub fn biggest(s1: Symbol) -> Constraint {
    let (s2, s3) = Symbol::all_symbols().filter(|s| *s != s1).collect_tuple().unwrap();
    cons(|c| c[s1] > c[s2] && c[s1] > c[s3], format!("{}>{}{}", s1, s2, s3))
}

pub fn eqv(s: Symbol, v: u8) -> Constraint {
    cons(|c| c[s] == v, format!("{}={}", s, v))
}

pub fn ltv(s: Symbol, v: u8) -> Constraint {
    cons(|c| c[s] < v, format!("{}<{}", s, v))
}

pub fn gtv(s: Symbol, v: u8) -> Constraint {
    cons(|c| c[s] > v, format!("{}>{}", s, v))
}

pub fn countv(v: u8, n: u8) -> Constraint {
    cons(|c| c.count(|x| x == v) == n, format!("#{}={}", v, n))
}

pub fn even(n: Digit) -> bool {
    n % 2 == 0
}

pub fn odd(n: Digit) -> bool {
    n % 2 != 0
}

pub fn evens(s: Symbol) -> Constraint {
    cons(|c| even(c[s]), format!("even({})", s))
}

pub fn odds(s: Symbol) -> Constraint {
    cons(|c| odd(c[s]), format!("odd({})", s))
}

pub fn count_even(n: u8) -> Constraint {
    cons(|c| c.count(even) == n, format!("#even={}", n))
}

pub fn count_odd(n: u8) -> Constraint {
    cons(|c| c.count(odd) == n, format!("#odd={}", n))
}

pub fn more_even() -> Constraint {
    cons(|c| c.count(even) > c.count(odd), format!("#even>#odd"))
}

pub fn more_odd() -> Constraint {
    cons(|c| c.count(odd) > c.count(even), format!("#odd>#even"))
}

pub fn sum_even() -> Constraint {
    cons(|c| even(c.sum()), format!("even({}+{}+{})", TRI, SQU, CIR))
}

pub fn sum_odd() -> Constraint {
    cons(|c| odd(c.sum()), format!("odd({}+{}+{})", TRI, SQU, CIR))
}

pub fn distinct(n: u8) -> Constraint {
    cons(|c| c.num_distinct() == n, format!("#distinct={}", n))
}

pub fn card_from_id(id: usize) -> Card {
    match id {
        1 => eqv(TRI, 1) | gtv(TRI, 1),
        2 => ltv(TRI, 3) | eqv(TRI, 3) | gtv(TRI, 3),
        3 => ltv(SQU, 3) | eqv(SQU, 3) | gtv(SQU, 3),
        4 => ltv(CIR, 3) | eqv(CIR, 3) | gtv(CIR, 3),
        5 => evens(TRI) | odds(TRI),
        6 => evens(SQU) | odds(SQU),
        7 => evens(CIR) | odds(CIR),
        8 => countv(1, 0) | countv(1, 1) | countv(1, 2) | countv(1, 3),
        9 => countv(3, 0) | countv(3, 1) | countv(3, 2) | countv(3, 3),
        10 => countv(4, 0) | countv(4, 1) | countv(4, 2) | countv(4, 3),
        11 => lts(TRI, SQU) | eqs(TRI, SQU) | gts(TRI, SQU),
        12 => lts(TRI, CIR) | eqs(TRI, CIR) | gts(TRI, CIR),
        13 => lts(SQU, CIR) | eqs(SQU, CIR) | gts(SQU, CIR),
        14 => smallest(TRI) | smallest(SQU) | smallest(CIR),
        15 => biggest(TRI) | biggest(SQU) | biggest(CIR),
        16 => more_even() | more_odd(),
        17 => count_even(0) | count_even(1) | count_even(2) | count_even(3),
        18 => sum_even() | sum_odd(),
        19 => cons(|c| c[TRI] + c[SQU] < 6, "▲+■<6".into()) | cons(|c| c[TRI] + c[SQU] == 6, "▲+■=6".into()) | cons(|c| c[TRI] + c[SQU] > 6, "▲+■>6".into()),
        20 => distinct(1) | distinct(2) | distinct(3),
        21 => cons(|c| c.num_distinct() != 2, "#distinct≠2".into()) | distinct(2),
        31 => gtv(TRI, 1) | gtv(SQU, 1) | gtv(CIR, 1),
        46 => countv(3, 0) | countv(3, 1) | countv(3, 2) | countv(4, 0) | countv(4, 1) | countv(4, 2),
        48 => Symbol::all_combinations().map(|(s1, s2)| lts(s1, s2) | eqs(s1, s2) | gts(s2, s2)).reduce(|a, b| a | b).unwrap(),
        _ => panic!("Card {} is unknown", id)
    }
}

pub fn problem_from_ids(ids: impl Iterator<Item = usize>) -> Problem {
    Problem::new(ids.map(card_from_id).collect())
}
