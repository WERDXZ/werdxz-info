use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;
use std::process::Command;

/// KV namespace ID (shared with API)
const KV_NAMESPACE_ID: &str = "ad9607c404424a8eb6949994a4383845";

/// Get a value from KV
pub fn kv_get<T: DeserializeOwned>(
    workspace_root: &Path,
    key: &str,
    remote: bool,
) -> Result<Option<T>> {
    let mut cmd = Command::new("npx");
    cmd.arg("wrangler")
        .arg("kv")
        .arg("key")
        .arg("get")
        .arg(key);

    if remote {
        cmd.arg("--remote");
    }

    cmd.arg("--namespace-id")
        .arg(KV_NAMESPACE_ID)
        .current_dir(workspace_root.join("portfolio"));

    let output = cmd.output().context("Failed to run wrangler kv get")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not found") || stderr.contains("No such key") {
            return Ok(None);
        }
        anyhow::bail!("Failed to get from KV: {}", stderr);
    }

    if output.stdout.is_empty() {
        return Ok(None);
    }

    let value: T = serde_json::from_slice(&output.stdout)
        .context("Failed to parse JSON from KV")?;

    Ok(Some(value))
}

/// Put a value into KV
pub fn kv_put<T: Serialize>(
    workspace_root: &Path,
    key: &str,
    value: &T,
    remote: bool,
) -> Result<()> {
    // Serialize to temp file
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("kv-{}.json", uuid::Uuid::new_v4()));

    let json = serde_json::to_string_pretty(value)
        .context("Failed to serialize value")?;

    std::fs::write(&temp_file, json)
        .context("Failed to write temp file")?;

    let mut cmd = Command::new("npx");
    cmd.arg("wrangler")
        .arg("kv")
        .arg("key")
        .arg("put")
        .arg(key);

    if remote {
        cmd.arg("--remote");
    }

    cmd.arg("--namespace-id")
        .arg(KV_NAMESPACE_ID)
        .arg("--path")
        .arg(&temp_file)
        .current_dir(workspace_root.join("portfolio"));

    let status = cmd.status().context("Failed to run wrangler kv put")?;

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);

    if !status.success() {
        anyhow::bail!("Failed to put to KV");
    }

    Ok(())
}

/// Delete a value from KV
pub fn kv_delete(workspace_root: &Path, key: &str, remote: bool) -> Result<()> {
    let mut cmd = Command::new("npx");
    cmd.arg("wrangler")
        .arg("kv")
        .arg("key")
        .arg("delete")
        .arg(key);

    if remote {
        cmd.arg("--remote");
    }

    cmd.arg("--namespace-id")
        .arg(KV_NAMESPACE_ID)
        .current_dir(workspace_root.join("portfolio"));

    let status = cmd.status().context("Failed to run wrangler kv delete")?;

    if !status.success() {
        anyhow::bail!("Failed to delete from KV");
    }

    Ok(())
}

/// List keys with a prefix
pub fn kv_list_keys(
    workspace_root: &Path,
    prefix: &str,
    remote: bool,
) -> Result<Vec<String>> {
    let mut cmd = Command::new("npx");
    cmd.arg("wrangler")
        .arg("kv")
        .arg("key")
        .arg("list");

    if remote {
        cmd.arg("--remote");
    }

    cmd.arg("--namespace-id")
        .arg(KV_NAMESPACE_ID)
        .arg("--prefix")
        .arg(prefix)
        .current_dir(workspace_root.join("portfolio"));

    let output = cmd.output().context("Failed to run wrangler kv list")?;

    if !output.status.success() {
        anyhow::bail!("Failed to list keys from KV");
    }

    // Parse JSON output
    let keys: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)
        .context("Failed to parse KV list output")?;

    let key_names: Vec<String> = keys
        .into_iter()
        .filter_map(|v: serde_json::Value| {
            v.get("name")
                .and_then(|n: &serde_json::Value| n.as_str())
                .map(String::from)
        })
        .collect();

    Ok(key_names)
}
