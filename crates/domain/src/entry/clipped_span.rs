use super::DateSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClippedSpan {
    span: DateSpan,
    clipped: bool,
}

impl ClippedSpan {
    pub(super) fn new(span: DateSpan, clipped: bool) -> Self {
        Self { span, clipped }
    }

    pub fn span(&self) -> &DateSpan {
        &self.span
    }

    pub fn was_clipped(&self) -> bool {
        self.clipped
    }
}
