use crate::state::State;
use std::collections::{HashMap, HashSet};

pub mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct Dfa {
    pub states: HashSet<State>,
    pub alphabet: HashSet<char>,
    pub start: State,
    pub transition_function: HashMap<(State, char), State>,
    pub accepting_states: HashSet<State>,
}

impl Dfa {
    pub fn add_transition(&mut self, source_state: State, symbol: char, destination_state: State) {
        self.transition_function
            .insert((source_state, symbol), destination_state);
    }

    pub fn recognizes(&self, word: &str) -> bool {
        self.accepting_states
            .contains(word.chars().fold(&self.start, |current_state, symbol| {
                self.transition_function
                    .get(&(*current_state, symbol))
                    .unwrap_or_else(|| {
                        panic!(
                            "No transition found for ({:?}, {:?})",
                            *current_state, symbol
                        )
                    })
            }))
    }
}
