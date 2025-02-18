use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{
    models::{
        extractor_config::{ExtractorConfig, FileType},
        InitConfig,
    },
    treesitter::TreeSitterInfo,
};

pub async fn run(init_config: InitConfig) -> Result<()> {
    // TODO: Working directory should be created if it doesn't exist

    let mut process_bar = MultiProgress::new();

    let config = create_config(&init_config)?;
    config
        .write(init_config.working_dir.join("codeql-extractor.yml"))
        .await?;

    let ts_info = tree_sitter_info(&init_config).await?;

    let liquid_object = liquid::object!({
        "config": config,
        "file_types": config.file_types,
        "ts": ts_info.nodes,
    });
    log::trace!("Liquid object: {:#?}", liquid_object);

    let liquid_parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("Failed to build liquid parser");

    walk_dir(
        &init_config,
        &liquid_parser,
        &liquid_object,
        &mut process_bar,
    )
    .await?;

    // Commands to run
    log::info!("Running cargo fmt...");
    tokio::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(init_config.working_dir.clone())
        .output()
        .await?;

    let create_extractor_script = PathBuf::from("./scripts/create-extractor-pack.sh");
    log::info!("Running `create-extractor-pack.sh`...");
    tokio::process::Command::new("bash")
        .arg(create_extractor_script)
        .current_dir(init_config.working_dir.clone())
        .status()
        .await?;

    let git_dir = init_config.working_dir.join(".git");
    if !git_dir.exists() {
        log::info!("Initializing git repository...");
        tokio::process::Command::new("git")
            .arg("init")
            .current_dir(init_config.working_dir.clone())
            .status()
            .await?;
    } else {
        log::info!("Git repository already exists, skipping initialization");
    }

    // Install
    log::info!("Running `codeql pack install ql/lib`...");
    tokio::process::Command::new("codeql")
        .args(&["pack", "install", "ql/lib"])
        .current_dir(init_config.working_dir.clone())
        .status()
        .await?;

    // Build the Lib Packs
    log::info!("Running `codeql pack create ql/lib`...");
    tokio::process::Command::new("codeql")
        .args(&["pack", "create", "ql/lib"])
        .current_dir(init_config.working_dir.clone())
        .status()
        .await?;

    Ok(())
}

async fn walk_dir(
    init_config: &InitConfig,
    liquid_parser: &liquid::Parser,
    liquid_object: &liquid::Object,
    process_bar: &mut MultiProgress,
) -> Result<()> {
    let files = WalkDir::new(init_config.template_dir.clone())
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path() != init_config.template_dir)
        .filter(|e| e.path().is_file())
        .collect::<Vec<_>>();

    let total = files.len();
    log::info!("Found {} files to process", total);

    for (index, entry) in files.into_iter().rev().enumerate() {
        process_bar.add(ProgressBar::new(index as u64 + 1));

        let template_path = entry.path();

        let filename = template_path
            .strip_prefix(init_config.template_dir.clone())?
            .components()
            .map(|c| {
                let loose = c.as_os_str().to_string_lossy();
                if loose.contains("EXTNAME") {
                    let extname = init_config.language_name.to_lowercase();
                    loose.to_string().replace("EXTNAME", extname.as_str())
                } else {
                    loose.to_string()
                }
            })
            .collect::<PathBuf>();

        let full_path = init_config.working_dir.join(filename.clone());

        if template_path.is_dir() {
            log::info!("Creating directory: {:?}", full_path);
            std::fs::create_dir_all(&full_path)?;
            continue;
        }

        log::debug!("Processing file: {:?} -> {:?}", filename, full_path);

        // Parent directory creation
        let parent = full_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory for {:?}", full_path))?;
        std::fs::create_dir_all(&parent)?;
        log::debug!("Parent directory created: {:?}", parent);

        // Render the file
        let content = std::fs::read_to_string(&template_path)?;
        let template = liquid_parser.parse(content.as_str())?;
        let rendered = template.render(liquid_object)?;

        // Write the file
        if full_path.exists() {
            log::debug!("File exists, overwriting: {:?}", full_path);
        } else {
            log::debug!("Creating file: {:?}", full_path);
        }
        std::fs::write(&full_path, rendered.as_bytes())?;

        // TODO
        if full_path.file_name().map(|e| e.to_str()) == Some(Some("AstNodes.qll")) {
            let file = full_path.as_os_str().to_str().unwrap();
            tokio::process::Command::new("codeql")
                .args(&["query", "format", "-o", file.clone(), file.clone()])
                .current_dir(init_config.working_dir.clone())
                .status()
                .await?;
        }
    }

    Ok(())
}

fn create_config(init_config: &InitConfig) -> Result<ExtractorConfig> {
    // Questions to ask:
    let language_display_name = init_config.language_name.clone();
    // .input_prompt("Language name"))

    let language_name = language_display_name.to_lowercase();

    let extensions: Vec<String> = init_config
        .language_extensions
        .clone()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    // Create the config
    let file_types: Vec<FileType> = extensions
        .iter()
        .map(|ext| FileType {
            name: language_name.clone(),
            display_name: language_display_name.clone(),
            extensions: vec![ext.clone()],
        })
        .collect();

    let config = ExtractorConfig {
        name: language_name,
        display_name: language_display_name.to_string(),
        version: "0.1.0".to_string(),
        column_kind: "utf8".to_string(),
        legacy_qltest_extraction: true,
        build_modes: vec!["none".to_string()],
        github_api_languages: vec![language_display_name.to_string()],
        scc_languages: vec![language_display_name.to_string()],
        file_types,
    };

    log::debug!("{:?}", config);
    Ok(config)
}

async fn tree_sitter_info(init_config: &InitConfig) -> Result<TreeSitterInfo> {
    let ts_root = PathBuf::from("extractor/tree-sitter");
    // TODO: Auto lookup for popular language grammars

    // let rev = input_optional_prompt("Tree-sitter revision (optional)");
    // let mut branch: Option<String> = None;
    // if rev.is_none() {
    //     branch = input_optional_prompt("Tree-sitter branch (optional)");
    // }

    // TODO: Submodule??

    let ts = TreeSitterInfo::load(init_config, &ts_root).await?;

    // let ts = treesitter::TreeSitter::load("lua", &node_types_json)?;

    Ok(ts)
}
