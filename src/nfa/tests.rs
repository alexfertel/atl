use super::*;
use itertools::{iproduct, Itertools};

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

#[test]
fn test_nfa_eq() {
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

    let nfa = NFA::new(
        states.clone(),
        alphabet.clone(),
        start,
        transition_function.clone(),
        accepting_states.clone(),
    );

    assert_eq!(
        nfa,
        NFA {
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

    let nfa = NFA::new(
        states.clone(),
        alphabet.clone(),
        start,
        transition_function.clone(),
        accepting_states.clone(),
    );

    assert_eq!(nfa.recognizes("bababa"), true);
    assert_eq!(nfa.recognizes(""), false);
    assert_eq!(nfa.recognizes("ababa"), true);
    assert_eq!(nfa.recognizes("a"), true);
    assert_eq!(nfa.recognizes("b"), true);
}

#[test]
fn test_epsilon_transitions() {
    let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
    let start = State::new(1);
    let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

    let mut transition_function: HashMap<(State, Symbol), HashSet<State>> = HashMap::new();

    transition_function.insert(
        (State::new(1), Symbol::Epsilon),
        HashSet::from_iter([State::new(2)].iter().cloned()),
    );
    transition_function.insert(
        (State::new(1), Symbol::Identifier('a')),
        HashSet::from_iter([State::new(1)].iter().cloned()),
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

    let nfa = NFA::new(
        states.clone(),
        alphabet.clone(),
        start,
        transition_function.clone(),
        accepting_states.clone(),
    );

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
    let mut nfa = setup_nfa();
    let mut set = HashSet::new();
    set.insert(State::new(2));
    nfa.add_transition(State::new(1), Symbol::Identifier('a'), set);

    assert_eq!(nfa.recognizes("bababa"), true);
    assert_eq!(nfa.recognizes(""), false);
    assert_eq!(nfa.recognizes("ababa"), true);
    assert_eq!(nfa.recognizes("a"), true);
    assert_eq!(nfa.recognizes("b"), true);
}
