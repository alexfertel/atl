mod nfa;
mod state;
mod symbol;

use nfa::NFA;
use state::State;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
use symbol::Symbol;

fn setup_nfa() -> NFA {
    let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State::new(1);
    let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

    let mut transition_function: HashMap<(State, Symbol), HashSet<State>> = HashMap::new();

    transition_function.insert(
        (State::new(1), Symbol::Identifier('a')),
        HashSet::from_iter([State::new(1), State::new(2)].iter().cloned()),
    );
    transition_function.insert(
        (State::new(1), Symbol::Identifier('b')),
        HashSet::from_iter([State::new(2)].iter().cloned()),
    );
    transition_function.insert(
        (State::new(2), Symbol::Identifier('a')),
        HashSet::from_iter([State::new(2)].iter().cloned()),
    );
    transition_function.insert(
        (State::new(2), Symbol::Identifier('b')),
        HashSet::from_iter([State::new(2)].iter().cloned()),
    );

    NFA::new(
        states.clone(),
        alphabet.clone(),
        start,
        transition_function.clone(),
        accepting_states.clone(),
    )
}

fn main() {
    let nfa = setup_nfa();
    let result = nfa.recognizes("a");
    print!("Did recognize? {}", result);
}
