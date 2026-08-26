#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("configuration file {0} was named by the environment but does not exist")]
    NamedFileMissing(String),
    #[error("could not read configuration: {0}")]
    Unreadable(#[from] Box<figment::Error>),
}
