use anyhow::{Context, Result};
use std::path::Path;

use super::kv::{kv_get, kv_list_keys};
use super::types::ContactSubmission;
use crate::{status, PortfolioContactCommands};

const CONTACT_PREFIX: &str = "portfolio:contact:";

pub fn handle_command(workspace_root: &Path, command: PortfolioContactCommands) -> Result<()> {
    match command {
        PortfolioContactCommands::List { remote } => list_contacts(workspace_root, remote),
    }
}

fn list_contacts(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Listing", "contact submissions ({})", mode);

    // List all keys with contact prefix
    let keys = kv_list_keys(workspace_root, CONTACT_PREFIX, remote)
        .context("Failed to list contact submissions")?;

    if keys.is_empty() {
        println!("No contact submissions found");
        return Ok(());
    }

    println!("\nFound {} contact submission(s):\n", keys.len());

    // Fetch and display each submission (in reverse chronological order)
    let mut submissions: Vec<(String, ContactSubmission)> = Vec::new();

    for key in &keys {
        let submission: Option<ContactSubmission> = kv_get(workspace_root, key, remote)?;

        if let Some(sub) = submission {
            submissions.push((key.clone(), sub));
        }
    }

    // Sort by timestamp (newest first)
    submissions.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));

    for (key, submission) in submissions {
        let timestamp_str = format_timestamp(submission.timestamp);

        println!("  [{}]", timestamp_str);
        println!("    Name: {}", submission.name);
        println!("    Email: {}", submission.email);
        println!("    Message:");

        // Print message with indentation, truncate if very long
        let message_lines: Vec<&str> = submission.message.lines().collect();
        let preview_lines = if message_lines.len() > 5 {
            &message_lines[..5]
        } else {
            &message_lines[..]
        };

        for line in preview_lines {
            println!("      {}", line);
        }

        if message_lines.len() > 5 {
            println!("      ... ({} more lines)", message_lines.len() - 5);
        }

        println!("    KV Key: {}", key);
        println!();
    }

    Ok(())
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let duration = Duration::from_secs(timestamp);
    let datetime = UNIX_EPOCH + duration;

    match datetime.duration_since(SystemTime::now()) {
        Ok(_) => "Future".to_string(), // Shouldn't happen
        Err(e) => {
            let elapsed = e.duration();
            let secs = elapsed.as_secs();

            if secs < 60 {
                format!("{} seconds ago", secs)
            } else if secs < 3600 {
                format!("{} minutes ago", secs / 60)
            } else if secs < 86400 {
                format!("{} hours ago", secs / 3600)
            } else {
                format!("{} days ago", secs / 86400)
            }
        }
    }
}
