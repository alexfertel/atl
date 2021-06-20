use crate::{state::State, symbol::Symbol};
use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, Arc, Mutex};
use threadpool::ThreadPool;

const WORKER_COUNT: usize = 1024;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NFA {
    states: HashSet<State>,
    alphabet: HashSet<Symbol>,
    start: State,
    transition_function: HashMap<(State, Symbol), HashSet<State>>,
    accepting_states: HashSet<State>,
}

impl NFA {
    pub fn new(
        states: HashSet<State>,
        alphabet: HashSet<Symbol>,
        start: State,
        transition_function: HashMap<(State, Symbol), HashSet<State>>,
        accepting_states: HashSet<State>,
    ) -> NFA {
        NFA {
            states,
            alphabet,
            start,
            transition_function,
            accepting_states,
        }
    }

    pub fn add_transition(
        &mut self,
        source_state: State,
        symbol: Symbol,
        destination_states: HashSet<State>,
    ) {
        self.transition_function
            .insert((source_state, symbol), destination_states);
    }

    fn step(&self, ch: char, state: State) -> &HashSet<State> {
        self.transition_function
            .get(&(state, Symbol::Identifier(ch)))
            .expect(&format!("No transition found for ({:?}, {:?})", state, ch))
    }

    fn recognize_in_parallel<'a>(
        &self,
        word: &str,
        state: State,
        tx: mpsc::Sender<bool>,
        pool: Arc<Mutex<ThreadPool>>,
    ) {
        let mut state = state;
        for ch in word.chars() {
            let next_states = self.step(ch, state);
            if next_states.len() > 1 {
                let mut next_states = next_states.iter();
                state = *next_states.next().unwrap();
                for &state in next_states {
                    let tx = tx.clone();
                    let child_automata = self.clone();
                    let word = word.to_owned();
                    let local_handle = Arc::clone(&pool);
                    pool.lock().unwrap().execute(move || {
                        child_automata.recognize_in_parallel(&word[1..], state, tx, local_handle);
                    })
                }
            } else {
                state = *next_states.iter().next().unwrap();
            }
        }

        tx.send(self.accepting_states.contains(&state)).unwrap();
    }

    pub fn recognizes(&self, word: &str) -> bool {
        let pool = threadpool::ThreadPool::new(WORKER_COUNT);
        let pool = Arc::new(Mutex::new(pool));

        let (tx, rx) = mpsc::channel();

        let child_automata = self.clone();
        let word = word.to_owned();
        let thread_handle = Arc::clone(&pool);
        let pool = pool.lock().unwrap();
        pool.execute(move || {
            child_automata.recognize_in_parallel(
                &word[..],
                child_automata.start,
                tx,
                thread_handle,
            );
        });

        loop {
            match rx.recv() {
                Ok(true) => break true,
                Ok(false) => continue,
                Err(_) => break false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{iproduct, Itertools};

    fn setup_nfa() -> NFA {
        let states: HashSet<_> = [State::new(1), State::new(2)].iter().cloned().collect();

        let alphabet: HashSet<_> = "ab".chars().map(Symbol::Identifier).collect();
        let start = State::new(1);
        let accepting_states: HashSet<_> = [State::new(2)].iter().cloned().collect();

        let states_domain = states.iter().cloned();
        let domain = iproduct!(states_domain, "ab".chars().map(Symbol::Identifier))
            .sorted_by_key(|x| x.0.id);
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
        let domain = iproduct!(states_domain, "ab".chars().map(Symbol::Identifier))
            .sorted_by_key(|x| x.0.id);
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
    fn test_recognizes() {
        let nfa = setup_nfa();

        assert_eq!(nfa.recognizes("bababa"), true);
        assert_eq!(nfa.recognizes(""), false);
        assert_eq!(nfa.recognizes("ababa"), true);
        assert_eq!(nfa.recognizes("a"), false);
        assert_eq!(nfa.recognizes("b"), true);
    }

    #[test]
    fn test_add_transition() {
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
}
