use domain::activity::{Activity, ActivityId, ActivityName, CategoryName};
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct ActivityRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub category: Option<String>,
    pub archived: bool,
    pub created_at: String,
}

impl ActivityRow {
    pub fn into_domain(self) -> Activity {
        Activity::from_persistence(
            ActivityId::from_uuid(self.id.parse().unwrap()),
            UserId::from_uuid(self.user_id.parse().unwrap()),
            ActivityName::from_persistence(self.name),
            self.category.map(CategoryName::from_persistence),
            self.archived,
            self.created_at.parse().unwrap(),
        )
    }
}
