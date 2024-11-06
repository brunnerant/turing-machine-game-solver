use itertools::Itertools;
use crate::code::{Code, Digit};
use crate::problem::Card;
use crate::{code::Symbol, constraint::Constraint};

const TRI: Symbol = Symbol::Triangle;
const SQU: Symbol = Symbol::Square;
const CIR: Symbol = Symbol::Circle;

fn cons<F: Fn(Code) -> bool>(f: F, name: String) -> (String, Constraint) {
    (name, Constraint::new(f))
}

fn eqs(s1: Symbol, s2: Symbol) -> (String, Constraint) {
    cons(|c| c[s1] == c[s2], format!("{}={}", s1, s2))
}

fn lts(s1: Symbol, s2: Symbol) -> (String, Constraint) {
    cons(|c| c[s1] < c[s2], format!("{}<{}", s1, s2))
}

fn gts(s1: Symbol, s2: Symbol) -> (String, Constraint) {
    cons(|c| c[s1] > c[s2], format!("{}>{}", s1, s2))
}

fn smallest(s1: Symbol) -> (String, Constraint) {
    let (s2, s3) = Symbol::all_symbols().filter(|s| *s != s1).collect_tuple().unwrap();
    cons(|c| c[s1] < c[s2] && c[s1] < c[s3], format!("{}<{}{}", s1, s2, s3))
}

fn biggest(s1: Symbol) -> (String, Constraint) {
    let (s2, s3) = Symbol::all_symbols().filter(|s| *s != s1).collect_tuple().unwrap();
    cons(|c| c[s1] > c[s2] && c[s1] > c[s3], format!("{}>{}{}", s1, s2, s3))
}

fn eqv(s: Symbol, v: u8) -> (String, Constraint) {
    cons(|c| c[s] == v, format!("{}={}", s, v))
}

fn ltv(s: Symbol, v: u8) -> (String, Constraint) {
    cons(|c| c[s] < v, format!("{}<{}", s, v))
}

fn gtv(s: Symbol, v: u8) -> (String, Constraint) {
    cons(|c| c[s] > v, format!("{}>{}", s, v))
}

fn numv(v: u8, n: u8) -> (String, Constraint) {
    cons(|c| c.count(|x| x == v) == n, format!("#{}={}", v, n))
}

fn sumeqv(s1: Symbol, s2: Symbol, n: u8) -> (String, Constraint) {
    cons(|c| c[s1] + c[s2] == n, format!("{}+{}={}", s1, s2, n))
}

fn even(n: Digit) -> bool {
    n % 2 == 0
}

fn odd(n: Digit) -> bool {
    n % 2 != 0
}

fn evens(s: Symbol) -> (String, Constraint) {
    cons(|c| even(c[s]), format!("even({})", s))
}

fn odds(s: Symbol) -> (String, Constraint) {
    cons(|c| odd(c[s]), format!("odd({})", s))
}

fn num_even(n: u8) -> (String, Constraint) {
    cons(|c| c.count(even) == n, format!("#even={}", n))
}

fn num_distinct(n: u8) -> (String, Constraint) {
    cons(|c| c.num_distinct() == n, format!("#distinct={}", n))
}

fn num_steps_up(n: u8) -> (String, Constraint) {
    cons(|c| c.count_adj(|a, b| a + 1 == b) == n, format!("#step-up={}", n))
}

