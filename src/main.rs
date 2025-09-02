use anyhow::Result;

mod cli;
mod config;
mod extractor;
mod init;
mod models;
mod treesitter;
mod utils;

use crate::{cli::*, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = init();
    log::info!("Starting codeql-extractor v{}", VERSION);

    let mut config = match Config::load_with_arguments(&arguments.config, &arguments).await {
        Ok(config) => {
            log::info!("Loaded configuration from {}", arguments.config.display());
            config
        }
        Err(e) => {
            log::warn!("Failed to load configuration: {}", e);
            Config::default()
        }
    };

    match arguments.commands {
        Some(ArgumentCommands::Init {
            name,
            extensions,
            tree_sitter_repo,
        }) => {
            log::info!("Initializing working directory");

            // let ts_path = arguments.working_dir.join("extractor").join("tree-sitter");
            // let init_config = InitConfig {
            //     working_dir: arguments.working_dir.canonicalize()?,
            //     template_dir: arguments.template_dir,
            //     language_name: name.unwrap_or_else(|| input_prompt("Language name")),
            //     language_extensions: extensions
            //         .unwrap_or_else(|| input_prompt("Language extensions (comma separated)")),
            //     ts_repo: Repository::new(
            //         tree_sitter_repo.unwrap_or_else(|| input_prompt("Tree-sitter repository URL")),
            //         &ts_path,
            //     ),
            // };

            init::run(&config).await?;

            config.save().await?;
        }
        Some(ArgumentCommands::Update {}) => {
            log::info!("Updating extractor");

            extractor::update(&config).await?;
        }
        Some(ArgumentCommands::Extractor) => {
            log::info!("Building extractor");

            extractor::run(&config).await?;
        }
        None => {
            return Err(anyhow::anyhow!("No subcommand provided"));
        }
    }

    Ok(())
}
