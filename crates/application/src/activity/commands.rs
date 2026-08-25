use domain::activity::{ActivityId, ActivityName, CategoryName};
use domain::user::UserId;

#[derive(Debug)]
pub struct CreateActivityCommand {
    pub user_id: UserId,
    pub name: ActivityName,
    pub category: Option<CategoryName>,
}

#[derive(Debug)]
pub struct RenameActivityCommand {
    pub activity_id: ActivityId,
    pub new_name: ActivityName,
}

#[derive(Debug)]
pub struct SetCategoryCommand {
    pub activity_id: ActivityId,
    pub category: Option<CategoryName>,
}
