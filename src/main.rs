use {
    atl::{nfa::Nfa, state::State, symbol::Symbol},
    std::collections::{HashMap, HashSet},
};

fn setup_nfa() -> Nfa {
    let states: HashSet<_> = [State(1), State(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State(1);
    let accepting_states: HashSet<_> = [State(2)].iter().cloned().collect();

    let mut transition_function: HashMap<(State, Symbol), HashSet<State>> = HashMap::new();

    transition_function.insert(
        (State(1), Symbol::Identifier('a')),
        [State(1), State(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State(1), Symbol::Identifier('b')),
        [State(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State(2), Symbol::Identifier('a')),
        [State(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State(2), Symbol::Identifier('b')),
        [State(2)].iter().cloned().collect(),
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
    println!("Result: {}", nfa.recognizes("a"));
}
