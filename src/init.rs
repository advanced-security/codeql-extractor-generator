use anyhow::Result;
use codeql_extractor::node_types::Entry;
use indicatif::{MultiProgress, ProgressBar};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{
    config::Config,
    models::extractor_config::{ExtractorConfig, FileType},
    treesitter::TreeSitterInfo,
    utils::prompts::input_prompt,
};

pub async fn run(config: &Config) -> Result<()> {
    // TODO: Working directory should be created if it doesn't exist

    let mut process_bar = MultiProgress::new();

    let extractor_config = create_config(&config)?;
    log::info!("Extractor config: {:?}", extractor_config);
    extractor_config
        .write(config.codeql_extractor_file())
        .await?;

    // let ts_info = tree_sitter_info(&config).await?;
    let ts_info = config.load_tree_sitter_info().await?;
    log::info!("Loaded TreeSitter info");

    let liquid_object = liquid::object!({
        "config": extractor_config,
        "file_types": extractor_config.file_types,
        "ts": ts_info.nodes,
    });
    log::trace!("Liquid object: {:#?}", liquid_object);

    let liquid_parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("Failed to build liquid parser");

    log::info!("Features enabled: {:?}", config.features);

    walk_dir(&config, &liquid_parser, &liquid_object, &mut process_bar).await?;

    if config.features.ast_internal {
        let ast_root = config.ast_dir();
        log::info!("AST Root :: {}", ast_root.display());

        let content = std::fs::read_to_string("./ast-internal.qll.jinja")?;
        let template = liquid_parser.parse(content.as_str())?;

        let internel = ast_root.join("internal");

        for (node_tname, node_entry) in ts_info.node_types.iter() {
            let name = node_entry.ql_class_name.clone();
            let full_path = internel.join(format!("{}.qll", name));

            let supertype = ts_info
                .nodes
                .iter()
                .find_map(|(stype, names)| {
                    if *stype != "all" && *stype != "Misc" && names.contains(&name) {
                        Some(stype.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or("AstNode".to_string());

            log::debug!(
                "Processing the `{}` node at `{}` ({})",
                name,
                full_path.display(),
                supertype
            );

            let liquid_object = liquid::object!({
                "name": name.clone(),
                "supertype": supertype,
                "ts_namespace": config.language_name.to_uppercase(),
            });

            let data = template.render(&liquid_object).map_err(|e| {
                log::error!("Error in: {}", full_path.display());
                e
            })?;

            tokio::fs::write(full_path, data.as_bytes()).await?;
        }
    }

    if config.features.extractor {
        // Commands to run
        log::info!("Running cargo fmt...");
        tokio::process::Command::new("cargo")
            .arg("fmt")
            .current_dir(config.working_dir.clone())
            .output()
            .await?;

        let create_extractor_script = PathBuf::from("./scripts/create-extractor-pack.sh");
        log::info!("Running `create-extractor-pack.sh`...");
        tokio::process::Command::new("bash")
            .arg(create_extractor_script)
            .current_dir(config.working_dir.clone())
            .status()
            .await?;

        let git_dir = config.working_dir.join(".git");
        if !git_dir.exists() {
            log::info!("Initializing git repository...");
            tokio::process::Command::new("git")
                .arg("init")
                .current_dir(config.working_dir.clone())
                .status()
                .await?;
        } else {
            log::info!("Git repository already exists, skipping initialization");
        }
    }

    // Install
    // log::info!("Running `codeql pack install ql/lib`...");
    // tokio::process::Command::new("codeql")
    //     .args(&["pack", "install", "ql/lib"])
    //     .current_dir(config.working_dir.clone())
    //     .status()
    //     .await?;

    // Build the Lib Packs
    log::info!("Running `codeql pack create ql/lib`...");
    tokio::process::Command::new("codeql")
        .args(&["pack", "create", "ql/lib"])
        .current_dir(config.working_dir.clone())
        .status()
        .await?;

    Ok(())
}

async fn generate_ast_file(
    root: &PathBuf,
    name: &String,
    entry: &Entry,
    liquid_parser: &liquid::Parser,
) -> Result<()> {
    let full_path = root.join(format!("{}.qll", name));
    log::info!(
        "Processing the `{}` node at `{}`",
        name,
        full_path.display()
    );

    // tokio::fs::write(full_path, b"").await?;

    Ok(())
}

async fn generate_ast_internal_file(
    root: &PathBuf,
    name: &String,
    entry: &Entry,
    template: &liquid::Template,
) -> Result<()> {
    Ok(())
}

pub async fn walk_dir(
    config: &Config,
    liquid_parser: &liquid::Parser,
    liquid_object: &liquid::Object,
    process_bar: &mut MultiProgress,
) -> Result<()> {
    log::info!("Walking directory: {:?}", config.template_dir);
    let files = WalkDir::new(config.template_dir.clone())
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path() != config.template_dir)
        .filter(|e| e.path().is_file())
        .collect::<Vec<_>>();

    let total = files.len();
    log::info!("Found {} files to process", total);

    for (index, entry) in files.into_iter().rev().enumerate() {
        process_bar.add(ProgressBar::new(index as u64 + 1));

        let template_path = entry.path();

        let filename = template_path
            .strip_prefix(config.template_dir.clone())?
            .components()
            .map(|c| {
                let loose = c.as_os_str().to_string_lossy();
                if loose.contains("EXTNAME") {
                    let extname = config.language_name.to_lowercase();
                    loose.to_string().replace("EXTNAME", extname.as_str())
                } else {
                    loose.to_string()
                }
            })
            .collect::<PathBuf>();

        let full_path = config.working_dir.join(filename.clone());
        log::debug!("Full path: {:?}", full_path);

        if template_path.is_dir() {
            log::info!("Creating directory: {:?}", full_path);
            std::fs::create_dir_all(&full_path)?;
            continue;
        }

        if !config.is_feature_enabled(&full_path) {
            log::debug!("Skipping file: {:?} (disabled in features)", full_path);
            continue;
        } else {
            log::info!("Processing file: {:?}", full_path);
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

        let rendered = template.render(liquid_object).map_err(|e| {
            log::error!("Error in: {}", full_path.display());
            log::trace!("TS: {:#?}", liquid_object.get("ts"));
            e
        })?;

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
                .current_dir(config.working_dir.clone())
                .status()
                .await?;
        }
    }

    Ok(())
}

fn create_config(config: &Config) -> Result<ExtractorConfig> {
    // Questions to ask:
    let language_display_name = match config.language_name.is_empty() {
        true => input_prompt("Language display name"),
        false => config.language_name.clone(),
    };
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

// async fn tree_sitter_info(config: &Config) -> Result<TreeSitterInfo> {
//     let ts_root = PathBuf::from("extractor/tree-sitter");
//     // TODO: Auto lookup for popular language grammars

//     // let rev = input_optional_prompt("Tree-sitter revision (optional)");
//     // let mut branch: Option<String> = None;
//     // if rev.is_none() {
//     //     branch = input_optional_prompt("Tree-sitter branch (optional)");
//     // }

//     // TODO: Submodule??

//     let ts = TreeSitterInfo::load(config, &ts_root).await?;

//     // let ts = treesitter::TreeSitter::load("lua", &node_types_json)?;

//     Ok(ts)
// }
