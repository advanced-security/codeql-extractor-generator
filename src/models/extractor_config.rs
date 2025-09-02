use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractorConfig {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub column_kind: String,
    pub legacy_qltest_extraction: bool,
    pub build_modes: Vec<String>,
    pub github_api_languages: Vec<String>,
    pub scc_languages: Vec<String>,
    pub file_types: Vec<FileType>,
}

impl ExtractorConfig {
    pub fn from_config(config: &Config) -> Result<Self> {
        // Questions to ask:
        let language_display_name = config.language_name.clone();
        // .input_prompt("Language name"))

        let language_name = language_display_name.to_lowercase();

        let extensions: Vec<String> = config.language_extensions.clone();

        // Create the config
        let file_type: FileType = FileType {
            name: language_name.clone(),
            display_name: language_display_name.clone(),
            extensions,
        };

        let config = ExtractorConfig {
            name: language_name,
            display_name: language_display_name.to_string(),
            version: "0.1.0".to_string(),
            column_kind: "utf8".to_string(),
            legacy_qltest_extraction: true,
            build_modes: vec!["none".to_string()],
            github_api_languages: vec![language_display_name.to_string()],
            scc_languages: vec![language_display_name.to_string()],
            file_types: vec![file_type],
        };

        log::debug!("{:?}", config);
        Ok(config)
    }

    /// Read and load YAML file
    pub async fn load(path: PathBuf) -> Result<Self> {
        log::debug!("Loading config from {:?}", path);

        let file = tokio::fs::read(path).await?;
        let config: ExtractorConfig = serde_yaml::from_slice(&file)?;
        Ok(config)
    }

    /// Write the config to a YAML file
    pub async fn write(&self, path: PathBuf) -> Result<()> {
        log::debug!("Writing config to {:?}", path);

        let file = serde_yaml::to_string(&self)?;
        tokio::fs::write(path, file).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    pub name: String,
    pub display_name: String,
    pub extensions: Vec<String>,
}
