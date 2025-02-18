use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub mod extractor_config;

pub use extractor_config::ExtractorConfig;

#[derive(Debug, Default, Clone)]
pub struct InitConfig {
    pub working_dir: PathBuf,
    pub template_dir: PathBuf,

    pub language_name: String,
    pub language_extensions: String,
    pub ts_repo: Repository,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Repository {
    pub url: String,
    pub path: PathBuf,
    pub rev: Option<String>,
    pub branch: Option<String>,
}

impl Repository {
    pub fn new(url: String, path: &PathBuf) -> Self {
        Self {
            url,
            path: path.clone(),
            rev: None,
            branch: None,
        }
    }

    pub async fn clone_repo(&self, init_config: &InitConfig) -> Result<()> {
        let status = tokio::process::Command::new("git")
            .args(&[
                "clone",
                "--depth=1",
                self.url.as_str(),
                self.path.to_str().unwrap(),
            ])
            .current_dir(init_config.working_dir.as_os_str())
            .status()
            .await?;
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to clone repository"));
        }
        Ok(())
    }

    pub fn find_file(&self, name: &str) -> Result<PathBuf> {
        Ok(WalkDir::new(self.path.clone())
            .contents_first(true)
            .into_iter()
            .filter_map(Result::ok)
            .find(|e| e.path().ends_with(name))
            .ok_or_else(|| anyhow::anyhow!("Failed to find `{}` file", name))?
            .into_path()
            .canonicalize()?)
    }
}
