use anyhow::{Context, Result};
use std::path::Path;

use super::kv::{kv_get, kv_put};
use crate::{status, PortfolioPostCommands};

const FEATURED_POSTS_KEY: &str = "portfolio:featured_posts";

pub fn handle_command(workspace_root: &Path, command: PortfolioPostCommands) -> Result<()> {
    match command {
        PortfolioPostCommands::Add { slug, remote } => add_post(workspace_root, &slug, remote),
        PortfolioPostCommands::List { remote } => list_posts(workspace_root, remote),
        PortfolioPostCommands::Remove { slug, remote } => {
            remove_post(workspace_root, &slug, remote)
        }
    }
}

fn add_post(workspace_root: &Path, slug: &str, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Adding", "featured post '{}' ({})", slug, mode);

    // Validate slug format
    if !slug
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("Post slug can only contain alphanumeric characters, hyphens, and underscores");
    }

    // Update featured posts index
    let mut featured_slugs: Vec<String> =
        kv_get(workspace_root, FEATURED_POSTS_KEY, remote)?.unwrap_or_default();

    if featured_slugs.contains(&slug.to_string()) {
        println!("Post '{}' is already in featured posts", slug);
        return Ok(());
    }

    featured_slugs.push(slug.to_string());
    kv_put(workspace_root, FEATURED_POSTS_KEY, &featured_slugs, remote)
        .context("Failed to update featured posts index")?;

    status!("Finished", "adding featured post '{}'", slug);
    println!("       count: {} featured posts", featured_slugs.len());
    println!();
    println!("Note: The post must exist in the blog API for it to be displayed.");
    println!("      The portfolio site will fetch post details from /v1/posts/{}", slug);

    Ok(())
}

fn list_posts(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Listing", "featured posts ({})", mode);

    // Get featured posts index
    let featured_slugs: Vec<String> =
        kv_get(workspace_root, FEATURED_POSTS_KEY, remote)?.unwrap_or_default();

    if featured_slugs.is_empty() {
        println!("No featured posts found");
        return Ok(());
    }

    println!("\nFound {} featured post(s):\n", featured_slugs.len());

    for slug in &featured_slugs {
        println!("  - {}", slug);
    }

    println!();
    println!("These posts will be fetched from the blog API at runtime.");

    Ok(())
}

fn remove_post(workspace_root: &Path, slug: &str, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Removing", "featured post '{}' ({})", slug, mode);

    // Remove from featured index
    let mut featured_slugs: Vec<String> =
        kv_get(workspace_root, FEATURED_POSTS_KEY, remote)?.unwrap_or_default();

    let before_len = featured_slugs.len();
    featured_slugs.retain(|featured_slug| featured_slug != slug);

    if featured_slugs.len() == before_len {
        println!("Post '{}' was not in featured posts", slug);
        return Ok(());
    }

    kv_put(workspace_root, FEATURED_POSTS_KEY, &featured_slugs, remote)
        .context("Failed to update featured posts index")?;

    status!("Finished", "removing featured post '{}'", slug);
    println!("       count: {} featured posts", featured_slugs.len());

    Ok(())
}
