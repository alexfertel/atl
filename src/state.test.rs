use super::*;

#[test]
fn test_create_state() {
    let state = State::new(1);

    assert_eq!(state, State { id: 1 });
}
