#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Significance {
    LikelyReal,
    Noise,
    NotEnoughData,
    NotTested,
}
