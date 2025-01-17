use std::{collections::HashMap, iter::repeat};
use itertools::Itertools;
use num::Rational32;
use crate::{code::{Code, Symbol}, constraint::Constraint, problem::{Problem, ProblemMode}};

// This trait is used to inject user input into the solver to run it in testing mode
pub trait Verifier {
    fn accepts(&self, idx: usize, code: Code) -> bool;
}

// This verifier asks the user about what a card answers for a given code
pub struct CommandLineVerifier;

// This verifier automatically answers what the card would answer given the constraint that they obey
pub struct AutomaticVerifier(Vec<Constraint>);

impl Verifier for CommandLineVerifier {
    fn accepts(&self, idx: usize, code: Code) -> bool {
        let letter = "ABCDEF".chars().nth(idx).unwrap();
        println!();
        println!("Please type in the answer of verifier {} for the code ▲■●={}", letter, code);
        input_validation::get_bool("Answer [y/n] > ")
    }
}

impl Verifier for AutomaticVerifier {
    fn accepts(&self, idx: usize, code: Code) -> bool {
        self.0[idx].accepts(code)
    }
}

#[derive(PartialEq, Eq)]
pub enum SolverVerbosity {
    None, Normal, Verbose
}

pub enum SolverError {
    Impossible(Vec<usize>),
    MultipleSolutions(Vec<Code>),
}

pub struct Solver<V: Verifier> {
    verifiers: Vec<Vec<Constraint>>, // The set of constraints for every verifier
    questions: Vec<Code>, // The questions that were asked
    answers: Vec<HashMap<usize, bool>>, // The answers that were given
    verbosity: SolverVerbosity,
    verifier: V,
}

impl<V: Verifier> Solver<V> {
    pub fn new(problem: &Problem) -> Solver<CommandLineVerifier> {
        let verifiers: Vec<Vec<_>> = match problem.mode {
            ProblemMode::Normal => {
                Self::assign_groups(problem.cards.iter()
                    .map(|c| c.constraints())
                    .collect())
            },
            ProblemMode::Extreme => {
                Self::assign_groups(problem.cards.iter()
                    .chunks(2).into_iter()
                    .map(|b| b.flat_map(|c| c.constraints()).collect())
                    .collect())
            },
            ProblemMode::Nightmare => {
                let constraints = Self::assign_groups(
                        problem.cards.iter()
                        .map(|c| c.constraints())
                        .collect()
                    ).into_iter().flatten().collect();
                repeat(constraints).take(problem.cards.len()).collect()
            }
        };
        Solver {
            verifiers,
            questions: Vec::new(),
            answers: Vec::new(),
            verbosity: SolverVerbosity::Normal,
            verifier: CommandLineVerifier,
        }
    }

    // Change the verbosity of the solver
    pub fn verbosity(mut self, verbosity: SolverVerbosity) -> Self {
        self.verbosity = verbosity;
        self
    }

    // Puts the solver into automatic mode: outputs nothing and automatically answers questions. Useful for testing.
    pub fn automatic(self, verifiers: Vec<Constraint>) -> Solver<AutomaticVerifier> {
        Solver {
            verifiers: self.verifiers,
            questions: self.questions,
            answers: self.answers,
            verbosity: SolverVerbosity::None,
            verifier: AutomaticVerifier(verifiers),
        }
    }

    pub fn num_rounds(&self) -> usize {
        self.questions.len()
    }

    pub fn num_questions(&self) -> usize {
        self.answers.iter().map(|a| a.len()).sum()
    }

    fn assign_groups(mut constraints: Vec<Vec<Constraint>>) -> Vec<Vec<Constraint>> {
        constraints.iter_mut().enumerate()
            .for_each(|(i, cs)| cs.into_iter().for_each(|c| *c = c.with_group(i as u8)));
        constraints
    }

    // Prints the state of the solver
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

    fn err_if_invalid(&self) -> Result<(), SolverError> {
        let vs: Vec<_> = self.verifiers.iter().enumerate()
            .filter_map(|(i, cs)| if cs.is_empty() { Some(i) } else { None }).collect();
        if vs.is_empty() { Ok(()) } else { Err(SolverError::Impossible(vs)) }
    }

    // Checks if a solution was found or not, or returns an error if several solutions are possible
    fn has_solution(&self) -> Result<Option<Code>, SolverError> {
        let known_constraints = self.verifiers.iter()
            .map(|cs| if cs.len() == 1 { Some(cs[0]) } else { None }).collect::<Vec<_>>();

        if known_constraints.iter().all(|c| c.is_some()) {
            let cons = Constraint::inter(known_constraints.iter().flatten().copied());
            cons.solution().map_or_else(
                || Err(SolverError::MultipleSolutions(cons.solutions().collect())),
                |s| Ok(Some(s)))
        } else {
            Ok(None)
        }
    }

