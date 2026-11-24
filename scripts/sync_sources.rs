//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! walkdir = "2"
//! chrono = { version = "0.4", features = ["serde"] }
//! anyhow = "1.0"
//! ```

use anyhow::{Context, Result};
// use chrono::{DateTime, FixedOffset, Utc}; // reserved for future
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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
    title: Option<String>,
    description: Option<String>,
    resource: Option<String>,
    tags: Option<Vec<String>>,
    status: Option<String>,
    generated: Option<Generated>,
    verified: Option<serde_yaml::Value>,
    stale_after: Option<String>,
    sources: Option<Vec<SourceEntry>>,
    // Allow extra fields
    #[serde(flatten)]
    extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Generated {
    by: String,
    at: String,
}

fn main() -> Result<()> {
    // Roots come from CLI arguments or environment variables; no host-specific
    // absolute path is embedded in the script.
    let mut args = std::env::args_os().skip(1);
    let knowledge_root = args
        .next()
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("KNOWLEDGE_ROOT").map(PathBuf::from))
        .context("set KNOWLEDGE_ROOT or pass the knowledge root as the first argument")?;
    let references_root = args
        .next()
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("REFERENCES_ROOT").map(PathBuf::from))
        .unwrap_or_else(|| {
            knowledge_root
                .parent()
                .map(|parent| parent.join("references"))
                .unwrap_or_else(|| knowledge_root.join("references"))
        });

    let mut scanned_files = 0;
    let mut updated_count = 0;
    let mut error_count = 0;

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
        scanned_files += 1;

        if let Err(e) = process_file(path, &references_root) {
            eprintln!("⚠️  {}: {}", rel_path.display(), e);
            error_count += 1;
        } else {
            updated_count += 1;
        }
    }

    // Fail closed: a missing or empty root must not look like a clean sync.
    if scanned_files == 0 {
        anyhow::bail!(
            "no Markdown files found under {}; check KNOWLEDGE_ROOT",
            knowledge_root.display()
        );
    }

    println!("✅ Synced {} files, {} errors", updated_count, error_count);
    Ok(())
}

fn process_file(file_path: &Path, references_root: &Path) -> Result<()> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Reading {}", file_path.display()))?;

    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(&content)?;
    let mut fm: Frontmatter = serde_yaml::from_str(&frontmatter)
        .with_context(|| format!("Parsing frontmatter in {}", file_path.display()))?;

    let mut modified = false;

    if let Some(sources) = &mut fm.sources {
        for source in sources.iter_mut() {
            if let Some(new_modified) = fetch_git_last_modified(&source.resource, references_root)?
            {
                if source.last_modified.as_deref() != Some(&new_modified) {
                    println!(
                        "  📝 {}: {} -> {}",
                        source.id,
                        source.last_modified.as_deref().unwrap_or("null"),
                        new_modified
                    );
                    source.last_modified = Some(new_modified);
                    modified = true;
                }
            }
            // Optionally update usage_count from git log --oneline count
            // Only update usage_count for local file:// sources, preserve upstream estimates
            if source.resource.starts_with("file://") {
                if let Some(count) = fetch_git_usage_count(&source.resource, references_root)? {
                    if source.usage_count != Some(count) {
                        source.usage_count = Some(count);
                        modified = true;
                    }
                }
            }
        }
    }

    if modified {
        let new_frontmatter = serde_yaml::to_string(&fm)?;
        let new_content = format!("---\n{}---{}", new_frontmatter, body);
        fs::write(file_path, new_content)
            .with_context(|| format!("Writing {}", file_path.display()))?;
        println!("✅ Updated: {}", file_path.display());
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

fn fetch_git_last_modified(resource: &str, references_root: &Path) -> Result<Option<String>> {
    let path = resolve_file_url(resource, references_root)?;
    if !path.exists() {
        return Ok(None);
    }

    // Get the git repo root
    let repo_root = find_git_root(&path)?;
    let rel_path = path.strip_prefix(&repo_root).unwrap_or(&path);

    let output = Command::new("git")
        .arg("-C")
        .arg(&repo_root)
        .arg("log")
        .arg("-1")
        .arg("--format=%cI")
        .arg("--")
        .arg(rel_path)
        .output()?;

    if output.status.success() {
        let timestamp = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !timestamp.is_empty() {
            return Ok(Some(timestamp));
        }
    }
    Ok(None)
}

fn fetch_git_usage_count(resource: &str, references_root: &Path) -> Result<Option<u64>> {
    let path = resolve_file_url(resource, references_root)?;
    if !path.exists() {
        return Ok(None);
    }

    let repo_root = find_git_root(&path)?;
    let rel_path = path.strip_prefix(&repo_root).unwrap_or(&path);

    let output = Command::new("git")
        .arg("-C")
        .arg(&repo_root)
        .arg("log")
        .arg("--oneline")
        .arg("--")
        .arg(rel_path)
        .output()?;

    if output.status.success() {
        let count = String::from_utf8_lossy(&output.stdout).lines().count() as u64;
        if count > 0 {
            return Ok(Some(count));
        }
    }
    Ok(None)
}

fn resolve_file_url(resource: &str, references_root: &Path) -> Result<PathBuf> {
    if let Some(path_str) = resource.strip_prefix("file://") {
        let path = PathBuf::from(path_str);
        if path.is_absolute() {
            Ok(path)
        } else {
            Ok(references_root.join(path))
        }
    } else {
        Ok(references_root.join(resource))
    }
}

fn find_git_root(path: &Path) -> Result<PathBuf> {
    let mut current = path.to_path_buf();
    if current.is_file() {
        current = current
            .parent()
            .context("No parent directory")?
            .to_path_buf();
    }
    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }
    anyhow::bail!("No .git directory found for {}", path.display())
}
