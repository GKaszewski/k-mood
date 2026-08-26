use figment::Jail;

use config::load;

const CONFIG_FILE: &str = "config.toml";

#[test]
fn a_malformed_file_is_refused_rather_than_ignored() {
    Jail::expect_with(|jail| {
        jail.create_file(CONFIG_FILE, "[server\nport = ")?;

        assert!(load().is_err());
        Ok(())
    });
}

#[test]
fn a_value_of_the_wrong_type_is_refused() {
    Jail::expect_with(|jail| {
        jail.set_env("KMOOD_SERVER__PORT", "not-a-port");

        assert!(load().is_err());
        Ok(())
    });
}

#[test]
fn a_refusal_names_the_key_at_fault() {
    Jail::expect_with(|jail| {
        jail.set_env("KMOOD_SERVER__PORT", "not-a-port");

        let message = load().unwrap_err().to_string().to_lowercase();

        assert!(message.contains("port"), "unhelpful message: {message}");
        Ok(())
    });
}

#[test]
fn a_named_file_that_is_missing_is_refused_rather_than_ignored() {
    Jail::expect_with(|jail| {
        jail.set_env("KMOOD_CONFIG_FILE", "nowhere.toml");

        assert!(load().is_err());
        Ok(())
    });
}
