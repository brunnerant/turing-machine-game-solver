use std::{fmt::Display, ptr};

use itertools::Itertools;

use crate::card::Card;

pub struct Problem { pub cards: Vec<Card> }

impl Problem {
    pub fn new(cards: Vec<Card>) -> Self {
        Problem { cards }
    }

    pub fn disable_insufficient_constraints(&mut self) {
        loop {
            let mut to_remove = Vec::new();
            for card_idx in 0..self.cards.len() {
                let card = &self.cards[card_idx];
                let mut indices = Vec::new();

                for constraint_idx in 0..card.constraints.len() {
                    let constraint = &card.constraints[constraint_idx];
                    if constraint.is_disabled() {
                        continue;
                    }
                    let other_cards = self.cards.iter().filter(|&c| !ptr::eq(c, card));
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

            // Stop the loop if there are no more insufficient constraints
            if to_remove.iter().all(|c| c.is_empty()) {
                break;
            }

            // Otherwise, disable the insufficient constraints and continue
            for (card, to_remove) in self.cards.iter_mut().zip(to_remove) {
                for i in to_remove {
                    card.constraints[i].disable();
                }
            }
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.cards.iter() {
            writeln!(f, "{}", c)?
        }
        Ok(())
    }
}
