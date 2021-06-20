mod nfa;
mod state;
mod symbol;

use itertools::{iproduct, Itertools};
use nfa::NFA;
use state::State;
use std::collections::{HashMap, HashSet};
use symbol::Symbol;

fn setup_nfa() -> NFA {
    let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State::new(1);
    let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain =
        iproduct!(states_domain, "ab".chars().map(Symbol::Identifier)).sorted_by_key(|x| x.0.id);
    let image = [State::new(1), State::new(2), State::new(2), State::new(2)];
    let transition_function: HashMap<_, _> = domain
        .zip(image.iter().map(|&st| {
            let set: HashSet<State> = [st].iter().cloned().collect();
            set.clone()
        }))
        .collect();

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
    println!("After setting up the nfa");
    let result = nfa.recognizes("bababa");
    print!("Did recognize? {}", result);
}
