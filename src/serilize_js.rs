use std::{collections::HashMap, fs, path::Path};

use crate::datatypes::{DirectoryNode, ScanConfig};

pub fn scan_directory(
    path: &Path,
    current_depth: usize,
    max_depth: usize,
    config: &ScanConfig,
) -> Result<DirectoryNode, Box<dyn std::error::Error>> {
    let name = path
        .file_name()
        .unwrap_or_else(|| path.as_os_str())
        .to_string_lossy()
        .into_owned();

    let mut node = DirectoryNode {
        name: name.clone(),
        // path: path.to_string_lossy().into_owned(),
        files: Vec::new(),
        subdirectories: HashMap::new(),
    };

    if current_depth >= max_depth {
        return Ok(node);
    }

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name().to_string_lossy().into_owned();

        // Skip hidden files/directories if configured to do so
        if !config.include_hidden.unwrap_or(false) && entry_name.starts_with('.') {
            continue;
        }

        // Skip excluded directories
        if let Some(excluded_dirs) = &config.exclude_dirs {
            if excluded_dirs.contains(&entry_name) {
                continue;
            }
        }

        if entry_path.is_dir() {
            let subdir = scan_directory(&entry_path, current_depth + 1, max_depth, config)?;
            node.subdirectories.insert(entry_name, subdir);
        } else {
            // Skip files with excluded extensions
            if let Some(excluded_exts) = &config.exclude_extensions {
                if let Some(ext) = entry_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if excluded_exts.iter().any(|e| e.to_lowercase() == ext_str) {
                        continue;
                    }
                }
            }
            node.files.push(entry_name);
        }
    }

    Ok(node)
}
