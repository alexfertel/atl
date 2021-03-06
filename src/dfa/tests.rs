use super::Dfa;
use super::State;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::collections::HashSet;

fn setup_dfa() -> Dfa {
    let states: HashSet<_> = [State(1), State(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().collect();
    let start = State(1);
    let accepting_states: HashSet<_> = [State(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain = iproduct!(states_domain, "ab".chars()).sorted_by_key(|x| x.0 .0);
    let image = [State(1), State(2), State(2), State(2)];
    let transition_function: HashMap<_, _> = domain.zip(image.iter().cloned()).collect();

    Dfa {
        states,
        alphabet,
        start,
        transition_function,
        accepting_states,
    }
}

#[test]
fn test_dfa_eq() {
    let states: HashSet<_> = [State(1), State(2)].iter().cloned().collect();

    let alphabet: HashSet<_> = "ab".chars().collect();
    let start = State(1);
    let accepting_states: HashSet<_> = [State(2)].iter().cloned().collect();

    let states_domain = states.iter().cloned();
    let domain = iproduct!(states_domain, "ab".chars()).sorted_by_key(|x| x.0 .0);
    let image = [State(1), State(2), State(2), State(2)];
    let transition_function: HashMap<_, _> = domain.zip(image.iter().cloned()).collect();
    let dfa = Dfa {
        states: states.clone(),
        alphabet: alphabet.clone(),
        start,
        transition_function: transition_function.clone(),
        accepting_states: accepting_states.clone(),
    };

    assert_eq!(
        dfa,
        Dfa {
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
    dfa.add_transition(State(1), 'a', State(2));

    assert_eq!(dfa.recognizes("bababa"), true);
    assert_eq!(dfa.recognizes(""), false);
    assert_eq!(dfa.recognizes("ababa"), true);
    assert_eq!(dfa.recognizes("a"), true);
    assert_eq!(dfa.recognizes("b"), true);
}
