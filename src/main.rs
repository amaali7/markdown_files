mod datatypes;
mod deserilize_js;
mod serilize_js;
use datatypes::{Config, ScanConfig};
use serilize_js::scan_directory;
use std::{fs, path::Path};
use toml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = if !Path::new("libjson.toml").exists() {
        println!("libjson.toml not found. Creating default configuration...");
        let default_config = Config {
            scan: ScanConfig {
                root_dir: ".".to_string(), // Current directory
                json_index: "index.json".to_string(),
                include_hidden: Some(false),
                max_depth: Some(3),
                exclude_dirs: Some(vec!["target".to_string(), "node_modules".to_string()]),
                exclude_extensions: Some(vec!["tmp".to_string(), "bak".to_string()]),
            },
        };

        let toml = toml::to_string_pretty(&default_config)?;
        fs::write("libjson.toml", toml)?;
        println!("Default libjson.toml created. Please modify it and rerun the program.");
        return Ok(());
    } else {
        // Read and parse existing config
        let config_content = fs::read_to_string("libjson.toml")?;
        toml::from_str(&config_content)?
    };

    // Verify root directory exists
    if !Path::new(&config.scan.root_dir).exists() {
        return Err(format!("Root directory '{}' does not exist", config.scan.root_dir).into());
    }

    let root_path = Path::new(&config.scan.root_dir);
    let dir_tree = scan_directory(
        root_path,
        0,
        config.scan.max_depth.unwrap_or(usize::MAX),
        &config.scan,
    )?;

    // Convert the directory tree to JSON and write to file
    let json_output = serde_json::to_string_pretty(&dir_tree)?;
    fs::write(config.scan.json_index, json_output)?;

    println!("Directory structure successfully written to index.json");
    Ok(())
}