    // Finds the best question to ask with the elimination heuristic
    fn best_question(&self) -> Code {
        let mut questions = Vec::with_capacity(125);
        for c in Code::all() {
            let elims = self.verifiers.iter().map(|card| Self::expected_eliminations(card, c));
            let total_elims = elims.sorted().rev().take(3).sum::<Rational32>();
            questions.push((total_elims, c));
        }
        questions.sort_by_key(|(e, _)| *e);
        let (e, c) = questions.pop().unwrap();
        if self.verbosity == SolverVerbosity::Verbose {
            println!("Expected number of eliminations from the question: {:.1}", *e.numer() as f32 / *e.denom() as f32)
        }
        c
    }

    // Checks if the given set of constraints are valid or not
    fn valid_constraints(constraints: &Vec<Constraint>) -> bool {
        // The constraints must all come from different cards (for nightmare mode only)
        if !constraints.iter().map(|c| c.group()).all_unique() {
            return false;
        }

        // The constraints must uniquely define the solution
        if !Constraint::inter(constraints.iter().copied()).has_unique_solution() {
            return false;
        }

        // No constraint should be redundant
        (0..constraints.len()).all(|i| {
            let others = constraints.iter().enumerate().filter_map(|(j, c)| if j == i { None } else { Some(c) }).copied();
            !constraints[i].is_superset_of(&Constraint::inter(others))
        })
    }

    // Eliminates impossible constraints, that is, constraints for which no combination
    // of the other constraints gives valid solutions
    fn eliminate(&mut self) -> Result<(), SolverError> {
        let mut num_elims = 0;
        loop {
            let n = self.eliminate_step();
            num_elims += n;
            if n == 0 { break }
        }
        if self.verbosity == SolverVerbosity::Verbose {
            println!("Number of eliminations from deductions: {}", num_elims);
        }
        self.err_if_invalid()
    }

    fn eliminate_step(&mut self) -> usize {
        let mut impossible: Vec<_> = self.verifiers.iter().map(|cs| vec![true; cs.len()]).collect();

        for idx in self.verifiers.iter().map(|cs| 0..cs.len()).multi_cartesian_product() {
            let constraints = self.verifiers.iter().zip(idx.iter().copied()).map(|(cs, i)| cs[i]).collect();
            if Self::valid_constraints(&constraints) {
                impossible.iter_mut().zip(idx.iter().copied()).for_each(|(imp, i)| imp[i] = false)
            }
        }

        let mut num_elim = 0;
        for (cs, imp) in self.verifiers.iter_mut().zip(impossible) {
            let mut i = 0;
            cs.retain(|_| { let res = !imp[i]; i += 1; res });
            num_elim += imp.into_iter().filter(|&i| i).count();
        }
        num_elim
    }

    fn best_verifier_for_question(&self, code: Code) -> Option<usize> {
        let elims = self.verifiers.iter().map(|v| Self::expected_eliminations(v, code));
        let (v_idx, e) = elims.enumerate().max_by_key(|(_, e)| *e).unwrap();
        if self.verbosity == SolverVerbosity::Verbose {
            if e == Rational32::ZERO {
                println!("No more information from question.")
            } else {
                println!("Expected number of eliminations from the answer: {:.1}", *e.numer() as f32 / *e.denom() as f32);
            }
        }
        if e == Rational32::ZERO { None } else { Some(v_idx) } 
    }

    fn expected_eliminations(verifier: &Vec<Constraint>, code: Code) -> Rational32 {
        let counts = verifier.iter().map(|c| c.accepts(code)).counts();
        let n0 = counts.get(&false).copied().unwrap_or(0) as i32;
        let n1 = counts.get(&true).copied().unwrap_or(0) as i32;
        if n0 + n1 == 0 { Rational32::ZERO } else { Rational32::new_raw(2 * n0 * n1, n0 + n1) }
    }

    pub fn question(&mut self, code: Code, verifier: usize) -> Result<bool, SolverError> {
        // Retrieve the card that we are querying and ask the user for the result
        let constraints = &mut self.verifiers[verifier];
        let answer = self.verifier.accepts(verifier, code);
        
        // From the answer, eliminate the constraints that didn't agree
        let len_before = constraints.len();
        constraints.retain(|c| c.accepts(code) == answer);
        if self.verbosity == SolverVerbosity::Verbose {
            println!("Number of eliminations from answer: {}", len_before - constraints.len());
        }

        // Eliminate constraints
        self.eliminate()?;
        Ok(answer)
    }

    pub fn round(&mut self, code: Code) -> Result<(), SolverError> {
        let mut answers = HashMap::new();
        for _ in 0..3 {
            let v_idx = if let Some(i) = self.best_verifier_for_question(code) { i } else { break };
            let answer = self.question(code, v_idx)?;
            answers.insert(v_idx, answer);
        }

        if !answers.is_empty() {
            self.questions.push(code);
            self.answers.push(answers);
        }
        Ok(())
    }

    pub fn solve(&mut self) -> Result<Code, SolverError> {
        let mut round = 1;
        self.eliminate()?;
        
        loop {
            if self.verbosity != SolverVerbosity::None {
                println!();
                println!("━ Round {} ━━━", round);
                self.print_state();
            }

            let c = self.best_question();
            self.round(c)?;

            if let Some(sol) = self.has_solution()? {
                return Ok(sol);
            }

            round += 1;
        }
    }
}
