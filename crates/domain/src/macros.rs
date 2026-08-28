macro_rules! uuid_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
            serde::Serialize, serde::Deserialize,
        )]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn from_uuid(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }

            pub fn value(&self) -> uuid::Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::generate()
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

pub(crate) use uuid_id;

macro_rules! bounded_metric {
    ($name:ident, $inner:ty, $min:expr, $max:expr, $label:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($inner);

        impl $name {
            pub const MINIMUM: $inner = $min;
            pub const MAXIMUM: $inner = $max;

            pub fn new(value: $inner) -> Result<Self, crate::errors::DomainError> {
                if !(Self::MINIMUM..=Self::MAXIMUM).contains(&value) {
                    return Err(crate::errors::DomainError::InvalidInput(format!(
                        concat!($label, " must be between {} and {}, got {}"),
                        Self::MINIMUM,
                        Self::MAXIMUM,
                        value
                    )));
                }

                Ok(Self(value))
            }

            pub fn from_persistence(value: $inner) -> Self {
                Self(value)
            }

            pub fn value(&self) -> $inner {
                self.0
            }
        }
    };
}

pub(crate) use bounded_metric;
