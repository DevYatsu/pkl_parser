use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Indexing {
    indexed: Box<Expr>,
    indexor: Box<Expr>,
    with: IndexingOperator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndexingOperator {
    Dot,
    Bracket,
    NullPropagation,
}
