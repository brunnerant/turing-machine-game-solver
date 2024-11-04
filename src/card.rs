use std::{fmt::Display, ops::BitOr};
use crate::constraint::Constraint;
use itertools::Itertools;

#[derive(Clone)]
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

    pub fn active_constraints_mut(&mut self) -> impl Iterator<Item = &mut Constraint> {
        self.constraints.iter_mut().filter(|c| !c.is_disabled())
    }

    fn single_elem<T>(it: impl Iterator<Item = T>) -> Option<T> {
        if let Some((elem,)) = it.collect_tuple() {
            Some(elem)
        } else {
            None
        }
    }

    pub fn known_constraint(&self) -> Option<&Constraint> {
        Self::single_elem(self.active_constraints())
    }

    pub fn known_constraint_mut(&mut self) -> Option<&mut Constraint> {
        Self::single_elem(self.active_constraints_mut())
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
        if f.sign_plus() {
            write!(f, "[{}]", self.constraints.iter().map(|c| format!("{:+}", c)).collect::<Vec<_>>().join(", "))
        } else {
            write!(f, "[{}]", self.constraints.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join(", "))
        }
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
