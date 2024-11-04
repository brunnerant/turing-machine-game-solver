use std::fmt::Display;

use crate::card::*;

pub struct CardAssignment {
    pub letter: char,
    pub card: Card,
}

impl Display for CardAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            write!(f, "{}: {:+}", self.letter, self.card)
        } else {
            write!(f, "{}: {}", self.letter, self.card)
        }
    }
}

pub struct Problem { pub cards: Vec<CardAssignment> }

impl Problem {
    pub fn new(cards: Vec<Card>) -> Self {
        Problem { cards: "ABCDEF".chars().zip(cards).map(|(letter, card)| CardAssignment { letter, card }).collect() }
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter().map(|c| &c.card)
    }

    pub fn cards_mut(&mut self) -> impl Iterator<Item = &mut Card> {
        self.cards.iter_mut().map(|c| &mut c.card)
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            for c in self.cards.iter() {
                writeln!(f, "{:+}", c)?
            }
        } else {
            for c in self.cards.iter() {
                writeln!(f, "{}", c)?
            }
        }
        Ok(())
    }
}
