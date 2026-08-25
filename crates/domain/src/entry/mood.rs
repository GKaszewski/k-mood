use crate::errors::DomainError;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Mood {
    Awful = 1,
    Bad = 2,
    Meh = 3,
    Good = 4,
    Rad = 5,
}

impl Mood {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl TryFrom<u8> for Mood {
    type Error = DomainError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Awful),
            2 => Ok(Self::Bad),
            3 => Ok(Self::Meh),
            4 => Ok(Self::Good),
            5 => Ok(Self::Rad),
            _ => Err(DomainError::InvalidInput(format!(
                "mood value must be between 1 and 5, got {value}"
            ))),
        }
    }
}
