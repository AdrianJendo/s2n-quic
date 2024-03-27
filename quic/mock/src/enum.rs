pub enum WithVal<T> {
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
    Eq(T),
}