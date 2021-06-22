use crate::state::State;
use std::collections::{HashMap, HashSet};

mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct DFA {
    states: HashSet<State>,
    alphabet: HashSet<char>,
    start: State,
    transition_function: HashMap<(State, char), State>,
    accepting_states: HashSet<State>,
}

impl DFA {
    pub fn new(
        states: HashSet<State>,
        alphabet: HashSet<char>,
        start: State,
        transition_function: HashMap<(State, char), State>,
        accepting_states: HashSet<State>,
    ) -> DFA {
        DFA {
            states,
            alphabet,
            start,
            transition_function,
            accepting_states,
        }
    }

    pub fn add_transition(&mut self, source_state: State, symbol: char, destination_state: State) {
        self.transition_function
            .insert((source_state, symbol), destination_state);
    }

    pub fn recognizes(&self, word: &str) -> bool {
        self.accepting_states
            .contains(word.chars().fold(&self.start, |current_state, symbol| {
                self.transition_function
                    .get(&(*current_state, symbol))
                    .expect(&format!(
                        "No transition found for ({:?}, {:?})",
                        *current_state, symbol
                    ))
            }))
    }
}
