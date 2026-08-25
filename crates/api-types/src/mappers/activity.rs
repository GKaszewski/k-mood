use domain::activity::{Activity, ActivityId, ActivityName, CategoryName};
use domain::user::UserId;

use application::activity::commands::{
    CreateActivityCommand, RenameActivityCommand, SetCategoryCommand,
};

use crate::errors::ApiValidationError;
use crate::requests::{CreateActivityRequest, RenameActivityRequest, SetCategoryRequest};
use crate::responses::ActivityResponse;

impl CreateActivityRequest {
    pub fn into_command(
        self,
        user_id: UserId,
    ) -> Result<CreateActivityCommand, ApiValidationError> {
        let name = ActivityName::new(self.name)?;
        let category = self.category.map(CategoryName::new).transpose()?;

        Ok(CreateActivityCommand {
            user_id,
            name,
            category,
        })
    }
}

impl RenameActivityRequest {
    pub fn into_command(
        self,
        activity_id: ActivityId,
    ) -> Result<RenameActivityCommand, ApiValidationError> {
        let new_name = ActivityName::new(self.new_name)?;

        Ok(RenameActivityCommand {
            activity_id,
            new_name,
        })
    }
}

impl SetCategoryRequest {
    pub fn into_command(
        self,
        activity_id: ActivityId,
    ) -> Result<SetCategoryCommand, ApiValidationError> {
        let category = self.category.map(CategoryName::new).transpose()?;

        Ok(SetCategoryCommand {
            activity_id,
            category,
        })
    }
}

impl From<Activity> for ActivityResponse {
    fn from(activity: Activity) -> Self {
        Self {
            id: activity.id().value(),
            name: activity.name().value().to_string(),
            category: activity.category().map(|c| c.value().to_string()),
            archived: activity.is_archived(),
            created_at: *activity.created_at(),
        }
    }
}
