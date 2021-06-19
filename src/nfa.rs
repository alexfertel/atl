use crate::{state::State, symbol::Symbol};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    states: HashSet<State>,
    alphabet: HashSet<Symbol>,
    start: State,
    transition_function: HashMap<(State, Symbol), State>,
    accepting_states: HashSet<State>,
}

impl NFA {
    fn new(
        states: HashSet<State>,
        alphabet: HashSet<Symbol>,
        start: State,
        transition_function: HashMap<(State, Symbol), State>,
        accepting_states: HashSet<State>,
    ) -> NFA {
        NFA {
            states,
            alphabet,
            start,
            transition_function,
            accepting_states,
        }
    }

    fn add_transition(&mut self, source_state: State, symbol: Symbol, destination_state: State) {
        self.transition_function
            .insert((source_state, symbol), destination_state);
    }

    fn recognizes(&self, word: &str) -> bool {}
}
