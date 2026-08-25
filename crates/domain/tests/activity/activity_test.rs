use domain::activity::{Activity, ActivityName, CategoryName};
use domain::errors::DomainError;
use domain::user::UserId;

#[test]
fn new_activity_is_not_archived() {
    let category = CategoryName::new("hobbies").unwrap();
    let activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        Some(category.clone()),
    );

    assert!(!activity.is_archived());
    assert_eq!(activity.name().value(), "gaming");
    assert_eq!(activity.category().map(|c| c.value()), Some("hobbies"));
}

#[test]
fn archive_sets_archived_flag() {
    let mut activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        None,
    );

    activity.archive().unwrap();

    assert!(activity.is_archived());
}

#[test]
fn archiving_already_archived_activity_fails() {
    let mut activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        None,
    );

    activity.archive().unwrap();
    let result = activity.archive();

    assert!(matches!(result, Err(DomainError::Conflict(_))));
}

#[test]
fn unarchive_clears_archived_flag() {
    let mut activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        None,
    );

    activity.archive().unwrap();
    activity.unarchive().unwrap();

    assert!(!activity.is_archived());
}

#[test]
fn unarchiving_active_activity_fails() {
    let mut activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        None,
    );

    let result = activity.unarchive();

    assert!(matches!(result, Err(DomainError::Conflict(_))));
}

#[test]
fn rename_updates_name() {
    let mut activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        None,
    );

    activity.rename(ActivityName::new("video games").unwrap());

    assert_eq!(activity.name().value(), "video games");
}

#[test]
fn set_category_updates_category() {
    let mut activity = Activity::new(
        UserId::generate(),
        ActivityName::new("gaming").unwrap(),
        None,
    );

    activity.set_category(Some(CategoryName::new("hobbies").unwrap()));
    assert_eq!(activity.category().map(|c| c.value()), Some("hobbies"));

    activity.set_category(None);
    assert_eq!(activity.category(), None);
}
