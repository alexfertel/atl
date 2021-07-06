use crate::{state::State, symbol::Symbol};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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
        recognized: Arc<AtomicBool>,
        pool: ThreadPool,
    ) {
        for ch in word.chars() {
            if recognized.load(Ordering::SeqCst) {
                return;
            }

            let next_states = self.step(ch, state);

            if next_states.len() == 1 {
                state = *next_states.iter().next().unwrap();
            } else {
                let mut next_states = next_states.iter();
                state = *next_states.next().unwrap();

                for &state in next_states {
                    let child_nfa = self.clone();
                    let word = word.to_string();
                    let recognized_clone = Arc::clone(&recognized);
                    let pool_clone = pool.clone();

                    pool.execute(move || {
                        child_nfa.recognize_in_parallel(
                            &word[1..],
                            state,
                            recognized_clone,
                            pool_clone,
                        )
                    });
                }
            }
        }

        if self.accepting_states.contains(&state) {
            recognized.store(true, Ordering::SeqCst);
        }
    }

    pub fn recognizes(&self, word: &str) -> bool {
        let pool = ThreadPool::new(WORKER_COUNT);

        let recognized = Arc::new(AtomicBool::new(false));

        let root_nfa = self.clone();
        let word = word.to_string();
        let recognized_clone = Arc::clone(&recognized);
        let pool_clone = pool.clone();
        pool.execute(move || {
            root_nfa.recognize_in_parallel(&word[..], root_nfa.start, recognized_clone, pool_clone);
        });

        pool.join();

        recognized.load(Ordering::SeqCst)
    }
}
