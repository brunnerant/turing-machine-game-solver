use std::env;
use itertools::Itertools;
use turing_machine_game::dsl::*;
use turing_machine_game::solve::{Solver, SolverError};

fn main() {
    let ids = env::args().skip(1).map(|arg| arg.parse()).collect_vec();
    if ids.is_empty() || ids.iter().any(|v| v.is_err()) {
        println!("Please pass the IDs of the cards as argument to the script.");
        return;
    }

    let problem = problem_from_ids(ids.into_iter().map(|v| v.unwrap()));
    let mut solver = Solver::new(problem);
    match solver.solve() {
        Ok(sol) => {
            println!("Found solution: {}", sol);
            println!("Number of rounds: {}", solver.num_rounds());
            println!("Number of questions: {}", solver.num_questions());
        },
        Err(SolverError::InvalidCard(card)) => {
            println!("Card {} is invalid because all its constraints are impossible.", card.letter);
            println!("You might have entered a wrong value, or the problem is ill-defined.");
        },
        Err(SolverError::MultipleSolutions(sols)) => {
            println!("The set of cards leads to several solutions: {}", sols.iter().map(|s| format!("{}", s)).join(", "));
            println!("You might have entered a wrong value, or the problem is ill-defined.");
        },
    }

    println!();
    solver.print_state();
}
