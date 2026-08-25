use chrono::{DateTime, Utc};

use crate::errors::DomainError;
use crate::user::UserId;

use super::{ActivityId, ActivityName, CategoryName};

#[derive(Debug, Clone)]
pub struct Activity {
    id: ActivityId,
    user_id: UserId,
    name: ActivityName,
    category: Option<CategoryName>,
    archived: bool,
    created_at: DateTime<Utc>,
}

impl Activity {
    pub fn new(user_id: UserId, name: ActivityName, category: Option<CategoryName>) -> Self {
        Self {
            id: ActivityId::generate(),
            user_id,
            name,
            category,
            archived: false,
            created_at: Utc::now(),
        }
    }

    pub fn from_persistence(
        id: ActivityId,
        user_id: UserId,
        name: ActivityName,
        category: Option<CategoryName>,
        archived: bool,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            name,
            category,
            archived,
            created_at,
        }
    }
}

impl Activity {
    pub fn id(&self) -> &ActivityId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn name(&self) -> &ActivityName {
        &self.name
    }

    pub fn category(&self) -> Option<&CategoryName> {
        self.category.as_ref()
    }

    pub fn is_archived(&self) -> bool {
        self.archived
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

impl Activity {
    pub fn rename(&mut self, name: ActivityName) {
        self.name = name;
    }

    pub fn set_category(&mut self, category: Option<CategoryName>) {
        self.category = category;
    }

    pub fn archive(&mut self) -> Result<(), DomainError> {
        if self.archived {
            return Err(DomainError::Conflict("activity is already archived".into()));
        }
        self.archived = true;
        Ok(())
    }

    pub fn unarchive(&mut self) -> Result<(), DomainError> {
        if !self.archived {
            return Err(DomainError::Conflict("activity is not archived".into()));
        }
        self.archived = false;
        Ok(())
    }
}
