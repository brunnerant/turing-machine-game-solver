use std::{collections::HashMap, iter::repeat};
use itertools::Itertools;
use num::Rational32;
use crate::{code::{Code, Symbol}, constraint::Constraint, problem::{Problem, ProblemMode}};

pub enum SolverError {
    Impossible(char),
    MultipleSolutions(Vec<Code>),
}

pub struct Solver {
    verifiers: Vec<Vec<Constraint>>, // The set of constraints for every verifier
    questions: Vec<Code>, // The questions that were asked
    answers: Vec<HashMap<usize, bool>>, // The answers that were given
}

impl Solver {
    pub fn new(problem: &Problem) -> Solver {
        let verifiers: Vec<Vec<_>> = match problem.mode {
            ProblemMode::Normal => {
                Self::assign_groups(problem.cards.iter()
                    .map(|c| c.constraints.values().copied().collect())
                    .collect())
            },
            ProblemMode::Extreme => {
                Self::assign_groups(problem.cards.iter()
                    .chunks(2).into_iter()
                    .map(|b| b.flat_map(|c| c.constraints.values().copied()).collect())
                    .collect())
            },
            ProblemMode::Nightmare => {
                let constraints = Self::assign_groups(
                        problem.cards.iter()
                        .map(|c| c.constraints.values().copied().collect())
                        .collect()
                    ).into_iter().flatten().collect();
                repeat(constraints).take(problem.cards.len()).collect()
            }
        };
        Solver { verifiers, questions: Vec::new(), answers: Vec::new() }
    }

    fn assign_groups(mut constraints: Vec<Vec<Constraint>>) -> Vec<Vec<Constraint>> {
        constraints.iter_mut().enumerate()
            .for_each(|(i, cs)| cs.into_iter().for_each(|c| c.set_group(i as u8)));
        constraints
    }

    pub fn num_rounds(&self) -> usize {
        self.questions.len()
    }

    pub fn num_questions(&self) -> usize {
        self.answers.iter().map(|a| a.len()).sum()
    }

    pub fn print_state(&self) {
        if self.questions.is_empty() {
            println!("No questions asked yet.");
        } else {
            println!("{} │ {}", Symbol::all_symbols().join(""), "ABCDEF".chars().take(self.verifiers.len()).join(" "));
            println!("────┼{}", "─".repeat(2 * self.verifiers.len()));
            for (q, a) in self.questions.iter().zip(self.answers.iter()) {
                let mut answers = (0..self.verifiers.len())
                    .map(|i| a.get(&i).map_or("☐", |&r| if r { "☑" } else { "☒" }));
                println!("{} │ {}", format!("{}", q), answers.join(" "))
            }
        }
    }

    fn has_solution(&self) -> Result<Option<Code>, SolverError> {
        let known_constraints = self.verifiers.iter()
            .map(|cs| if cs.len() == 1 { Some(cs[0]) } else { None }).collect::<Vec<_>>();

        if known_constraints.iter().all(|c| c.is_some()) {
            let cons = known_constraints.iter().flatten().fold(Constraint::none(), |a, &b| a & b.clone());
            cons.solution().map_or_else(
                || Err(SolverError::MultipleSolutions(cons.solutions().collect())),
                |s| Ok(Some(s)))
        } else {
            Ok(None)
        }
    }

    // TODO: Remove redundant constraints ?
    // Constraints that make all the constraints of another card redundant

    fn best_question(&self) -> Code {
        let mut questions = Vec::with_capacity(125);
        for c in Code::all() {
            let elims = self.verifiers.iter().map(|card| Self::expected_eliminations(card, c));
            let total_elims = elims.sorted().rev().take(3).sum::<Rational32>();
            questions.push((total_elims, c));
        }
        questions.sort_by_key(|(e, _)| *e);
        questions.pop().unwrap().1
    }

    fn best_verifier_for_question(&self, code: Code) -> Option<usize> {
        let elims = self.verifiers.iter().map(|v| Self::expected_eliminations(v, code));
        let (v_idx, total_elims) = elims.enumerate().max_by_key(|(_, e)| *e).unwrap();
        if total_elims == Rational32::ZERO { None } else { Some(v_idx) } 
    }

    fn expected_eliminations(verifier: &Vec<Constraint>, code: Code) -> Rational32 {
        let counts = verifier.iter().map(|c| c.accepts(code)).counts();
        let n0 = counts.get(&false).copied().unwrap_or(0) as i32;
        let n1 = counts.get(&true).copied().unwrap_or(0) as i32;
        if n0 + n1 == 0 { Rational32::ZERO } else { Rational32::new_raw(2 * n0 * n1, n0 + n1) }
    }

    pub fn question(&mut self, code: Code, verifier: usize) -> Result<bool, SolverError> {
        // Retrieve the card that we are querying
        let constraints = &mut self.verifiers[verifier];
        let letter = "ABCDEF".chars().nth(verifier).unwrap();

        // Ask the user for the result
        println!("Please type in the answer of verifier {} for the code ▲■●={}", letter, code);
        let answer = input_validation::get_bool("Answer [y/n] > ");
        println!();
        
        // From the answer, eliminate the constraints that didn't agree
        constraints.retain(|c| c.accepts(code) == answer);

        // Eliminate constraints
        Ok(answer)
    }

    pub fn round(&mut self, code: Code) -> Result<(), SolverError> {
        // Start the round of questions
        self.questions.push(code);
        let mut answers = HashMap::new();

        for _ in 0..3 {
            let v_idx = if let Some(i) = self.best_verifier_for_question(code) { i } else {
                self.answers.push(answers);
                return Ok(())
            };
            let answer = self.question(code, v_idx)?;
            answers.insert(v_idx, answer);
        }

        self.answers.push(answers);
        Ok(())
    }

    pub fn solve(&mut self) -> Result<Code, SolverError> {
        let mut round = 1;
        // Eliminate

        loop {
            println!("--- Round {} ---", round);
            self.print_state();

            let c = self.best_question();
            self.round(c)?;

            if let Some(sol) = self.has_solution()? {
                return Ok(sol);
            }

            round += 1;
        }
    }
}
