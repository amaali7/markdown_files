use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub scan: ScanConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScanConfig {
    pub root_dir: String,
    pub json_index: String,
    pub include_hidden: Option<bool>,
    pub max_depth: Option<usize>,
    pub exclude_dirs: Option<Vec<String>>,
    pub exclude_extensions: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DirectoryNode {
    pub name: String,
    // path: String,
    pub files: Vec<String>,
    pub subdirectories: HashMap<String, DirectoryNode>,
}
