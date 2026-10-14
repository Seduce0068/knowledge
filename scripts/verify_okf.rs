//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! walkdir = "2"
//! anyhow = "1.0"
//! chrono = { version = "0.4", features = ["serde"] }
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct Frontmatter {
    #[serde(rename = "type")]
    type_: String,
    title: Option<String>,
    description: Option<String>,
    resource: Option<String>,
    tags: Option<Vec<String>>,
    status: Option<String>,
    generated: Option<Generated>,
    verified: Option<serde_yaml::Value>,
    stale_after: Option<String>,
    sources: Option<Vec<serde_yaml::Value>>,
    #[serde(flatten)]
    extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Generated {
    by: String,
    at: String,
}

fn main() -> Result<()> {
    let knowledge_root = PathBuf::from("/home/koppe/knowledge");

    let mut total_files = 0;
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for entry in WalkDir::new(&knowledge_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let path = entry.path();
        let rel_path = path.strip_prefix(&knowledge_root)?;

        // Skip index.md and log.md at root (special files)
        if rel_path == Path::new("index.md") || rel_path == Path::new("log.md") {
            continue;
        }

        total_files += 1;
        let content =
            fs::read_to_string(path).with_context(|| format!("Reading {}", path.display()))?;

        let (frontmatter_str, _) = extract_frontmatter(&content)?;

        if frontmatter_str.trim().is_empty() {
            errors.push(format!("{} : Missing frontmatter", rel_path.display()));
            continue;
        }

        let fm: Frontmatter = serde_yaml::from_str(&frontmatter_str)
            .with_context(|| format!("Parsing frontmatter in {}", path.display()))?;

        // Required: type
        if fm.type_.trim().is_empty() {
            errors.push(format!(
                "{} : Missing required 'type' field",
                rel_path.display()
            ));
        }

        // Recommended fields
        if fm.title.is_none() {
            warnings.push(format!(
                "{} : Missing recommended 'title'",
                rel_path.display()
            ));
        }
        if fm.description.is_none() {
            warnings.push(format!(
                "{} : Missing recommended 'description'",
                rel_path.display()
            ));
        }
        if fm.tags.as_ref().map_or(true, |t| t.is_empty()) {
            warnings.push(format!(
                "{} : Missing recommended 'tags'",
                rel_path.display()
            ));
        }
        if fm.generated.is_none() {
            warnings.push(format!(
                "{} : Missing 'generated' (provenance)",
                rel_path.display()
            ));
        } else if let Some(gen) = &fm.generated {
            if gen.by.trim().is_empty() || gen.at.trim().is_empty() {
                errors.push(format!(
                    "{} : 'generated.by' and 'generated.at' required",
                    rel_path.display()
                ));
            }
            // Validate RFC 3339 timestamp
            if !is_rfc3339(&gen.at) {
                errors.push(format!(
                    "{} : 'generated.at' not RFC 3339: {}",
                    rel_path.display(),
                    gen.at
                ));
            }
        }
        if fm.stale_after.is_none() {
            warnings.push(format!(
                "{} : Missing 'stale_after' (freshness)",
                rel_path.display()
            ));
        } else if let Some(sa) = &fm.stale_after {
            if !is_rfc3339(sa) {
                errors.push(format!(
                    "{} : 'stale_after' not RFC 3339: {}",
                    rel_path.display(),
                    sa
                ));
            }
        }
        if fm.verified.is_none() {
            warnings.push(format!(
                "{} : Missing 'verified' (trust tier unverified)",
                rel_path.display()
            ));
        }

        // Sources validation
        if let Some(sources) = &fm.sources {
            for (idx, source) in sources.iter().enumerate() {
                let source_map = source.as_mapping().context("Source must be a mapping")?;

                let has_id = source_map.iter().any(|(k, _)| k.as_str() == Some("id"));
                let has_resource = source_map
                    .iter()
                    .any(|(k, _)| k.as_str() == Some("resource"));

                if !has_resource {
                    errors.push(format!(
                        "{} : sources[{}].resource required",
                        rel_path.display(),
                        idx
                    ));
                }
                if !has_id && source_map.len() > 1 {
                    warnings.push(format!(
                        "{} : sources[{}].id recommended for attribution",
                        rel_path.display(),
                        idx
                    ));
                }

                // Validate file:// paths exist
                if let Some(resource_val) = source_map.iter().find_map(|(k, v)| {
                    if k.as_str() == Some("resource") {
                        v.as_str()
                    } else {
                        None
                    }
                }) {
                    if resource_val.starts_with("file://") {
                        let path_str = resource_val.strip_prefix("file://").unwrap();
                        let check_path = PathBuf::from(path_str);
                        if !check_path.exists() {
                            warnings.push(format!(
                                "{} : sources[{}].resource '{}' not found locally",
                                rel_path.display(),
                                idx,
                                resource_val
                            ));
                        }
                    }
                }
            }
        }

        // Status validation
        if let Some(status) = &fm.status {
            if !matches!(status.as_str(), "draft" | "stable" | "deprecated") {
                errors.push(format!(
                    "{} : Invalid status '{}' (must be draft|stable|deprecated)",
                    rel_path.display(),
                    status
                ));
            }
        }
    }

    println!("=== OKF Conformance Check ===");
    println!("Files checked: {}", total_files);
    println!("Errors: {}", errors.len());
    println!("Warnings: {}", warnings.len());

    if !warnings.is_empty() {
        println!("\nWarnings:");
        for w in &warnings {
            println!("  ⚠️  {}", w);
        }
    }

    if !errors.is_empty() {
        println!("\nErrors:");
        for e in &errors {
            println!("  ❌ {}", e);
        }
        std::process::exit(1);
    }

    println!("\n✅ All OKF conformance checks passed");
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

fn is_rfc3339(s: &str) -> bool {
    // Basic RFC 3339 validation - must have timezone offset
    chrono::DateTime::parse_from_rfc3339(s).is_ok()
}
