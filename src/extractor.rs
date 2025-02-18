use anyhow::Result;
use crate::models;


pub async fn run(
    config: &models::InitConfig
) -> Result<()> {
    let extractor = models::ExtractorConfig::load(
        config.working_dir.join("codeql-extractor.yml"),
    ).await?;
    log::info!("Extractor config: {:?}", extractor);

    Ok(())
}
