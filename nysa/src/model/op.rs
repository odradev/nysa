#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Op {
    Less,
    LessEq,
    More,
    MoreEq,
    Eq,
    NotEq,
}
