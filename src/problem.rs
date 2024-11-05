use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{cards::card_from_id, constraint::Constraint};

#[derive(Clone)]
pub struct Card { pub constraints: HashMap<String, Constraint> }

impl Card {
    pub fn new(cons: Vec<(String, Constraint)>) -> Card {
        Card { constraints: HashMap::from_iter(cons.into_iter()) }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.constraints.keys().join(", "))
    }
}

pub enum ProblemMode {
    Normal, Extreme, Nightmare
}

pub struct Problem { pub cards: Vec<Card>, pub mode: ProblemMode }

impl Problem {
    pub fn from_cards(mode: ProblemMode, cards: Vec<Card>) -> Problem {
        Problem { cards, mode }
    }

    pub fn from_card_ids(mode: ProblemMode, ids: Vec<usize>) -> Problem {
        Problem { cards: ids.into_iter().map(card_from_id).collect(), mode }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mode {
            ProblemMode::Normal =>
                for (card, letter) in self.cards.iter().zip("ABCDEF".chars()) {
                    writeln!(f, "{}: {}", letter, card)?
                }
            ProblemMode::Extreme =>
                for (mut cards, letter) in self.cards.iter().chunks(2).into_iter().zip("ABCDEF".chars()) {
                    writeln!(f, "{}: {}", letter, cards.join(" "))?
                }
            ProblemMode::Nightmare =>
                for (card, letter) in self.cards.iter().zip("ABCDEF".chars()) {
                    writeln!(f, "{}?: {}", letter, card)?
                }
        }
        Ok(())
    }
}
