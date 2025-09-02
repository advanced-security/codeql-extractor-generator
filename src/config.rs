use anyhow::{Context, Result};
use codeql_extractor::extractor;
use liquid::{Object, Parser, Template};
use std::path::{self, PathBuf};
use walkdir::WalkDir;

use crate::{
    cli::{ArgumentCommands, Arguments},
    models::{extractor_config, ExtractorConfig},
    treesitter::{mappings::AstMap, TreeSitterInfo},
    utils::prompts,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    // Paths
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub working_dir: PathBuf,
    #[serde(default)]
    pub template_dir: PathBuf,

    pub extractor: String,
    #[serde(default)]
    pub language_name: String,
    #[serde(default)]
    pub language_extensions: Vec<String>,

    #[serde(default)]
    pub features: Features,

    #[serde(skip, default)]
    pub templates: Templates,

    #[serde(rename = "tree-sitter-repo")]
    pub tree_sitter_repo: Repository,

    #[serde(default)]
    pub mappings: Vec<AstMap>,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Features {
    #[serde(default)]
    pub extractor: bool,
    #[serde(default)]
    pub ast: bool,
    #[serde(default)]
    pub ast_internal: bool,
    #[serde(default)]
    pub cfg: bool,
    #[serde(default)]
    pub cfg_internal: bool,
    #[serde(default)]
    pub data_flow: bool,
    #[serde(default)]
    pub data_flow_internal: bool,
}

pub fn is_false(value: bool) -> bool {
    !value
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Templates {
    pub ast_internal: String,
}

impl Config {
    pub async fn load(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        log::info!("Loading configuration from {}", path.display());

        let content = tokio::fs::read_to_string(&path).await?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(Config { path, ..config })
    }

    pub async fn load_with_arguments(path: impl Into<PathBuf>, args: &Arguments) -> Result<Self> {
        let path = path.into();
        let full_path = args
            .working_dir
            .join(path)
            .canonicalize()
            .context("Failed to load path")?;

        let mut config = Config::load(&full_path).await?;
        config.working_dir = args.working_dir.clone().canonicalize().unwrap();
        config.template_dir = args.template_dir.clone().canonicalize().unwrap();

        config.tree_sitter_repo.path = config.working_dir.join("extractor").join("tree-sitter");

        match &args.commands {
            Some(ArgumentCommands::Init {
                name,
                extensions,
                tree_sitter_repo,
            }) => {
                if let Some(name) = name {
                    config.language_name = name.clone();
                }
                if let Some(extensions) = extensions {
                    config.language_extensions = extensions
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
                if let Some(repo) = tree_sitter_repo {
                    config.tree_sitter_repo.url = repo.clone();
                }
            }
            _ => {}
        }

        Ok(config)
    }

    pub async fn save(&self) -> Result<()> {
        log::info!("Saving configuration to {}", self.path.display());
        let content = serde_yaml::to_string(self)?;
        tokio::fs::write(&self.path, content).await?;
        Ok(())
    }

    pub async fn extractor_config(&self) -> Result<ExtractorConfig> {
        // TODO: Load
        log::debug!("Creating new extractor config");
        ExtractorConfig::from_config(self)
    }

    pub async fn load_tree_sitter_info(&self) -> Result<TreeSitterInfo> {
        let ts_path = self
            .tree_sitter_repo
            .path
            .clone()
            .canonicalize()
            .context("Failed to canonicalize tree-sitter path")?;
        log::info!("Loading TreeSitter from `{}`", ts_path.display());

        if !ts_path.exists() {
            self.tree_sitter_repo
                .clone_repo(self)
                .await
                .context("Failed to clone tree-sitter repository")?;
        }

        TreeSitterInfo::load(self, &ts_path).await
    }

    pub fn is_feature_enabled(&self, path: &PathBuf) -> bool {
        let parent = path
            .parent()
            .clone()
            .expect("Failed to get parent directory");
        // TODO: Improve this logic

        if !path.exists() {
            log::info!("File does not exist: {:?}", path);
            return true;
        }

        // Paths
        let extractor_dir = self.working_dir.join("extractor");
        let ast_dir = self.ast_dir();
        let ast_internal_dir = ast_dir.join("internal");
        let cfg_dir = self.cfg_dir();
        let cfg_internal_dir = cfg_dir.join("internal");
        let dataflow_dir = self
            .working_dir
            .join("ql")
            .join("lib")
            .join("codeql")
            .join(self.language_name.to_lowercase())
            .join("dataflow");
        let dataflow_internal_dir = dataflow_dir.join("internal");

        // Extractor
        if self.features.extractor && parent == extractor_dir {
            return true;
        }
        // AST
        else if self.features.ast && parent == ast_dir {
            return true;
        } else if self.features.ast_internal && parent == ast_internal_dir {
            return true;
        }
        // CFG
        else if self.features.cfg && parent == cfg_dir {
            return true;
        } else if self.features.cfg_internal && parent == cfg_internal_dir {
            return true;
        }
        // Data Flow
        else if self.features.data_flow && parent == dataflow_dir {
            return true;
        } else if self.features.data_flow_internal && parent == dataflow_internal_dir {
            return true;
        }

        false
    }

    pub fn codeql_extractor_file(&self) -> PathBuf {
        self.working_dir
            .join("codeql-extractor.yml")
            .canonicalize()
            .unwrap_or_else(|_| {
                log::warn!("Failed to canonicalize codeql-extractor.yml path, using original path");
                self.working_dir.join("codeql-extractor.yml")
            })
    }

    pub fn ast_dir(&self) -> PathBuf {
        self.working_dir
            .join("ql")
            .join("lib")
            .join("codeql")
            .join(self.language_name.to_lowercase())
            .join("ast")
            .canonicalize()
            .unwrap()
    }

    pub fn cfg_dir(&self) -> PathBuf {
        self.working_dir
            .join("ql")
            .join("lib")
            .join("codeql")
            .join(self.language_name.to_lowercase())
            .join("controlflow")
            .canonicalize()
            .unwrap()
    }

    pub async fn load_parser(&self) -> Result<(Parser, Object)> {
        let ts_info = self.load_tree_sitter_info().await?;
        let extractor_config = self.extractor_config().await?;

        let liquid_object = liquid::object!({
            "config": extractor_config,
            "file_types": extractor_config.file_types,
            "ts": ts_info.nodes,
        });
        log::trace!("Liquid object: {:#?}", liquid_object);

        let liquid_parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .expect("Failed to build liquid parser");

        Ok((liquid_parser, liquid_object))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: PathBuf::from(".codeql/generator.yml"),
            working_dir: PathBuf::from("./"),
            template_dir: PathBuf::from("./templates"),
            extractor: String::from("codeql-extractor"),
            language_name: String::new(),
            language_extensions: Vec::new(),
            features: Features::default(),
            templates: Templates::default(),
            tree_sitter_repo: Repository::default(),
            mappings: Vec::new(),
        }
    }
}

impl Default for Templates {
    fn default() -> Self {
        Self {
            ast_internal: String::from("ast-internal.qll.jinja"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Repository {
    pub url: String,
    #[serde(default)]
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            url: String::new(),
            path: PathBuf::from("./extractor/tree-sitter"),
            rev: None,
            branch: None,
        }
    }
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

    pub async fn clone_repo(&self, config: &Config) -> Result<()> {
        log::info!(
            "Cloning repository from `{}` to `{}`",
            self.url,
            self.path.display()
        );
        let status = tokio::process::Command::new("git")
            .args(&[
                "clone",
                "--depth=1",
                self.url.as_str(),
                self.path.to_str().unwrap(),
            ])
            .current_dir(config.working_dir.as_os_str())
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
