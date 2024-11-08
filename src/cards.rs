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
    cons(|c| c[s1] <= c[s2] && c[s1] <= c[s3], format!("{}≤{}{}", s1, s2, s3))
}

fn biggest(s1: Symbol) -> (String, Constraint) {
    let (s2, s3) = Symbol::all_symbols().filter(|s| *s != s1).collect_tuple().unwrap();
    cons(|c| c[s1] >= c[s2] && c[s1] >= c[s3], format!("{}≥{}{}", s1, s2, s3))
}

fn strictly_smallest(s1: Symbol) -> (String, Constraint) {
    let (s2, s3) = Symbol::all_symbols().filter(|s| *s != s1).collect_tuple().unwrap();
    cons(|c| c[s1] < c[s2] && c[s1] < c[s3], format!("{}<{}{}", s1, s2, s3))
}

fn strictly_biggest(s1: Symbol) -> (String, Constraint) {
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
    cons(|c| c.count_adj(|a, b| a + 1 == b) == n, format!("#steps-up={}", n))
}

fn num_steps(n: u8) -> (String, Constraint) {
    cons(|c| c.count_adj(|a, b| a + 1 == b).max(c.count_adj(|a, b| a - 1 == b)) == n, format!("#steps={}", n))
}

pub fn constraint_from_id(id: u8) -> (String, Constraint) {
    match id {
        1 | 2 | 3 | 4 | 5 => eqv(TRI, id),
        6 | 7 | 8 | 9 | 10 => eqv(SQU, id - 5),
        11 | 12 | 13 | 14 | 15 => eqv(CIR, id - 10),
        16 | 17 | 18 => gtv(TRI, id - 15),
        19 | 20 | 21 => gtv(SQU, id - 18),
        22 | 23 | 24 => gtv(CIR, id - 21),
        25 | 26 | 27 => ltv(TRI, id - 22),
        28 | 29 | 30 => ltv(SQU, id - 25),
        31 | 32 | 33 => ltv(CIR, id - 28),
        34 => evens(TRI),
        35 => evens(SQU),
        36 => evens(CIR),
        37 => odds(TRI),
        38 => odds(SQU),
        39 => odds(CIR),
        40 | 41 | 42 => numv(1, id - 40),
        43 | 44 | 45 => numv(2, id - 43),
        46 | 47 | 48 => numv(3, id - 46),
        49 | 50 | 51 => numv(4, id - 49),
        52 | 53 | 54 => numv(5, id - 52),
        55 => cons(|c| even(c.sum()), "even(▲+■+●)".into()),
        56 => cons(|c| odd(c.sum()), "odd(▲+■+●)".into()),
        57 | 58 | 59 => cons(|c| c.sum() % (id - 54) == 0, format!("▲+■+●={}n", id - 54)),
        60 | 61 | 62 | 63 | 64 | 65 | 66 => cons(|c| c.sum() == id - 54, format!("▲+■+●={}", id - 54)),
        67 | 68 | 69 | 70 | 71 | 72 | 73 => cons(|c| c.sum() > id - 61, format!("▲+■+●>{}", id - 61)),
        74 | 75 | 76 | 77 | 78 | 79 | 80 => cons(|c| c.sum() < id - 68, format!("▲+■+●<{}", id - 68)),
        81 => cons(|c| c.num_distinct() != 2, "#distinct≠2".into()),
        82 => num_distinct(2),
        83 => num_steps_up(0),
        84 => num_steps_up(1),
        85 | 86 | 87 | 88 => num_even(id - 85),
        89 => eqs(TRI, SQU),
        90 => eqs(TRI, CIR),
        91 => eqs(SQU, CIR),
        92 => gts(TRI, SQU),
        93 => gts(TRI, CIR),
        94 => gts(SQU, TRI),
        95 => gts(SQU, CIR),
        96 => gts(CIR, TRI),
        97 => gts(CIR, SQU),
        98 | 99 | 100 | 101 | 102 => cons(|c| c[TRI] + c[SQU] == id - 94, format!("▲+■={}", id - 94)),
        103 | 104 | 105 | 106 | 107 => cons(|c| c[TRI] + c[CIR] == id - 99, format!("▲+●={}", id - 99)),
        108 | 109 | 110 | 111 | 112 => cons(|c| c[SQU] + c[CIR] == id - 104, format!("■+●={}", id - 104)),
        113 => strictly_biggest(TRI),
        114 => strictly_biggest(SQU),
        115 => strictly_biggest(CIR),
        116 => strictly_smallest(TRI),
        117 => strictly_smallest(SQU),
        118 => strictly_smallest(CIR),
        119 | 120 | 121 => num_distinct(id - 118),
        122 | 123 | 124 => num_steps(id - 122),
        125 => biggest(TRI),
        126 => biggest(SQU),
        127 => biggest(CIR),
        128 => smallest(TRI),
        129 => smallest(SQU),
        130 => smallest(CIR),
        131 => cons(|c| c.count(even) > c.count(odd), format!("#even>#odd")),
        132 => cons(|c| c.count(odd) > c.count(even), format!("#odd>#even")),
        133 => cons(|c| c.count_adj(|a, b| a < b) == 2, "▲<■<●".into()),
        134 => cons(|c| c.count_adj(|a, b| a > b) == 2, "▲>■>●".into()),
        135 => cons(|c| c.count_adj(|a, b| a < b) != 2 && c.count_adj(|a, b| a > b) != 2, "not(▲<■<●|▲>■>●)".into()),
        136 => cons(|c| c[TRI] + c[SQU] > 6, "▲+■>6".into()),
        137 => cons(|c| c[TRI] + c[SQU] < 6, "▲+■<6".into()),
        138 => gtv(SQU, 4),
        139 => lts(TRI, SQU),
        140 => lts(TRI, CIR),
        141 => lts(SQU, CIR),
        142 => gtv(TRI, 4),
        143 => gtv(CIR, 4),
        144 => lts(SQU, TRI),
        145 => eqs(SQU, TRI),
        _ => panic!("Constraint {} is unknown", id),
    }
}