pub fn card_from_id(id: usize) -> Card {
    match id {
        1 => Card::new(vec![eqv(TRI, 1), gtv(TRI, 1)]),
        2 => Card::new(vec![ltv(TRI, 3), eqv(TRI, 3), gtv(TRI, 3)]),
        3 => Card::new(vec![ltv(SQU, 3), eqv(SQU, 3), gtv(SQU, 3)]),
        4 => Card::new(vec![ltv(CIR, 3), eqv(CIR, 3), gtv(CIR, 3)]),
        5 => Card::new(vec![evens(TRI), odds(TRI)]),
        6 => Card::new(vec![evens(SQU), odds(SQU)]),
        7 => Card::new(vec![evens(CIR), odds(CIR)]),
        8 => Card::new(vec![numv(1, 0), numv(1, 1), numv(1, 2)]),
        9 => Card::new(vec![numv(3, 0), numv(3, 1), numv(3, 2)]),
        10 => Card::new(vec![numv(4, 0), numv(4, 1), numv(4, 2)]),
        11 => Card::new(vec![lts(TRI, SQU), eqs(TRI, SQU), gts(TRI, SQU)]),
        12 => Card::new(vec![lts(TRI, CIR), eqs(TRI, CIR), gts(TRI, CIR)]),
        13 => Card::new(vec![lts(SQU, CIR), eqs(SQU, CIR), gts(SQU, CIR)]),
        14 => Card::new(vec![smallest(TRI), smallest(SQU), smallest(CIR)]),
        15 => Card::new(vec![biggest(TRI), biggest(SQU), biggest(CIR)]),
        16 => Card::new(vec![
            cons(|c| c.count(even) > c.count(odd), format!("#even>#odd")),
            cons(|c| c.count(odd) > c.count(even), format!("#odd>#even"))]),
        17 => Card::new(vec![num_even(0), num_even(1), num_even(2), num_even(3)]),
        18 => Card::new(vec![
            cons(|c| even(c.sum()), "even(▲+■+●)".into()),
            cons(|c| odd(c.sum()), "odd(▲+■+●)".into())]),
        19 => Card::new(vec![
            cons(|c| c[TRI] + c[SQU] < 6, "▲+■<6".into()),
            sumeqv(TRI, SQU, 6),
            cons(|c| c[TRI] + c[SQU] > 6, "▲+■>6".into())]),
        20 => Card::new(vec![num_distinct(1), num_distinct(2), num_distinct(3)]),
        21 => Card::new(vec![cons(|c| c.num_distinct() != 2, "#distinct≠2".into()), num_distinct(2)]),
        22 => Card::new(vec![
            cons(|c| c.count_adj(|a, b| a < b) == 2, "▲<■<●".into()),
            cons(|c| c.count_adj(|a, b| a > b) == 2, "▲>■>●".into()),
            cons(|c| c.count_adj(|a, b| a < b) != 2 && c.count_adj(|a, b| a > b) != 2, "not(▲<■<●|▲>■>●)".into())]),
        23 => Card::new(vec![
            cons(|c| c.sum() < 6, "▲+■+●<6".into()),
            cons(|c| c.sum() == 6, "▲+■+●=6".into()),
            cons(|c| c.sum() > 6, "▲+■+●>6".into())]),
        24 => Card::new(vec![num_steps_up(2), num_steps_up(1), num_steps_up(0)]),
        31 => Card::new(vec![gtv(TRI, 1), gtv(SQU, 1), gtv(CIR, 1)]),
        33 => Card::new(Symbol::all_symbols().flat_map(|s| vec![evens(s), odds(s)]).collect()),
        38 => Card::new(Symbol::all_combinations().map(|(s1, s2)| sumeqv(s1, s2, 6)).collect()),
        40 => Card::new(Symbol::all_symbols().flat_map(|s| vec![ltv(s, 3), eqv(s, 3), gtv(s, 3)]).collect()),
        44 => Card::new(vec![lts(SQU, TRI), eqs(SQU, TRI), gts(SQU, TRI), lts(SQU, CIR), eqs(SQU, CIR), gts(SQU, CIR)]),
        46 => Card::new(vec![numv(3, 0), numv(3, 1), numv(3, 2), numv(4, 0), numv(4, 1), numv(4, 2)]),
        48 => Card::new(Symbol::all_combinations().flat_map(|(s1, s2)| vec![lts(s1, s2), eqs(s1, s2), gts(s2, s2)]).collect()),
        _ => panic!("Card {} is unknown", id),
    }
}
