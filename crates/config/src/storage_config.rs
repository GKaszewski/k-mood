const DEFAULT_DATA_DIR: &str = "./data";
const DEFAULT_MEDIA_DIR: &str = "media";

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "backend")]
pub enum MediaBackend {
    #[serde(rename = "local")]
    Local { media_dir: String },
    #[serde(rename = "s3")]
    S3 {
        bucket: String,
        region: String,
        endpoint: Option<String>,
        access_key: Option<String>,
        secret_key: Option<String>,
    },
}

impl Default for MediaBackend {
    fn default() -> Self {
        Self::Local {
            media_dir: DEFAULT_MEDIA_DIR.into(),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    pub data_dir: String,
    pub database_url: Option<String>,
    pub media: MediaBackend,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: DEFAULT_DATA_DIR.into(),
            database_url: None,
            media: MediaBackend::default(),
        }
    }
}
