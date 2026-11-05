use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

pub mod mappings;
pub mod tsgrammar;

use codeql_extractor::node_types::{self, Entry, TypeName};
use std::collections::BTreeMap;

use crate::config::{Config, Repository};
use tsgrammar::TreeSitterGrammar;

pub const COMMENTS: [&str; 2] = ["Comment", "CommentBlock"];
pub const LITERALS: [&str; 12] = [
    "Boolean", "Bool", "True", "False", // Booleans
    "String", "Char", // Strings
    "Null", "Nil", // Nulls
    "Number", "Integer", "Int", "Float", // Numbers
];
pub const LOOPS: [&str; 3] = ["While", "For", "DoWhile"];

#[derive(Debug, Default, serde::Serialize)]
pub struct TreeSitterInfo {
    pub name: String,
    /// `extractor/tree-sitter`
    pub path: PathBuf,
    // Repo stuff
    pub repository: Repository,

    pub grammar: TreeSitterGrammar,
    #[serde(skip)]
    pub node_types: BTreeMap<TypeName, Entry>,
    #[serde(skip)]
    pub mappings: Vec<mappings::AstMap>,

    /// List of all AST nodes (literals, statements, expressions, loops, misc)
    pub nodes: HashMap<String, Vec<String>>,
    // /// Misc unknown node types
    // pub misc: Vec<String>,
    // /// Comment Block
    // pub comment: String,
    // /// Blocks is a sequence of statements (function body, if block, etc)
    // pub block_node: String,
    // /// List of literal node types
    // pub literals: Vec<String>,
    // pub liberal_abstractions: Vec<String>,
    // pub statements: Vec<String>,
    // pub expressions: Vec<String>,
    // pub loops: Vec<String>,
}

impl TreeSitterInfo {
    pub fn new(name: String, path: &PathBuf, repository: Repository) -> Self {
        Self {
            name,
            path: path.clone(),
            repository,
            ..Default::default()
        }
    }

    /// Load from root of the TreeSitter repo
    pub async fn load(config: &Config, path: &PathBuf) -> Result<Self> {
        log::info!("Loading TreeSitter from `{}`", path.display());

        // TODO: Check if the repository exists
        log::info!(
            "Checking if repository exists at `{}`",
            config.tree_sitter_repo.path.display()
        );
        if !config.tree_sitter_repo.path.exists() {
            config.tree_sitter_repo.clone_repo(config).await?;
        }

        let mut info = Self::new(
            config.language_name.clone(),
            path,
            config.tree_sitter_repo.clone(),
        );
        // info.update().await?;

        // // Load Mappings
        // let mappings_path = config.working_dir.join("extractor").join("mappings.yml");
        // if mappings_path.exists() {
        //     info.mappings = Mappings::load(&mappings_path)?;
        //     log::info!("Loaded mappings from `mappings.yml`");
        // } else {
        //     log::info!("No mappings found");
        //     info.mappings = Mappings::load(&PathBuf::from("./mapping.yml"))?;
        // }
        // TODO: Merge mappings
        info.mappings = config.mappings.clone();
        log::info!("Loaded mappings: {:?}", info.mappings.len());

        info.load_node_types()?;
        // info.load_ts_grammar(config)?;

        Ok(info)
    }

