use {
    atl::{nfa::Nfa, state::State, symbol::Symbol},
    std::collections::{HashMap, HashSet},
};

fn setup_nfa() -> Nfa {
    let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State::new(1);
    let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

    let mut transition_function: HashMap<(State, Symbol), HashSet<State>> = HashMap::new();

    transition_function.insert(
        (State::new(1), Symbol::Identifier('a')),
        [State::new(1), State::new(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State::new(1), Symbol::Identifier('b')),
        [State::new(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State::new(2), Symbol::Identifier('a')),
        [State::new(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State::new(2), Symbol::Identifier('b')),
        [State::new(2)].iter().cloned().collect(),
    );

    Nfa {
        states,
        alphabet,
        start,
        transition_function,
        accepting_states,
    }
}

fn main() {
    let nfa = setup_nfa();
    let result = nfa.recognizes("a");
    print!("Did recognize? {}", result);
}
