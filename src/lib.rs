use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct State {
    id: i32,
}

impl State {
    fn new(id: i32) -> State {
        State { id }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DFA {
    states: HashSet<State>,
    alphabet: HashSet<char>,
    start: State,
    transition_function: HashMap<(State, char), State>,
    accepting_states: HashSet<State>,
}

impl DFA {
    fn new(
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_state() {
        let state = State::new(1);

        assert_eq!(state, State { id: 1 });
    }

    #[test]
    fn test_create_dfa() {
        let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

        let alphabet: HashSet<_> = ['a', 'b'].iter().cloned().collect();
        let start = State::new(1);
        let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

        let states_domain = states.iter().cloned();
        let domain = states_domain.zip(alphabet.iter().cloned());
        let image = [State::new(1), State::new(2), State::new(2), State::new(2)];
        let transition_function: HashMap<_, _> = domain.zip(image.iter().cloned()).collect();

        let fa = DFA::new(
            states.clone(),
            alphabet.clone(),
            start,
            transition_function.clone(),
            accepting_states.clone(),
        );

        assert_eq!(
            fa,
            DFA {
                states,
                alphabet,
                start,
                accepting_states,
                transition_function
            }
        );
    }
}
