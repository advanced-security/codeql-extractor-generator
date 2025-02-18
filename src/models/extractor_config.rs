use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
