use anyhow::Result;

/// Application settings
#[derive(Debug, Clone)]
pub struct Settings {
    /// Path to the config file
    pub config_path: String,
}

impl Settings {
    /// Create new settings from command line arguments
    pub fn new(config_path: String) -> Result<Self> {
        Ok(Self { config_path })
    }
}