    pub async fn update(&self) -> Result<()> {
        log::info!("Updating TreeSitter from `{}`", self.path.display());
        let status = tokio::process::Command::new("tree-sitter")
            .args(&["generate"])
            .current_dir(self.repository.path.as_os_str())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to update TreeSitter"));
        }
        Ok(())
    }

    fn load_node_types(&mut self) -> Result<()> {
        log::debug!("Finding NodeTypes from `{:?}`", self.path);

        let node_types_json = self.repository.find_file("node-types.json")?;
        log::info!("Found `node-types.json` file: {:?}", node_types_json);

        if !node_types_json.exists() {
            panic!("Node types file does not exist");
        }
        if self.mappings.is_empty() {
            log::error!("Mappings are empty, cannot load node types");
            return Err(anyhow::anyhow!(
                "Mappings are empty, cannot load node types"
            ));
        }

        let data = std::fs::read_to_string(node_types_json)?;

        let prefix = node_types::to_snake_case(self.name.as_str());

        let btree_nodes = node_types::read_node_types_str(&prefix, data.as_str())
            .map_err(|e| anyhow::anyhow!("Failed to read node types: {}", e))?;

        for (name, entry) in btree_nodes.iter() {
            let class_name = entry.ql_class_name.clone();
            let mut known = false;

            if class_name.as_str() == "ReservedWord" {
                log::debug!("Skipping node type: {:#?}", class_name);
                continue;
            }
            log::debug!("Found node type: {:#?}", class_name);

            self.nodes
                .entry("all".to_string())
                .or_insert_with(Vec::new)
                .push(class_name.clone());

            for astmap in self.mappings.clone().into_iter() {
                // Always insert the node type
                if self.nodes.get(&astmap.name).is_none() {
                    self.nodes.insert(astmap.name.clone(), vec![]);
                }

                if astmap.matches(&class_name) {
                    let mut node_entry = self
                        .nodes
                        .entry(astmap.name.clone())
                        .or_insert_with(Vec::new);
                    // Only insert if not already in the list
                    if !node_entry.contains(&class_name) {
                        node_entry.push(class_name.clone());
                    }
                    known = true;
                    if astmap.halt {
                        break;
                    }
                }
            }

            if !known {
                log::trace!("Unknown node type: {:#?}", class_name);
                let mut node_entry = self
                    .nodes
                    .entry("Misc".to_string())
                    .or_insert_with(Vec::new);
                // Only insert if not already in the list
                if !node_entry.contains(&class_name) {
                    node_entry.push(class_name.clone());
                }
            }

            // // OLD
            // if COMMENTS.contains(&class_name.as_str()) {
            //     self.comment = class_name.clone();
            //     known = true;
            // } else if LITERALS.contains(&class_name.as_str()) {
            //     self.literals.push(class_name.clone());
            //     known = true;
            // } else if class_name == "Block" || class_name == "Program" {
            //     self.block_node = class_name.clone();
            //     known = true;
            // }

            // // Anything can be an expression or a statement but not both
            // if class_name.ends_with("Expression")
            //     || class_name.starts_with("Expression")
            //     || class_name.starts_with("Expr")
            // {
            //     self.expressions.push(class_name);
            // } else if class_name.ends_with("Statement") || class_name.starts_with("Statement") {
            //     self.statements.push(class_name.clone());

            //     if class_name.starts_with("For")
            //         || class_name.starts_with("While")
            //         || class_name.starts_with("DoWhile")
            //         || class_name.starts_with("Do")
            //     {
            //         self.loops.push(class_name.clone());
            //     }
            // } else {
            //     if !known {
            //         self.misc.push(class_name.clone());
            //     }
            // }
        }

        // Display Nodes metics
        log::debug!("Nodes :: {:#?}", self.nodes);
        let mut all_count = 0;
        let mut misc_count = 0;

        for (name, map) in &self.nodes {
            if name == "all" {
                all_count = map.len();
            }
            if name == "Misc" {
                misc_count = map.len();
            }
            log::info!("{:<18} :: {:#?}", name, map.len());
        }

        // Check if Misc contains all nodes
        if misc_count == all_count {
            log::error!("Misc contains all nodes, this is likely an error");
            return Err(anyhow::anyhow!(
                "Misc contains all nodes, this is likely an error"
            ));
        }

        // Old stuff
        // self.comment = self
        //     .nodes
        //     .get("comment")
        //     .ok_or(anyhow::anyhow!(
        //         "Comment node type not found in mappings or node-types.json"
        //     ))?
        //     .first()
        //     .unwrap()
        //     .clone();
        // self.block_node = self.nodes.get("block").unwrap().first().unwrap().clone();

        // self.literals = self
        //     .nodes
        //     .get("literals")
        //     .expect("Literals node type not found")
        //     .to_vec();
        // self.liberal_abstractions = self
        //     .nodes
        //     .get("literalsabstract")
        //     .unwrap_or(&vec![])
        //     .to_vec();

        // self.statements = self
        //     .nodes
        //     .get("statements")
        //     .expect("Statement node type not found")
        //     .to_vec();
        // self.expressions = self
        //     .nodes
        //     .get("expressions")
        //     .expect("Expressions node type not found")
        //     .to_vec();
        // // Not all languages have loops
        // self.loops = self.nodes.get("loops").unwrap_or(&vec![]).to_vec();

        // self.misc = self.nodes.get("misc").unwrap_or(&vec![]).to_vec();

        self.sort();

        // Loop block check

        if let Some(loops) = self.nodes.get("loops") {
            if let Some(block_node) = self.nodes.get("block") {
                if block_node.is_empty() && !loops.is_empty() {
                    return Err(anyhow::anyhow!("Block node must be set if loops are used"));
                }
            } else {
                return Err(anyhow::anyhow!("Loops node type not found"));
            }
        }

        self.node_types = btree_nodes;

        Ok(())
    }

    // fn load_ts_grammar(&mut self, config: &InitConfig) -> Result<TreeSitterGrammar> {
    //     // Find the `grammar.json` file
    //     let grammar_json = self.repository.find_file("grammar.json")?;
    //     log::info!("Found `grammar.json` file: {:?}", grammar_json);

    //     let grammar = TreeSitterGrammar::load(grammar_json)?;

    //     for (name, rule) in &grammar.rules {
    //         if name.starts_with("_") {
    //             continue;
    //         }
    //         if grammar.inline.contains(name) {
    //             continue;
    //         }
    //         // TODO: is this correct?
    //         if let Some(content) = &rule.content {
    //             if content.member_type == "SYMBOL" {
    //                 continue;
    //             }
    //         }

    //         let class_name = dbscheme_name_to_class_name(name);

    //         // self.ast_nodes.push(class_name.clone());

    //         if LITERALS.contains(&class_name.as_str()) {
    //             self.literals.push(class_name);
    //         } else if class_name.ends_with("Expression") || class_name.starts_with("Expression") {
    //             self.expressions.push(class_name);
    //         } else if class_name.ends_with("Statement") || class_name.starts_with("Statement") {
    //             self.statements.push(class_name);
    //         }
    //     }
    //     self.sort();
    //     Ok(grammar)
    // }

    pub fn sort(&mut self) {
        for (_, nodes) in &mut self.nodes {
            nodes.sort();
        }
        // self.literals.sort();
        // self.statements.sort();
        // self.expressions.sort();
        // self.loops.sort();
    }
}
