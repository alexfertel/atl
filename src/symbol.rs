#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum Symbol {
    Epsilon,
    Identifier(char),
}
