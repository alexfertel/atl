#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub struct State {
    pub id: i32,
}

impl State {
    pub fn new(id: i32) -> State {
        State { id }
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
}
