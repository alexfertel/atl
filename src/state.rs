#[cfg(test)]
#[path = "./state.test.rs"]
mod tests;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub struct State {
    pub id: i32,
}

impl State {
    pub fn new(id: i32) -> State {
        State { id }
    }
}
