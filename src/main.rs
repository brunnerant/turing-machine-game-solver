use std::env;
use itertools::Itertools;
use turing_machine_game::{problem::{Problem, ProblemMode}, solve::{Solver, SolverError}};

fn main() {
    let mut args = env::args().skip(1);
    let mode = match args.nth(0).as_deref() {
        Some("normal") => ProblemMode::Normal,
        Some("extreme") => ProblemMode::Extreme,
        Some("nightmare") => ProblemMode::Nightmare,
        _ => {
            println!("Please pass a valid problem mode as the first argument.");
            return;
        },
    };
    
    let ids = args.map(|arg| arg.parse()).collect_vec();
    if ids.is_empty() || ids.iter().any(|v| v.is_err()) {
        println!("Please pass the IDs of the cards as argument to the script.");
        return;
    }

    let problem = Problem::from_card_ids(mode, ids.into_iter().map(|v| v.unwrap()).collect());
    println!("{}", problem);

    let mut solver = Solver::new(&problem);
    match solver.solve() {
        Ok(sol) => {
            println!("Found solution: {}", sol);
            println!("Number of rounds: {}", solver.num_rounds());
            println!("Number of questions: {}", solver.num_questions());
        }
        Err(SolverError::Impossible(v)) => {
            println!("Verifier {} is invalid because all its constraints are impossible.", v);
            println!("You might have entered a wrong value, or the problem is ill-defined.");
        }
        Err(SolverError::MultipleSolutions(sols)) => {
            println!("The set of cards leads to several solutions: {}", sols.iter().map(|s| format!("{}", s)).join(", "));
            println!("You might have entered a wrong value, or the problem is ill-defined.");
        }
    }

    println!();
    solver.print_state();
}
