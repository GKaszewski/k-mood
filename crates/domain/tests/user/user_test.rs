use domain::user::{DisplayName, Email, PasswordHash, Timezone, User, UserRole, Username};

#[test]
fn new_user_has_default_role() {
    let user = User::new(
        Username::new("gabriel").unwrap(),
        Email::new("gabriel@example.com").unwrap(),
        PasswordHash::new("hashed".into()),
    );

    assert_eq!(user.role(), UserRole::User);
    assert!(!user.is_admin());
    assert!(user.display_name().is_none());
    assert!(user.timezone().is_none());
}

#[test]
fn promote_to_admin_changes_role() {
    let mut user = User::new(
        Username::new("gabriel").unwrap(),
        Email::new("gabriel@example.com").unwrap(),
        PasswordHash::new("hashed".into()),
    );

    user.promote_to_admin();

    assert_eq!(user.role(), UserRole::Admin);
    assert!(user.is_admin());
}

#[test]
fn update_display_name_touches_updated_at() {
    let mut user = User::new(
        Username::new("gabriel").unwrap(),
        Email::new("gabriel@example.com").unwrap(),
        PasswordHash::new("hashed".into()),
    );
    let before = *user.updated_at();

    let name = DisplayName::new("Gabriel K").unwrap();
    user.update_display_name(Some(name));

    assert_eq!(user.display_name().map(|d| d.value()), Some("Gabriel K"));
    assert!(*user.updated_at() >= before);
}

#[test]
fn update_timezone_sets_value() {
    let mut user = User::new(
        Username::new("gabriel").unwrap(),
        Email::new("gabriel@example.com").unwrap(),
        PasswordHash::new("hashed".into()),
    );

    let tz = Timezone::new("Europe/Warsaw").unwrap();
    user.update_timezone(Some(tz));

    assert_eq!(user.timezone().map(|t| t.value()), Some("Europe/Warsaw"));
}
