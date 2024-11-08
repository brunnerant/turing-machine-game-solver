use std::fs;

use colored::Colorize;
use itertools::Itertools;
use json::JsonValue;
use turing_machine_game::{cards::{card_from_id, constraint_from_id}, code::Code, problem::{Problem, ProblemMode}, solve::{AutomaticVerifier, Solver}};


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

fn success(mode: ProblemMode, diff: &str, cards: usize, rounds: usize, questions: usize, questions_ai: usize) {
    let color = if questions > questions_ai { "red" } else if questions == questions_ai { "yellow" } else { "green" };
    let mode = mode.to_string();
    let cards = cards.to_string();
    let rounds = rounds.to_string();
    let questions = questions.to_string().color(color);
    let questions_ai = questions_ai.to_string().color(color);
    println!("{:^11}│{:^12}│{:^7}│{:^4}│{:^4}│{:^4}", mode, diff, cards, rounds, questions, questions_ai);
}

fn failure(mode: ProblemMode, diff: &str, cards: usize) {
    println!("{}", format!("{:^11}│{:^12}│{:^7}│    │    │    ", mode, diff, cards).on_red());
}

fn main() {
    println!("   mode    │ difficulty │ cards │ R  │ Q  │ QAI");
    println!("───────────┼────────────┼───────┼────┼────┼────");

    let problems = json::parse(fs::read_to_string("data/games.json").unwrap().as_str()).unwrap();
    for obj in problems.members() {
        let (a, b, c) = obj["solution"].members().map(|d| d.as_u8().unwrap()).collect_tuple().unwrap();
        let constraints: Vec<_> = obj["laws"].members().map(|id| constraint_from_id(id.as_u8().unwrap()).1).collect();
        let solution = Code::new(a, b, c);
        let problem = json_to_problem(obj);
        let diff = obj["difficulty"].as_str().unwrap();
        let mut questions_ai = obj["num-questions-ai"].as_usize().unwrap();
        if problem.mode != ProblemMode::Normal {
            questions_ai = (questions_ai as f32 * 1.5).ceil() as usize;
        }

        let mut solver = Solver::<AutomaticVerifier>::new(&problem).automatic(constraints.clone());
        match solver.solve() {
            Ok(code) if code == solution => success(problem.mode, diff, constraints.len(), solver.num_rounds(), solver.num_questions(), questions_ai),
            _ => failure(problem.mode, diff, constraints.len()),
        }
    }
}
