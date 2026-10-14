//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! walkdir = "2"
//! anyhow = "1.0"
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct SourceEntry {
    id: String,
    resource: String,
    title: Option<String>,
    author: Option<String>,
    last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_window: Option<UsageWindow>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UsageWindow {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Frontmatter {
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    extra: HashMap<String, serde_yaml::Value>,
    sources: Option<Vec<SourceEntry>>,
}

fn main() -> Result<()> {
    let knowledge_root = PathBuf::from("/home/koppe/knowledge");
    let references_root = PathBuf::from("/home/koppe/references");

    let mut total_links = 0;
    let mut valid_links = 0;
    let mut broken_links = 0;
    let mut errors = Vec::new();

    for entry in WalkDir::new(&knowledge_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let path = entry.path();
        let rel_path = path.strip_prefix(&knowledge_root)?;

        // Skip index.md and log.md at root
        if rel_path == Path::new("index.md") || rel_path == Path::new("log.md") {
            continue;
        }

        let content =
            fs::read_to_string(path).with_context(|| format!("Reading {}", path.display()))?;

        let (frontmatter_str, _) = extract_frontmatter(&content)?;
        let fm: Frontmatter = serde_yaml::from_str(&frontmatter_str)
            .with_context(|| format!("Parsing frontmatter in {}", path.display()))?;

        if let Some(sources) = fm.sources {
            for source in sources {
                if source.resource.starts_with("file://") {
                    total_links += 1;
                    match check_file_url(&source.resource, &references_root) {
                        Ok(true) => {
                            valid_links += 1;
                        }
                        Ok(false) => {
                            broken_links += 1;
                            errors.push(format!(
                                "❌ {}: {} -> {}",
                                rel_path.display(),
                                source.id,
                                source.resource
                            ));
                        }
                        Err(e) => {
                            broken_links += 1;
                            errors.push(format!(
                                "⚠️  {}: {} -> {} ({})",
                                rel_path.display(),
                                source.id,
                                source.resource,
                                e
                            ));
                        }
                    }
                }
            }
        }
    }

    println!("\n=== Link Check Summary ===");
    println!("Total file:// links: {}", total_links);
    println!("✅ Valid: {}", valid_links);
    println!("❌ Broken: {}", broken_links);

    if !errors.is_empty() {
        println!("\nBroken links:");
        for err in &errors {
            println!("  {}", err);
        }
        std::process::exit(1);
    }

    Ok(())
}

fn extract_frontmatter(content: &str) -> Result<(String, String)> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() || lines[0] != "---" {
        return Ok((String::new(), content.to_string()));
    }
    let end_idx = lines[1..]
        .iter()
        .position(|l| *l == "---")
        .context("No closing --- for frontmatter")?
        + 1;
    let frontmatter = lines[1..end_idx].join("\n");
    let body = lines[end_idx + 1..].join("\n");
    Ok((frontmatter, body))
}

fn check_file_url(resource: &str, references_root: &Path) -> Result<bool> {
    let path_str = resource
        .strip_prefix("file://")
        .context("Not a file:// URL")?;

    let path = PathBuf::from(path_str);
    let absolute_path = if path.is_absolute() {
        path
    } else {
        references_root.join(path)
    };

    // Check if path exists (file or directory)
    Ok(absolute_path.exists())
}
