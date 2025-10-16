use std::{collections::HashMap, fs, path::Path};

use crate::datatypes::{Node, Output, WebConfig};

fn extract_metadata(content: &str) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            break; // Stop at first empty line (end of front matter)
        }
        if let Some((key, value)) = line.split_once(':') {
            metadata.insert(
                key.replace("<!--", "").trim().to_string(),
                value.replace("-->", "").trim().to_string(),
            );
        }
    }

    metadata
}

fn scan_directory(dir: &Path, base_path: &str) -> Option<Vec<Node>> {
    let mut children = Vec::new();

    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();

        // Skip hidden files
        if path.file_name()?.to_str()?.starts_with('.') {
            continue;
        }

        // Process name (convert to title case)
        let name = path
            .file_stem()?
            .to_str()?
            .to_string()
            .replace(['_', '-'], " ");
        let name = uppercase_first_letter(&name);

        if path.is_dir() {
            let dir_name = path.file_name()?.to_str()?.to_lowercase();
            let child_path = if base_path.is_empty() {
                format!("/{}", dir_name)
            } else {
                format!("{}/{}", base_path, dir_name)
            };

            let mut child_nodes = scan_directory(&path, &child_path);

            // Check for matching md file (e.g. home.md in home directory)
            let matching_md = path.join(format!("{}.md", dir_name));
            if matching_md.exists() {
                let content = fs::read_to_string(&matching_md).ok()?;
                let metadata = extract_metadata(&content);

                // Add the page node with directory path
                let page_node = Node {
                    name: name.clone(),
                    path: child_path.clone(), // Use directory path
                    node_type: "page".to_string(),
                    file: Some(format!("{}.md", dir_name)),
                    nav_order: metadata.get("nav_order").and_then(|s| s.parse().ok()),
                    nav_title: metadata.get("nav_title").cloned(),
                    date: metadata.get("date").cloned(),
                    hidden: metadata.get("hidden").map(|s| s == "true"),
                    children: None,
                };

                // Insert at beginning to maintain order
                let _ = child_nodes
                    .as_mut()
                    .map_or((), |nodes| nodes.insert(0, page_node));
            }

            children.push(Node {
                name,
                path: child_path,
                node_type: "directory".to_string(),
                file: None,
                nav_order: None,
                nav_title: None,
                date: None,
                hidden: None,
                children: child_nodes,
            });
        } else if path.extension()?.to_str()? == "md" {
            let file_name = path.file_name()?.to_str()?.to_string();
            let file_stem = path.file_stem()?.to_str()?.to_lowercase();
            let parent_dir = path.parent()?.file_name()?.to_str()?;

            // Skip if this is a same-name file (handled in directory case)
            if parent_dir == file_stem {
                continue;
            }

            let node_path = if base_path.is_empty() {
                format!("/{}", file_stem)
            } else {
                format!("{}/{}", base_path, file_stem)
            };

            let content = fs::read_to_string(&path).ok()?;
            let metadata = extract_metadata(&content);

            children.push(Node {
                name,
                path: node_path,
                node_type: "page".to_string(),
                file: Some(file_name),
                nav_order: metadata.get("nav_order").and_then(|s| s.parse().ok()),
                nav_title: metadata.get("nav_title").cloned(),
                date: metadata.get("date").cloned(),
                hidden: metadata.get("hidden").map(|s| s == "true"),
                children: None,
            });
        }
    }

    // Sort children by nav_order if present
    children.sort_by(|a, b| {
        a.nav_order
            .unwrap_or(u32::MAX)
            .cmp(&b.nav_order.unwrap_or(u32::MAX))
    });
    Some(children)
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn generate_json_output(markdown_dir: &str) -> Output {
    let root_node = Node {
        name: "Blogy".to_string(),
        path: "/".to_string(),
        node_type: "directory".to_string(),
        file: None,
        nav_order: None,
        nav_title: None,
        date: None,
        hidden: None,
        children: scan_directory(Path::new(markdown_dir), ""),
    };

    let config = WebConfig {
        default_page: "/pages/home".to_string(),
        markdown_dir: markdown_dir.to_string(),
        build_dir: "public".to_string(),
        nav_include_hidden: false,
    };

    Output {
        root: root_node,
        config,
    }
}
