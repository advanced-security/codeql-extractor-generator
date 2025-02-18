use anyhow::Result;
use models::Repository;

mod cli;
mod extractor;
mod init;
mod models;
mod treesitter;
mod utils;

use crate::{cli::*, models::InitConfig, utils::prompts::*};

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = init();
    log::info!("Starting codeql-extractor v{}", VERSION);

    match arguments.commands {
        Some(ArgumentCommands::Init {
            name,
            extensions,
            tree_sitter_repo,
        }) => {
            log::info!("Initializing working directory");

            let ts_path = arguments.working_dir.join("extractor").join("tree-sitter");
            let init_config = InitConfig {
                working_dir: arguments.working_dir.canonicalize()?,
                template_dir: arguments.template_dir,
                language_name: name.unwrap_or_else(|| input_prompt("Language name")),
                language_extensions: extensions
                    .unwrap_or_else(|| input_prompt("Language extensions (comma separated)")),
                ts_repo: Repository::new(
                    tree_sitter_repo.unwrap_or_else(|| input_prompt("Tree-sitter repository URL")),
                    &ts_path,
                ),
            };

            init::run(init_config).await?;
        }
        Some(ArgumentCommands::Extractor) => {
            log::info!("Building extractor");

            let config = InitConfig {
                working_dir: arguments.working_dir,
                template_dir: arguments.template_dir,
                ..Default::default()
            };
            extractor::run(&config).await?;
        }
        None => {
            return Err(anyhow::anyhow!("No subcommand provided"));
        }
    }

    Ok(())
}
