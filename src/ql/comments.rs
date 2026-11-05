use anyhow::Result;
use codeql_extractor::generator::ql::{Class, Type};
use codeql_extractor::node_types::Entry;

pub fn generate_comment(entry: &Entry) -> Result<String> {
    log::info!("Comment :: {:?}", entry);

    let comment_class = Class {
        name: "Comment",
        is_abstract: false,
        qldoc: Some("A comment in the source code.".to_string()),
        supertypes: vec![Type::At("AstNode"), Type::At("TComment")]
            .into_iter()
            .collect(),
        characteristic_predicate: None,
        predicates: vec![].into_iter().collect(),
    };

    Ok(comment_class.to_string())
}
