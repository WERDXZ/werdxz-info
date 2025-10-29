use anyhow::{Context, Result};
use std::path::Path;

use super::kv::{kv_delete, kv_get, kv_put};
use super::types::Experience;
use crate::{status, PortfolioExperienceCommands};

const FEATURED_EXPERIENCE_KEY: &str = "portfolio:featured_experience";

fn experience_key(id: &str) -> String {
    format!("portfolio:experience:{}", id)
}

pub fn handle_command(
    workspace_root: &Path,
    command: PortfolioExperienceCommands,
) -> Result<()> {
    match command {
        PortfolioExperienceCommands::Add {
            id,
            company,
            role,
            period,
            location,
            description,
            technologies,
            redirect_url,
            remote,
        } => add_experience(
            workspace_root,
            &id,
            company,
            role,
            period,
            location,
            description,
            technologies,
            redirect_url,
            remote,
        ),
        PortfolioExperienceCommands::List { remote } => {
            list_experience(workspace_root, remote)
        }
        PortfolioExperienceCommands::Remove { id, remote } => {
            remove_experience(workspace_root, &id, remote)
        }
    }
}

fn add_experience(
    workspace_root: &Path,
    id: &str,
    company: String,
    role: String,
    period: String,
    location: Option<String>,
    description: String,
    technologies: String,
    redirect_url: Option<String>,
    remote: bool,
) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Adding", "experience at '{}' ({})", company, mode);

    // Validate ID
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("Experience ID can only contain alphanumeric characters, hyphens, and underscores");
    }

    // Parse technologies
    let tech_list: Vec<String> = technologies
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Create experience
    let experience = Experience::new(
        company,
        role,
        period,
        location,
        description,
        tech_list,
        redirect_url,
    );

    // Store experience in KV
    kv_put(workspace_root, &experience_key(id), &experience, remote)
        .context("Failed to store experience in KV")?;

    // Update featured experience index
    let mut featured_ids: Vec<String> =
        kv_get(workspace_root, FEATURED_EXPERIENCE_KEY, remote)?.unwrap_or_default();

    if !featured_ids.contains(&id.to_string()) {
        featured_ids.push(id.to_string());
        kv_put(
            workspace_root,
            FEATURED_EXPERIENCE_KEY,
            &featured_ids,
            remote,
        )
        .context("Failed to update featured experience index")?;
    }

    status!("Finished", "adding experience '{}'", id);
    println!("          id: {}", id);
    println!(
        "       count: {} featured experiences",
        featured_ids.len()
    );

    Ok(())
}

fn list_experience(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Listing", "featured experiences ({})", mode);

    // Get featured experience index
    let featured_ids: Vec<String> =
        kv_get(workspace_root, FEATURED_EXPERIENCE_KEY, remote)?.unwrap_or_default();

    if featured_ids.is_empty() {
        println!("No featured experiences found");
        return Ok(());
    }

    println!("\nFound {} featured experience(s):\n", featured_ids.len());

    // Fetch and display each experience
    for id in &featured_ids {
        let experience: Option<Experience> =
            kv_get(workspace_root, &experience_key(id), remote)?;

        match experience {
            Some(exp) => {
                println!("  [{}]", id);
                println!("    Company: {}", exp.company);
                println!("    Role: {}", exp.role);
                println!("    Period: {}", exp.period);
                if let Some(loc) = exp.location {
                    println!("    Location: {}", loc);
                }
                println!("    Technologies: {}", exp.technologies.join(", "));
                if let Some(url) = exp.redirect_url {
                    println!("    Redirect: {}", url);
                }
                println!();
            }
            None => {
                println!("  [{}] - NOT FOUND (stale index entry)", id);
                println!();
            }
        }
    }

    Ok(())
}

fn remove_experience(workspace_root: &Path, id: &str, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Removing", "experience '{}' ({})", id, mode);

    // Remove from featured index
    let mut featured_ids: Vec<String> =
        kv_get(workspace_root, FEATURED_EXPERIENCE_KEY, remote)?.unwrap_or_default();

    featured_ids.retain(|featured_id| featured_id != id);

    kv_put(
        workspace_root,
        FEATURED_EXPERIENCE_KEY,
        &featured_ids,
        remote,
    )
    .context("Failed to update featured experience index")?;

    // Delete experience data
    kv_delete(workspace_root, &experience_key(id), remote)
        .context("Failed to delete experience from KV")?;

    status!("Finished", "removing experience '{}'", id);
    println!(
        "       count: {} featured experiences",
        featured_ids.len()
    );

    Ok(())
}
