use std::{collections::HashMap, ptr};
use itertools::Itertools;
use num::Rational32;
use crate::{card::Card, code::{Code, Symbol}, constraint::Constraint, problem::{CardAssignment, Problem}};

pub enum SolverError {
    InvalidCard(CardAssignment),
    MultipleSolutions(Vec<Code>),
}

pub struct Solver {
    problem: Problem,
    questions: Vec<Code>,
    answers: Vec<HashMap<usize, bool>>,
}

impl Solver {
    pub fn new(problem: Problem) -> Solver {
        Solver { problem, questions: Vec::new(), answers: Vec::new() }
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
            println!("{} | {}", Symbol::all_symbols().join(""), self.problem.cards.iter().map(|c| c.letter).join(" "));
            for (q, a) in self.questions.iter().zip(self.answers.iter()) {
                let mut answers = (0..self.problem.cards.len())
                    .map(|i| a.get(&i).map_or("☐", |&r| if r { "☑" } else { "☒" }));
                println!("{} | {}", format!("{}", q), answers.join(" "))
            }
        }
        println!("{:+}", self.problem);
    }

    pub fn has_solution(&self) -> Result<Option<Code>, SolverError> {
        let known_constraints = self.problem.cards.iter().map(|c| c.card.known_constraint()).collect::<Vec<_>>();
        if known_constraints.iter().all(|c| c.is_some()) {
            let cons = known_constraints.iter().flatten().fold(Constraint::none(), |a, &b| a & b.clone());
            cons.solution().map_or_else(|| Err(SolverError::MultipleSolutions(cons.solutions().collect())), |s| Ok(Some(s)))
        } else {
            Ok(None)
        }
    }
    
    // Disables the constraints that are insufficient for the problem. A constraint is insufficient if there does
    // not exist any combination of the other constraints that gives a unique solution. Selecting such constraints
    // would make the problem poorly-defined because of the multiplicity of the solution, so we can safely disable
    // such constraints.
    pub fn disable_insufficient_constraints(&mut self) {
        loop {
            // Stop the loop if there are no more insufficient constraints
            let to_remove = self.insufficient_constraints();
            if to_remove.iter().all(|c| c.is_empty()) {
                break;
            }

            // Otherwise, disable the insufficient constraints and continue
            for (card, to_remove) in self.problem.cards_mut().zip(to_remove) {
                for i in to_remove {
                    card.constraints[i].disable();
                }
            }
        }
    }

    fn insufficient_constraints(&self) -> Vec<Vec<usize>> {
        let mut to_remove = Vec::new();
        for card_idx in 0..self.problem.cards.len() {
            let card = &self.problem.cards[card_idx].card;
            let mut indices = Vec::new();

            for constraint_idx in 0..card.constraints.len() {
                let constraint = &card.constraints[constraint_idx];
                if constraint.is_disabled() {
                    continue;
                }
                let other_cards = self.problem.cards().filter(|&c| !ptr::eq(c, card));
                let mut sufficient = false;
                
                // For each constraint, check if it is sufficient. That is, if there exists some combination
                // of the other constraints that gives a unique solution. Otherwise, this constraint should
                // not be selected because it doesn't give a unique solution.
                for other_constraints in other_cards.map(|c| c.constraints.iter()).multi_cartesian_product() {
                    if other_constraints.into_iter().fold(constraint.clone(), |c1, c2| c1 & c2.clone()).is_sufficient() {
                        sufficient = true;
                        break;
                    }
                }

                if !sufficient {
                    indices.push(constraint_idx);
                }
            }

            to_remove.push(indices);
        }
        to_remove
    }

    // TODO: Remove redundant constraints ?
    // Constraints that make all the constraints of another card redundant

    pub fn best_question(&self) -> Code {
        let mut questions = Vec::with_capacity(125);
        for c in Code::all() {
            let elims = self.problem.cards().map(|card| Self::expected_eliminations(card, c));
            let total_elims = elims.sorted().rev().take(3).sum::<Rational32>();
            questions.push((total_elims, c));
        }
        questions.sort_by_key(|(e, _)| *e);
        questions.pop().unwrap().1
    }

    pub fn best_card_for_question(&self, code: Code) -> Option<usize> {
        let elims = self.problem.cards().map(|card| Self::expected_eliminations(card, code));
        let (card_idx, total_elims) = elims.enumerate().max_by_key(|(_, e)| *e).unwrap();
        if total_elims == Rational32::ZERO { None } else { Some(card_idx) } 
    }

    fn expected_eliminations(card: &Card, code: Code) -> Rational32 {
        let counts = card.active_constraints().map(|c| c.accepts(code)).counts();
        let n0 = counts.get(&false).copied().unwrap_or(0) as i32;
        let n1 = counts.get(&true).copied().unwrap_or(0) as i32;
        if n0 + n1 == 0 { Rational32::ZERO } else { Rational32::new_raw(2 * n0 * n1, n0 + n1) }
    }

    pub fn ask_question(&mut self, code: Code) -> Result<(), SolverError> {
        // Start the round of questions
        self.questions.push(code);
        let mut answers = HashMap::new();

        for _ in 0..3 {
            let card_idx = if let Some(i) = self.best_card_for_question(code) { i } else { break };
            let CardAssignment { card, letter } = &mut self.problem.cards[card_idx];
            
            // If all the constraints would answer the same thing, it doesn't give us information to ask the card
            if card.active_constraints().map(|c| c.accepts(code)).all_equal() {
                continue;
            }

            // Otherwise, ask the questino to the user
            println!("Please type in the answer of card {} for the code ▲■●={}", letter, code);
            println!("{}: {}", letter, card);
            let answer = input_validation::get_bool("Answer [y/n] > ");
            println!();
            answers.insert(card_idx, answer);

            // From the answer, eliminate the constraints that didn't agree
            for cons in card.active_constraints_mut() {
                if cons.accepts(code) != answer {
                    cons.disable();
                }
            }
            
            // If the card becomes invalid, return an error
            if card.invalid() {
                self.answers.push(answers);
                return Err(SolverError::InvalidCard(self.problem.cards[card_idx].clone()))
            }
            
            // If the constraint becomes known, select it
            if let Some(cons) = card.known_constraint_mut() {
                cons.select();
            }
        }

        self.answers.push(answers);
        Ok(())
    }

    pub fn solve(&mut self) -> Result<Code, SolverError> {
        let mut round = 1;
        self.disable_insufficient_constraints();

        loop {
            println!("--- Round {} ---", round);
            self.print_state();

            let c = self.best_question();
            self.ask_question(c)?;

            if let Some(sol) = self.has_solution()? {
                return Ok(sol);
            }

            round += 1;
        }
    }
}
