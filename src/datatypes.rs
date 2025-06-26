use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub default_page: String,
    pub markdown_dir: String,
    pub build_dir: String,
    pub nav_include_hidden: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub root: Node,
    pub config: WebConfig,
}
