#[derive(Debug, Clone, PartialEq)]
pub struct Ranked<T> {
    pub item: T,
    pub count: usize,
    pub share: f64,
}
