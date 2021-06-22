use super::State;
use super::DFA;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::collections::HashSet;

fn setup_dfa() -> DFA {
    let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().collect();
    let start = State::new(1);
    let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain = iproduct!(states_domain, "ab".chars()).sorted_by_key(|x| x.0.id);
    let image = [State::new(1), State::new(2), State::new(2), State::new(2)];
    let transition_function: HashMap<_, _> = domain.zip(image.iter().cloned()).collect();

    DFA::new(
        states.clone(),
        alphabet.clone(),
        start,
        transition_function.clone(),
        accepting_states.clone(),
    )
}

#[test]
fn test_dfa_eq() {
    let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().collect();
    let start = State::new(1);
    let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain = iproduct!(states_domain, "ab".chars()).sorted_by_key(|x| x.0.id);
    let image = [State::new(1), State::new(2), State::new(2), State::new(2)];
    let transition_function: HashMap<_, _> = domain.zip(image.iter().cloned()).collect();
    let dfa = DFA::new(
        states.clone(),
        alphabet.clone(),
        start,
        transition_function.clone(),
        accepting_states.clone(),
    );

    assert_eq!(
        dfa,
        DFA {
            states,
            alphabet,
            start,
            accepting_states,
            transition_function
        }
    );
}

#[test]
fn test_recognizes() {
    let dfa = setup_dfa();
    assert_eq!(dfa.recognizes("bababa"), true);
    assert_eq!(dfa.recognizes(""), false);
    assert_eq!(dfa.recognizes("ababa"), true);
    assert_eq!(dfa.recognizes("a"), false);
    assert_eq!(dfa.recognizes("b"), true);
}

#[test]
fn test_add_transition() {
    let mut dfa = setup_dfa();
    dfa.add_transition(State::new(1), 'a', State::new(2));

    assert_eq!(dfa.recognizes("bababa"), true);
    assert_eq!(dfa.recognizes(""), false);
    assert_eq!(dfa.recognizes("ababa"), true);
    assert_eq!(dfa.recognizes("a"), true);
    assert_eq!(dfa.recognizes("b"), true);
}