use crate::{state::State, symbol::Symbol};
use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, Arc, Mutex};
use threadpool::ThreadPool;

mod tests;

pub const WORKER_COUNT: usize = 8;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nfa {
    pub states: HashSet<State>,
    pub alphabet: HashSet<Symbol>,
    pub start: State,
    pub transition_function: HashMap<(State, Symbol), HashSet<State>>,
    pub accepting_states: HashSet<State>,
}

impl Nfa {
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
            .unwrap_or_else(|| panic!("No transition found for ({:?}, {:?})", state, ch));

        match self.transition_function.get(&(state, Symbol::Epsilon)) {
            Some(set) => symbol_states.union(set).cloned().collect(),
            None => symbol_states.clone(),
        }
    }

    fn recognize_in_parallel(
        &self,
        word: &str,
        mut state: State,
        tx: mpsc::Sender<bool>,
        pool: Arc<Mutex<ThreadPool>>,
    ) {
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
                    let word = word.to_string();
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
        let word = word.to_string();
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
