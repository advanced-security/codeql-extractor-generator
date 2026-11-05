use crate::{config::Config, init::walk_dir, models, utils::prompts};
use anyhow::Result;
use indicatif::MultiProgress;

pub async fn run(config: &Config) -> Result<()> {
    let extractor =
        models::ExtractorConfig::load(config.working_dir.join("codeql-extractor.yml")).await?;
    log::info!("Extractor config: {:?}", extractor);

    Ok(())
}

pub async fn update(config: &Config) -> Result<()> {
    let features = prompts::select("Select features to update", &["AST Internals"])?;

    let mut process_bar = MultiProgress::new();

    for feature in features {
        match feature.as_str() {
            "AST Internals" => {
                log::info!("Updating AST Internals");
                ast_internals(config).await?;
            }
            _ => {
                log::warn!("Unknown feature: {}", feature);
            }
        }
    }

    Ok(())
}

pub async fn ast_internals(config: &Config) -> Result<()> {
    let ast_root = config.ast_dir();
    let internel = ast_root.join("internal");
    log::info!("AST Root :: {}", ast_root.display());

    let ts_info = config.load_tree_sitter_info().await?;
    let extractor_config = config.extractor_config().await?;

    let (liquid_parser, liquid_object) = config.load_parser().await?;

    let content_path = config
        .template_dir
        .join("..")
        .join("ast-internal.qll.jinja")
        .canonicalize()?;
    let content = std::fs::read_to_string(content_path)?;
    let template = liquid_parser.parse(content.as_str())?;

    let template = liquid_parser.parse(content.as_str())?;

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
    Ok(())
}
