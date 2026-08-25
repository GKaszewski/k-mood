use crate::activity::{ActivityId, ActivityName};
use crate::entry::MoodEntryId;
use crate::reminder::ReminderId;
use crate::user::UserId;

#[derive(Debug, Clone)]
pub enum DomainEvent {
    EntryCreated {
        entry_id: MoodEntryId,
        user_id: UserId,
    },
    EntryUpdated {
        entry_id: MoodEntryId,
        user_id: UserId,
    },
    EntryDeleted {
        entry_id: MoodEntryId,
        user_id: UserId,
    },

    ActivityCreated {
        activity_id: ActivityId,
        user_id: UserId,
    },
    ActivityRenamed {
        activity_id: ActivityId,
        user_id: UserId,
        old_name: ActivityName,
        new_name: ActivityName,
    },
    ActivityArchived {
        activity_id: ActivityId,
        user_id: UserId,
    },
    ActivityUnarchived {
        activity_id: ActivityId,
        user_id: UserId,
    },

    ReminderCreated {
        reminder_id: ReminderId,
        user_id: UserId,
    },
    ReminderUpdated {
        reminder_id: ReminderId,
        user_id: UserId,
    },
    ReminderDeleted {
        reminder_id: ReminderId,
        user_id: UserId,
    },

    UserRegistered {
        user_id: UserId,
    },
}
