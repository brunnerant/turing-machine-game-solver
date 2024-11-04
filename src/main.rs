use turing_machine_game::dsl::*;
use turing_machine_game::problem::Problem;

fn main() {
    let card1 = Symbol::all_combinations().map(|(s1, s2)| cmps(s1, s2)).reduce(|a, b| a | b).unwrap();
    let card2 = (0..=3).map(|i| countv(3, i) | countv(4, i)).reduce(|a, b| a | b).unwrap();
    let card3 = Symbol::all_symbols().map(|s| Card::empty() | gtv(s, 1)).reduce(|a, b| a | b).unwrap();
    let card4 = more_even() | more_odd();

    let mut problem = Problem::new(vec![card1, card2, card3, card4]);
    problem.disable_insufficient_constraints();
    println!("{}", problem);
}
