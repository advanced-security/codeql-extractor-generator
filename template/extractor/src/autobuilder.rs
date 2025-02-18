use std::env;
use std::path::PathBuf;

use clap::Args;

use codeql_extractor::autobuilder;

#[derive(Args)]
// The autobuilder takes no command-line options, but this may change in the future.
pub struct Options {}

pub fn run(_: Options) -> std::io::Result<()> {
    let database = env::var("CODEQL_EXTRACTOR_{{ config.name | upcase }}_WIP_DATABASE")
        .expect("CODEQL_EXTRACTOR_{{ config.name | upcase }}_WIP_DATABASE not set");

    autobuilder::Autobuilder::new("{{ config.name }}", PathBuf::from(database))
        .include_extensions(&[
            {%- for ft in file_types %}
            {%- for ext in ft.extensions %}
            "{{ ext }}",
            {%- endfor %}
            {%- endfor %}
        ])
        .exclude_globs(&["**/.git"])
        .size_limit("10m")
        .run()
}