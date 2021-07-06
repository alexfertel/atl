use super::*;
use itertools::{iproduct, Itertools};

fn setup_nfa() -> Nfa {
    let states: HashSet<_> = [State(1), State(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State(1);
    let accepting_states: HashSet<_> = [State(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain =
        iproduct!(states_domain, "ab".chars().map(Symbol::Identifier)).sorted_by_key(|x| x.0 .0);
    let image = [State(1), State(2), State(2), State(2)];
    let transition_function: HashMap<_, _> = domain
        .zip(image.iter().map(|&st| {
            let set: HashSet<State> = [st].iter().cloned().collect();
            set
        }))
        .collect();

    Nfa {
        states,
        alphabet,
        start,
        transition_function,
        accepting_states,
    }
}

#[test]
fn test_nfa_eq() {
    let states: HashSet<_> = [State(1), State(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State(1);
    let accepting_states: HashSet<_> = [State(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain =
        iproduct!(states_domain, "ab".chars().map(Symbol::Identifier)).sorted_by_key(|x| x.0 .0);
    let image = [State(1), State(2), State(2), State(2)];
    let transition_function: HashMap<_, _> = domain
        .zip(image.iter().map(|&st| {
            let set: HashSet<State> = [st].iter().cloned().collect();
            set
        }))
        .collect();

    let nfa = Nfa {
        states: states.clone(),
        alphabet: alphabet.clone(),
        start,
        transition_function: transition_function.clone(),
        accepting_states: accepting_states.clone(),
    };

    assert_eq!(
        nfa,
        Nfa {
            states,
            alphabet,
            start,
            accepting_states,
            transition_function
        }
    );
}

#[test]
fn test_two_transition_same_symbol() {
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

    let nfa = Nfa {
        states: states.clone(),
        alphabet: alphabet.clone(),
        start,
        transition_function: transition_function.clone(),
        accepting_states: accepting_states.clone(),
    };

    assert_eq!(nfa.recognizes("bababa"), true);
    assert_eq!(nfa.recognizes(""), false);
    assert_eq!(nfa.recognizes("ababa"), true);
    assert_eq!(nfa.recognizes("a"), true);
    assert_eq!(nfa.recognizes("b"), true);
}

#[test]
fn test_epsilon_transitions() {
    let states: HashSet<_> = [State(1), State(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State(1);
    let accepting_states: HashSet<_> = [State(2)].iter().cloned().collect();

    let mut transition_function: HashMap<(State, Symbol), HashSet<State>> = HashMap::new();

    transition_function.insert(
        (State(1), Symbol::Epsilon),
        [State(2)].iter().cloned().collect(),
    );
    transition_function.insert(
        (State(1), Symbol::Identifier('a')),
        [State(1)].iter().cloned().collect(),
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

    let nfa = Nfa {
        states: states.clone(),
        alphabet,
        start,
        transition_function,
        accepting_states,
    };

    assert_eq!(nfa.recognizes("bababa"), true);
    assert_eq!(nfa.recognizes(""), false);
    assert_eq!(nfa.recognizes("ababa"), true);
    assert_eq!(nfa.recognizes("a"), true);
    assert_eq!(nfa.recognizes("b"), true);
}

#[test]
fn test_recognizes_as_dfa() {
    let nfa = setup_nfa();

    assert_eq!(nfa.recognizes("bababa"), true);
    assert_eq!(nfa.recognizes(""), false);
    assert_eq!(nfa.recognizes("ababa"), true);
    assert_eq!(nfa.recognizes("a"), false);
    assert_eq!(nfa.recognizes("b"), true);
}

#[test]
fn test_add_transition_as_dfa() {
    println!("Sup");
    let mut nfa = setup_nfa();
    let mut set = HashSet::new();
    set.insert(State(2));
    nfa.add_transition(State(1), Symbol::Identifier('a'), set);

    assert_eq!(nfa.recognizes("bababa"), true);
    assert_eq!(nfa.recognizes(""), false);
    assert_eq!(nfa.recognizes("ababa"), true);
    println!("ababa");
    assert_eq!(nfa.recognizes("a"), true);
    assert_eq!(nfa.recognizes("b"), true);
}
