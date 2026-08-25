#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum UserRole {
    Admin,
    #[default]
    User,
}
