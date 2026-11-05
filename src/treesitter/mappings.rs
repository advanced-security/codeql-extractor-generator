use serde::{Deserialize, Serialize};

/// AST Node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AstMap {
    /// Name of the AST node
    pub name: String,
    /// Parent AST node name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Description of the AST node
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Aliases for the AST node
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Excludes for the AST node
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excludes: Vec<String>,
    /// Halt flag to stop processing
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub halt: bool,
    /// Pattern to match the AST node
    #[serde(default)]
    pub pattern: Pattern,
    /// TS class
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
