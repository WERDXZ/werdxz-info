use anyhow::{Context, Result};
use std::path::Path;

use super::kv::{kv_delete, kv_get, kv_put};
use super::types::{Project, ProjectLink};
use crate::{status, PortfolioProjectCommands};

const FEATURED_PROJECTS_KEY: &str = "portfolio:featured_projects";

fn project_key(id: &str) -> String {
    format!("portfolio:project:{}", id)
}

pub fn handle_command(workspace_root: &Path, command: PortfolioProjectCommands) -> Result<()> {
    match command {
        PortfolioProjectCommands::Add {
            id,
            title,
            description,
            technologies,
            image_url,
            redirect_url,
            links,
            remote,
        } => add_project(
            workspace_root,
            &id,
            title,
            description,
            technologies,
            image_url,
            redirect_url,
            links,
            remote,
        ),
        PortfolioProjectCommands::List { remote } => list_projects(workspace_root, remote),
        PortfolioProjectCommands::Remove { id, remote } => {
            remove_project(workspace_root, &id, remote)
        }
    }
}

fn add_project(
    workspace_root: &Path,
    id: &str,
    title: String,
    description: String,
    technologies: String,
    image_url: String,
    redirect_url: Option<String>,
    link_specs: Vec<String>,
    remote: bool,
) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Adding", "project '{}' ({})", title, mode);

    // Validate ID
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("Project ID can only contain alphanumeric characters, hyphens, and underscores");
    }

    // Parse technologies
    let tech_list: Vec<String> = technologies
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Parse links
    let mut links = Vec::new();
    for link_spec in link_specs {
        let parts: Vec<&str> = link_spec.splitn(2, ':').collect();
        if parts.len() != 2 {
            anyhow::bail!(
                "Invalid link format '{}' (expected 'label:url')",
                link_spec
            );
        }
        links.push(ProjectLink::new(
            parts[0].trim().to_string(),
            parts[1].trim().to_string(),
        ));
    }

    // Create project
    let project = Project::new(
        title,
        description,
        tech_list,
        image_url,
        redirect_url,
        links,
    );

    // Store project in KV
    kv_put(workspace_root, &project_key(id), &project, remote)
        .context("Failed to store project in KV")?;

    // Update featured projects index
    let mut featured_ids: Vec<String> =
        kv_get(workspace_root, FEATURED_PROJECTS_KEY, remote)?.unwrap_or_default();

    if !featured_ids.contains(&id.to_string()) {
        featured_ids.push(id.to_string());
        kv_put(workspace_root, FEATURED_PROJECTS_KEY, &featured_ids, remote)
            .context("Failed to update featured projects index")?;
    }

    status!("Finished", "adding project '{}'", id);
    println!("          id: {}", id);
    println!("       count: {} featured projects", featured_ids.len());

    Ok(())
}

fn list_projects(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Listing", "featured projects ({})", mode);

    // Get featured projects index
    let featured_ids: Vec<String> =
        kv_get(workspace_root, FEATURED_PROJECTS_KEY, remote)?.unwrap_or_default();

    if featured_ids.is_empty() {
        println!("No featured projects found");
        return Ok(());
    }

    println!("\nFound {} featured project(s):\n", featured_ids.len());

    // Fetch and display each project
    for id in &featured_ids {
        let project: Option<Project> = kv_get(workspace_root, &project_key(id), remote)?;

        match project {
            Some(p) => {
                println!("  [{}]", id);
                println!("    Title: {}", p.title);
                println!("    Technologies: {}", p.technologies.join(", "));
                println!("    Image: {}", p.image_url);
                if let Some(url) = p.redirect_url {
                    println!("    Redirect: {}", url);
                }
                println!("    Links: {}", p.links.len());
                for link in &p.links {
                    println!("      - {}: {}", link.label, link.url);
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

fn remove_project(workspace_root: &Path, id: &str, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Removing", "project '{}' ({})", id, mode);

    // Remove from featured index
    let mut featured_ids: Vec<String> =
        kv_get(workspace_root, FEATURED_PROJECTS_KEY, remote)?.unwrap_or_default();

    featured_ids.retain(|featured_id| featured_id != id);

    kv_put(workspace_root, FEATURED_PROJECTS_KEY, &featured_ids, remote)
        .context("Failed to update featured projects index")?;

    // Delete project data
    kv_delete(workspace_root, &project_key(id), remote)
        .context("Failed to delete project from KV")?;

    status!("Finished", "removing project '{}'", id);
    println!("       count: {} featured projects", featured_ids.len());

    Ok(())
}
