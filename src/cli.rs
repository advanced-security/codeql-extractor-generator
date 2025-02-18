use clap::{Parser, Subcommand};
use console::style;
use std::path::PathBuf;

pub const BANNER: &str = r#"
________  .____     ________               
\_____  \ |    |   /  _____/  ____   ____  
 /  / \  \|    |  /   \  ____/ __ \ /    \ 
/   \_/.  \    |__\    \_\  \  ___/|   |  \
\_____\ \_/_______ \______  /\___  >___|  /
       \__>       \/      \/     \/     \/"#;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Enable Debugging
    #[clap(long, env, default_value_t = false)]
    pub debug: bool,

    /// Disable Banner
    #[clap(long, default_value_t = false)]
    pub disable_banner: bool,

    /// Configuration file path
    #[clap(short, long, env, default_value = "./template")]
    pub template_dir: PathBuf,

    /// Working Directory
    #[clap(short, long, env, default_value = "./")]
    pub working_dir: PathBuf,

    /// Subcommands
    #[clap(subcommand)]
    pub commands: Option<ArgumentCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ArgumentCommands {
    /// Initialize the working directory
    Init {
        /// Language name
        #[clap(short, long, env, default_value = "")]
        name: Option<String>,

        /// Language Extensions (comma-separated)
        #[clap(short, long, env, default_value = "")]
        extensions: Option<String>,

        /// Tree-sitter repository
        #[clap(short, long, env, default_value = "")]
        tree_sitter_repo: Option<String>,
    },
    Extractor,
}

pub fn init() -> Arguments {
    let arguments = Arguments::parse();

    let log_level = match &arguments.debug {
        false => log::LevelFilter::Info,
        true => log::LevelFilter::Debug,
    };

    env_logger::builder()
        .filter_level(log_level)
        .parse_default_env()
        .filter_level(log_level)
        .init();

    if !arguments.disable_banner {
        println!(
            "{}    {} - v{}\n",
            style(BANNER).green(),
            style(AUTHOR).red(),
            style(VERSION).blue()
        );
    }

    arguments
}
