use anyhow::Result;
use std::path::Path;

use crate::{
    PortfolioContactCommands, PortfolioExperienceCommands, PortfolioPostCommands,
    PortfolioProjectCommands,
};

mod contact;
mod experience;
mod kv;
mod post;
mod project;
mod types;

pub use types::{ContactSubmission, Experience, Project, ProjectLink};

/// Handle portfolio project commands
pub fn handle_project_command(
    workspace_root: &Path,
    command: PortfolioProjectCommands,
) -> Result<()> {
    project::handle_command(workspace_root, command)
}

/// Handle portfolio experience commands
pub fn handle_experience_command(
    workspace_root: &Path,
    command: PortfolioExperienceCommands,
) -> Result<()> {
    experience::handle_command(workspace_root, command)
}

/// Handle portfolio post commands
pub fn handle_post_command(workspace_root: &Path, command: PortfolioPostCommands) -> Result<()> {
    post::handle_command(workspace_root, command)
}

/// Handle portfolio contact commands
pub fn handle_contact_command(
    workspace_root: &Path,
    command: PortfolioContactCommands,
) -> Result<()> {
    contact::handle_command(workspace_root, command)
}
