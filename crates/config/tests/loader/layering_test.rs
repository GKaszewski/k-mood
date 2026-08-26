use figment::Jail;

use config::{MediaBackend, load};

const CONFIG_FILE: &str = "config.toml";

#[test]
fn with_no_file_and_no_environment_every_default_stands() {
    Jail::expect_with(|_jail| {
        let loaded = load().unwrap();

        assert_eq!(loaded.server.port, 3000);
        assert_eq!(loaded.server.host, "0.0.0.0");
        assert_eq!(loaded.storage.data_dir, "./data");
        assert!(loaded.auth.jwt_secret.is_none());
        Ok(())
    });
}

#[test]
fn a_file_overrides_the_defaults_it_names() {
    Jail::expect_with(|jail| {
        jail.create_file(CONFIG_FILE, "[server]\nport = 4321\n")?;

        let loaded = load().unwrap();

        assert_eq!(loaded.server.port, 4321);
        assert_eq!(loaded.server.host, "0.0.0.0");
        Ok(())
    });
}

#[test]
fn the_environment_overrides_the_file() {
    Jail::expect_with(|jail| {
        jail.create_file(CONFIG_FILE, "[server]\nport = 4321\nhost = \"127.0.0.1\"\n")?;
        jail.set_env("KMOOD_SERVER__PORT", "8080");

        let loaded = load().unwrap();

        assert_eq!(loaded.server.port, 8080);
        assert_eq!(loaded.server.host, "127.0.0.1");
        Ok(())
    });
}

#[test]
fn a_double_underscore_descends_a_section() {
    Jail::expect_with(|jail| {
        jail.set_env("KMOOD_SERVER__CORS__ALLOW_ANY_ORIGIN", "false");

        let loaded = load().unwrap();

        assert!(!loaded.server.cors.allow_any_origin);
        Ok(())
    });
}

#[test]
fn a_single_underscore_stays_inside_a_field_name() {
    Jail::expect_with(|jail| {
        jail.set_env("KMOOD_STORAGE__DATA_DIR", "/var/lib/kmood");
        jail.set_env("KMOOD_AUTH__JWT_SECRET", "from-the-environment");

        let loaded = load().unwrap();

        assert_eq!(loaded.storage.data_dir, "/var/lib/kmood");
        assert_eq!(
            loaded.auth.jwt_secret.as_deref(),
            Some("from-the-environment")
        );
        Ok(())
    });
}

#[test]
fn the_environment_can_switch_the_media_backend() {
    Jail::expect_with(|jail| {
        jail.set_env("KMOOD_STORAGE__MEDIA__BACKEND", "s3");
        jail.set_env("KMOOD_STORAGE__MEDIA__BUCKET", "moods");
        jail.set_env("KMOOD_STORAGE__MEDIA__REGION", "eu-central-1");

        let loaded = load().unwrap();

        match loaded.storage.media {
            MediaBackend::S3 { bucket, region, .. } => {
                assert_eq!(bucket, "moods");
                assert_eq!(region, "eu-central-1");
            }
            other => panic!("expected the s3 backend, got {other:?}"),
        }
        Ok(())
    });
}

#[test]
fn an_unrelated_variable_leaves_the_configuration_alone() {
    Jail::expect_with(|jail| {
        jail.set_env("PORT", "9999");
        jail.set_env("SERVER__PORT", "9999");

        let loaded = load().unwrap();

        assert_eq!(loaded.server.port, 3000);
        Ok(())
    });
}

#[test]
fn a_list_arrives_as_an_array() {
    Jail::expect_with(|jail| {
        jail.set_env(
            "KMOOD_SERVER__CORS__ALLOWED_ORIGINS",
            r#"["https://a.example","https://b.example"]"#,
        );

        let loaded = load().unwrap();

        assert_eq!(
            loaded.server.cors.allowed_origins,
            vec!["https://a.example", "https://b.example"]
        );
        Ok(())
    });
}

#[test]
fn the_file_to_read_can_be_named_by_the_environment() {
    Jail::expect_with(|jail| {
        jail.create_file("elsewhere.toml", "[server]\nport = 7777\n")?;
        jail.set_env("KMOOD_CONFIG_FILE", "elsewhere.toml");

        let loaded = load().unwrap();

        assert_eq!(loaded.server.port, 7777);
        Ok(())
    });
}
