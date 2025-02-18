use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Mappings {
    mappings: Vec<AstMap>,
}

impl Mappings {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str::<Self>(&content)?)
    }
}

// Iterator over all mappings in order
impl IntoIterator for Mappings {
    type Item = AstMap;
    type IntoIter = std::vec::IntoIter<AstMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.mappings.into_iter()
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct AstMap {
    pub name: String,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default)]
    pub excludes: Vec<String>,

    #[serde(default)]
    pub halt: bool,

    #[serde(default)]
    pub pattern: Pattern,
    pub ts_class: Option<TsClass>,
}

impl AstMap {
    pub fn new(name: String, aliases: Vec<String>, pattern: Pattern) -> Self {
        Self {
            name,
            aliases,
            pattern,
            ..Default::default()
        }
    }

    pub fn matches(&self, class_name: &String) -> bool {
        if self.excludes.contains(class_name) {
            return false;
        }

        match self.pattern {
            Pattern::Match => self.aliases.contains(class_name),
            Pattern::Contains => self.aliases.iter().any(|alias| alias.contains(class_name)),
            Pattern::StartsWith => self
                .aliases
                .iter()
                .any(|alias| class_name.starts_with(alias)),
            Pattern::EndsWith => self.aliases.iter().any(|alias| class_name.ends_with(alias)),
            Pattern::StartsOrEndsWith => self
                .aliases
                .iter()
                .any(|alias| class_name.starts_with(alias) || class_name.ends_with(alias)),
            Pattern::StartsAndEndsWith => self
                .aliases
                .iter()
                .any(|alias| class_name.starts_with(alias) && class_name.ends_with(alias)),
        }
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct TsClass {
    pub name: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub enum Pattern {
    #[default]
    Match,
    Contains,
    StartsWith,
    EndsWith,
    StartsOrEndsWith,
    StartsAndEndsWith,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        let node = AstMap::new(
            "function".to_string(),
            vec!["Function".to_string(), "Call".to_string()],
            Pattern::Match,
        );
        assert!(node.matches(&"Function".to_string()));
        assert!(node.matches(&"Call".to_string()));
        assert!(!node.matches(&"FunctionCall".to_string()));
    }
    #[test]
    fn test_startswith() {
        let node = AstMap::new(
            "function".to_string(),
            vec!["Function".to_string(), "Call".to_string()],
            Pattern::StartsWith,
        );
        assert!(node.matches(&"Function".to_string()));
        assert!(node.matches(&"FunctionCall".to_string()));
        assert!(node.matches(&"CallDef".to_string()));
    }
}
