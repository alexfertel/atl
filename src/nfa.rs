use crate::{state::State, symbol::Symbol};
use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, Arc, Mutex};
use threadpool::ThreadPool;

#[cfg(test)]
#[path = "./nfa.test.rs"]
mod tests;

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

    fn step(&self, ch: char, state: State) -> HashSet<State> {
        let symbol_states = self
            .transition_function
            .get(&(state, Symbol::Identifier(ch)))
            .expect(&format!("No transition found for ({:?}, {:?})", state, ch));

        match self.transition_function.get(&(state, Symbol::Epsilon)) {
            Some(set) => symbol_states.union(set).cloned().collect(),
            None => symbol_states.to_owned(),
        }
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

            if next_states.len() == 1 {
                state = *next_states.iter().next().unwrap();
            } else {
                let mut next_states = next_states.iter();
                state = *next_states.next().unwrap();

                for &state in next_states {
                    let tx = tx.clone();
                    let child_nfa = self.clone();
                    let word = word.to_owned();
                    let pool_clone = Arc::clone(&pool);

                    pool.lock().unwrap().execute(move || {
                        child_nfa.recognize_in_parallel(&word[1..], state, tx, pool_clone);
                    })
                }
            }
        }

        tx.send(self.accepting_states.contains(&state)).unwrap();
    }

    pub fn recognizes(&self, word: &str) -> bool {
        let pool = threadpool::ThreadPool::new(WORKER_COUNT);
        let pool = Arc::new(Mutex::new(pool));

        let (tx, rx) = mpsc::channel();

        let root_nfa = self.clone();
        let word = word.to_owned();
        let pool_clone = Arc::clone(&pool);
        pool.lock().unwrap().execute(move || {
            root_nfa.recognize_in_parallel(&word[..], root_nfa.start, tx, pool_clone);
        });

        let did_recognize = rx.iter().any(|did_recognize| did_recognize);

        // If we don't join here, `rx` would be droped
        // and a thread might try to send to a closed channel.
        pool.lock().unwrap().join();

        did_recognize
    }
}
