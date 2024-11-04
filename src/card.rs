use std::{fmt::Display, ops::BitOr};
use crate::constraint::Constraint;

pub struct Card { pub constraints: Vec<Constraint> }

impl Card {
    pub fn new(constraints: Vec<Constraint>) -> Card {
        Card { constraints }
    }

    pub fn empty() -> Card {
        Card { constraints: Vec::new() }
    }

    pub fn active_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter().filter(|c| !c.is_disabled())
    }

    pub fn known_constraint(&self) -> Option<&Constraint> {
        let mut active = self.active_constraints();
        if let Some(c) = active.next() {
            if active.next().is_some() {
                None
            } else {
                Some(c)
            }
        } else {
            None
        }
    }

    pub fn disable_constraint(&mut self, idx: usize) {
        self.constraints[idx].disable();
    }

    pub fn invalid(&self) -> bool {
        self.constraints.iter().all(|c| c.is_disabled())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.constraints.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join(", "))
    }
}

// Constraint | Constraint -> Card
impl BitOr for Constraint {
    type Output = Card;

    fn bitor(self, rhs: Constraint) -> Self::Output {
        Card { constraints: vec![self, rhs] }
    }
}

// Card | Constraint -> Card
impl BitOr<Constraint> for Card {
    type Output = Card;

    fn bitor(mut self, rhs: Constraint) -> Self::Output {
        self.constraints.push(rhs);
        self
    }
}

// Card | Card -> Card
impl BitOr for Card {
    type Output = Card;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.constraints.extend(rhs.constraints.into_iter());
        self
    }
}