fn card_from_ids<const N: usize>(ids: [u8; N]) -> Card {
    Card::new(ids.into_iter().map(constraint_from_id).collect())
}

pub fn card_from_id(id: u8) -> Card {
    match id {
        1 => card_from_ids([1, 16]),
        2 => card_from_ids([25, 3, 18]),
        3 => card_from_ids([28, 8, 21]),
        4 => card_from_ids([29, 9, 138]),
        5 => card_from_ids([34, 37]),
        6 => card_from_ids([35, 38]),
        7 => card_from_ids([36, 39]),
        8 => card_from_ids([40, 41, 42]),
        9 => card_from_ids([46, 47, 48]),
        10 => card_from_ids([49, 50, 51]),
        11 => card_from_ids([139, 89, 92]),
        12 => card_from_ids([140, 90, 93]),
        13 => card_from_ids([141, 91, 95]),
        14 => card_from_ids([116, 117, 118]),
        15 => card_from_ids([113, 114, 115]),
        16 => card_from_ids([131, 132]),
        17 => card_from_ids([85, 86, 87, 88]),
        18 => card_from_ids([55, 56]),
        19 => card_from_ids([137, 100, 136]),
        20 => card_from_ids([119, 120, 121]),
        21 => card_from_ids([81, 82]),
        22 => card_from_ids([133, 134, 135]),
        23 => card_from_ids([74, 60, 67]),
        24 => Card::new(vec![num_steps_up(2), num_steps_up(1), num_steps_up(0)]),
        25 => card_from_ids([122, 123, 124]),
        26 => card_from_ids([25, 28, 31]),
        27 => card_from_ids([26, 29, 32]),
        28 => card_from_ids([1, 6, 11]),
        29 => card_from_ids([3, 8, 13]),
        30 => card_from_ids([4, 9, 14]),
        31 => card_from_ids([16, 19, 22]),
        32 => card_from_ids([18, 21, 24]),
        33 => card_from_ids([34, 37, 35, 38, 36, 39]),
        34 => card_from_ids([128, 129, 130]),
        35 => card_from_ids([125, 126, 127]),
        36 => card_from_ids([57, 58, 59]),
        37 => card_from_ids([98, 103, 108]),
        38 => card_from_ids([100, 105, 110]),
        39 => card_from_ids([1, 16, 6, 19, 11, 22]),
        40 => card_from_ids([25, 3, 18, 28, 8, 21, 31, 13, 24]),
        41 => card_from_ids([26, 4, 142, 29, 9, 138, 32, 14, 143]),
        42 => card_from_ids([116, 113, 117, 114, 118, 115]),
        43 => card_from_ids([139, 140, 89, 90, 92, 93]),
        44 => card_from_ids([144, 141, 145, 91, 94, 95]),
        45 => card_from_ids([40, 41, 42, 46, 47, 48]),
        46 => card_from_ids([46, 47, 48, 49, 50, 51]),
        47 => card_from_ids([40, 41, 41, 49, 50, 51]),
        48 => card_from_ids([139, 89, 92, 140, 90, 93, 141, 91, 95]),
        _ => panic!("Card {} is unknown", id),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, iter::repeat};

    use itertools::Itertools;
    use json::JsonValue;
    use crate::{code::Code, constraint::Constraint, problem::{Problem, ProblemMode}};
    use super::{card_from_id, constraint_from_id};

    fn json_to_problem(obj: &JsonValue) -> Problem {
        let mode = match obj["mode"].as_str() {
            Some("normal") => ProblemMode::Normal,
            Some("extreme") => ProblemMode::Extreme,
            Some("nightmare") => ProblemMode::Nightmare,
            _ => panic!("Invalid game mode encountered.")
        };
        let cards = obj["cards"].members().map(|id| card_from_id(id.as_u8().unwrap())).collect();
        Problem::from_cards(mode, cards)
    }

    #[test]
    pub fn problems_are_well_defined() {
        let problems = json::parse(fs::read_to_string("data/games.json").unwrap().as_str()).unwrap();
        for obj in problems.members() {
            let (a, b, c) = obj["solution"].members().map(|d| d.as_u8().unwrap()).collect_tuple().unwrap();
            let solution = Code::new(a, b, c);
            let constraints = obj["laws"].members().map(|id| constraint_from_id(id.as_u8().unwrap()).1);
            let problem = json_to_problem(obj);
            
            // The intersection of the constraints should give the unique solution of the problem
            assert_eq!(Constraint::inter(constraints.clone()).solution(), Some(solution));
            
            // Each verifier should have a single constraint associated with it
            let possible_constraints: Vec<Vec<_>> = match problem.mode {
                ProblemMode::Normal => problem.cards.iter().map(|c| c.constraints()).collect(),
                ProblemMode::Extreme => problem.cards.iter().chunks(2).into_iter().map(|c| c.flat_map(|c| c.constraints()).collect()).collect(),
                ProblemMode::Nightmare => {
                    let constraints = problem.cards.iter().flat_map(|c| c.constraints()).collect();
                    repeat(constraints).take(problem.cards.len()).collect()
                },
            };

            for (constraint, possible) in constraints.zip(possible_constraints) {
                assert_eq!(possible.into_iter().filter(|&c| c == constraint).count(), 1, "{}", obj);
            }
        }
    }
}
