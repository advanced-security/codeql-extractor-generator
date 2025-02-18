use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(i64),
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreeSitterGrammar {
    pub name: String,
    pub word: Option<String>,

    pub rules: HashMap<String, TreeSitterRule>,

    pub extras: Vec<TreeSitterRule>,

    pub conflicts: Vec<Vec<String>>,

    pub precedences: Vec<Vec<TreeSitterRuleMember>>,
    pub externals: Vec<TreeSitterRule>,

    pub inline: Vec<String>,
    pub supertypes: Vec<String>,
}

impl TreeSitterGrammar {
    pub fn load(path: PathBuf) -> Result<Self, std::io::Error> {
        log::debug!("Loading grammar from `{:?}`", path);
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let grammar = serde_json::from_reader(reader)?;
        Ok(grammar)
    }
}

/// https://github.com/github/codeql/blob/main/shared/tree-sitter-extractor/src/node_types.rs#L416
pub fn dbscheme_name_to_class_name(dbscheme_name: &str) -> String {
    fn to_title_case(word: &str) -> String {
        let mut first = true;
        let mut result = String::new();
        for c in word.chars() {
            if first {
                first = false;
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
        }
        result
    }
    dbscheme_name
        .split('_')
        .map(to_title_case)
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TreeSitterRule {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub content: Option<TreeSitterRuleMember>,
    pub members: Option<Vec<TreeSitterRuleMember>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TreeSitterRuleMember {
    #[serde(rename = "type")]
    pub member_type: String,
    pub name: Option<String>,
    pub value: Option<StringOrNumber>,
}
