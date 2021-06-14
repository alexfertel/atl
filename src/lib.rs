use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct State {
    id: i32,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Symbol {
    Epsilon,
    Identifier(char),
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

    fn add_transition(&mut self, source_state: State, symbol: char, destination_state: State) {
        self.transition_function
            .insert((source_state, symbol), destination_state);
    }

    fn recognizes(&self, word: &str) -> bool {
        self.accepting_states
            .contains(word.chars().fold(&self.start, |current_state, symbol| {
                self.transition_function
                    .get(&(*current_state, symbol))
                    .expect(&format!(
                        "No transition found for ({:?}, {:?})",
                        *current_state, symbol
                    ))
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_dfa() -> DFA {
        let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

        let alphabet: HashSet<_> = ['a', 'b'].iter().cloned().collect();
        let start = State::new(1);
        let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

        let states_domain = states.iter().cloned();
        let domain = states_domain.zip(alphabet.iter().cloned());
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
    fn test_create_state() {
        let state = State::new(1);

        assert_eq!(state, State { id: 1 });
    }

    #[test]
    fn test_dfa_eq() {
        let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

        let alphabet: HashSet<_> = ['a', 'b'].iter().cloned().collect();
        let start = State::new(1);
        let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

        let states_domain = states.iter().cloned();
        let domain = states_domain.zip(alphabet.iter().cloned());
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
        assert!(dfa.recognizes("ababa"));
    }
}
