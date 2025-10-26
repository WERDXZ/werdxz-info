use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

// Structs for parsing wrangler.toml
#[derive(Debug, Deserialize)]
struct WranglerConfig {
    d1_databases: Option<Vec<D1Database>>,
    r2_buckets: Option<Vec<R2Bucket>>,
    kv_namespaces: Option<Vec<KvNamespace>>,
}

#[derive(Debug, Deserialize)]
struct D1Database {
    binding: String,
    database_name: String,
    #[allow(dead_code)]
    database_id: String,
}

#[derive(Debug, Deserialize)]
struct R2Bucket {
    binding: String,
    bucket_name: String,
}

#[derive(Debug, Deserialize)]
struct KvNamespace {
    binding: String,
    id: String,
}

macro_rules! status {
    ($action:expr, $msg:expr) => {
        println!("{}{:>12}{} {}", GREEN, $action, RESET, $msg)
    };
    ($action:expr, $msg:expr, $($arg:tt)*) => {
        println!("{}{:>12}{} {}", GREEN, $action, RESET, format!($msg, $($arg)*))
    };
}

#[derive(Parser)]
#[command(name = "cargo xtask", bin_name = "cargo xtask")]
#[command(about = "Admin CLI for werdxz-info project", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Project management commands
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },

    /// Blog post management commands
    Post {
        #[command(subcommand)]
        command: PostCommands,
    },

    /// Resume management commands
    Resume {
        #[command(subcommand)]
        command: ResumeCommands,
    },

    /// Apply database migrations
    Migrate {
        /// Apply to remote database (default is local)
        #[arg(long)]
        remote: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// List all projects in workspace
    List,

    /// Deploy a project to Cloudflare Workers
    Deploy {
        /// Project name (auto-discovered from workspace)
        project: String,

        /// Deploy to production (default is preview)
        #[arg(long)]
        production: bool,
    },
}

#[derive(Subcommand)]
enum PostCommands {
    /// Publish a blog post
    Publish {
        /// Path to markdown file
        file: String,

        /// URL slug for the post
        #[arg(long)]
        slug: String,

        /// Post title
        #[arg(long)]
        title: String,

        /// Post summary
        #[arg(long)]
        summary: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// External URL (for cross-posted content)
        #[arg(long)]
        external_url: Option<String>,

        /// Publish to remote (default is local)
        #[arg(long)]
        remote: bool,
    },

    /// List all published posts
    List {
        /// List from remote database (default is local)
        #[arg(long)]
        remote: bool,
    },

    /// Delete a blog post
    Delete {
        /// Post slug to delete
        slug: String,

        /// Delete from remote (default is local)
        #[arg(long)]
        remote: bool,
    },
}

#[derive(Subcommand)]
enum ResumeCommands {
    /// Update resume data in KV from cloud.werdxz.info
    Update {
        /// Update remote KV (default is local)
        #[arg(long)]
        remote: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let workspace_root = find_workspace_root()?;

    match cli.command {
        Commands::Project { command } => match command {
            ProjectCommands::List => list_projects(&workspace_root),
            ProjectCommands::Deploy { project, production } => deploy(&workspace_root, &project, production),
        },
        Commands::Post { command } => match command {
            PostCommands::Publish { file, slug, title, summary, tags, external_url, remote } => {
                publish_post(&workspace_root, &file, &slug, &title, summary.as_deref(), tags.as_deref(), external_url.as_deref(), remote)
            }
            PostCommands::List { remote } => list_posts(&workspace_root, remote),
            PostCommands::Delete { slug, remote } => delete_post(&workspace_root, &slug, remote),
        },
        Commands::Resume { command } => match command {
            ResumeCommands::Update { remote } => update_resume(&workspace_root, remote),
        },
        Commands::Migrate { remote } => migrate(&workspace_root, remote),
    }
}

/// Load wrangler.toml configuration
fn load_wrangler_config(workspace_root: &Path) -> Result<WranglerConfig> {
    let config_path = workspace_root.join("api/wrangler.toml");
    let content = std::fs::read_to_string(&config_path)
        .context("Failed to read api/wrangler.toml")?;
    let config: WranglerConfig = toml::from_str(&content)
        .context("Failed to parse wrangler.toml")?;
    Ok(config)
}

/// Find the workspace root by looking for Cargo.toml with \[workspace\]
fn find_workspace_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let mut dir = current_dir.as_path();

    loop {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml)?;
            if content.contains("[workspace]") {
                return Ok(dir.to_path_buf());
            }
        }

        dir = dir.parent()
            .ok_or_else(|| anyhow::anyhow!("Could not find workspace root"))?;
    }
}

/// Discover projects in workspace (directories with package.json or Cargo.toml)
fn discover_projects(workspace_root: &Path) -> Result<Vec<String>> {
    let mut projects = Vec::new();

    for entry in std::fs::read_dir(workspace_root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Skip hidden directories and xtask itself
            if dir_name.starts_with('.') || dir_name == "xtask" || dir_name == "target" {
                continue;
            }

            // Check for package.json or Cargo.toml
            if path.join("package.json").exists() || path.join("Cargo.toml").exists() {
                projects.push(dir_name.to_string());
            }
        }
    }

    projects.sort();
    Ok(projects)
}

fn list_projects(workspace_root: &Path) -> Result<()> {
    let projects = discover_projects(workspace_root)?;

    for project in &projects {
        let project_path = workspace_root.join(project);
        let has_package_json = project_path.join("package.json").exists();
        let has_cargo_toml = project_path.join("Cargo.toml").exists();

        let type_info = match (has_package_json, has_cargo_toml) {
            (true, true) => "Node.js + Rust",
            (true, false) => "Node.js",
            (false, true) => "Rust",
            (false, false) => "Unknown",
        };

        status!("Project", "{} ({})", project, type_info);
    }

    Ok(())
}

fn deploy(workspace_root: &Path, project: &str, production: bool) -> Result<()> {
    // Validate project exists
    let projects = discover_projects(workspace_root)?;
    if !projects.contains(&project.to_string()) {
        anyhow::bail!(
            "Project '{}' not found. Available projects: {}",
            project,
            projects.join(", ")
        );
    }

    let project_path = workspace_root.join(project);

    let target = if production { "production" } else { "preview" };
    status!("Deploying", "{} to {}", project, target);

    let mut cmd = Command::new("npx");
    cmd.arg("wrangler").arg("deploy");

    cmd.current_dir(&project_path);

    let deploy_status = cmd.status()
        .context("Failed to run wrangler deploy")?;

    if !deploy_status.success() {
        anyhow::bail!("Deployment failed");
    }

    status!("Finished", "deployment");
    Ok(())
}

fn update_resume(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Updating", "resume data ({})", mode);

    // Load wrangler config
    let config = load_wrangler_config(workspace_root)?;
    let kv_namespace_id = config.kv_namespaces
        .and_then(|namespaces| {
            namespaces.iter()
                .find(|ns| ns.binding == "RESUME_KV")
                .map(|ns| ns.id.clone())
        })
        .context("RESUME_KV namespace not found in wrangler.toml")?;

    // Fetch resume from cloud.werdxz.info
    let resume_json = Command::new("curl")
        .args(["-s", "https://cloud.werdxz.info/resume/public/resume.json"])
        .output()
        .context("Failed to fetch resume")?;

    if !resume_json.status.success() {
        anyhow::bail!("Failed to download resume");
    }

    // Save to temp file using cross-platform temp directory
    let temp_dir = dirs::cache_dir()
        .or_else(|| std::env::temp_dir().into())
        .context("Failed to get temp directory")?;
    let temp_file = temp_dir.join("resume.json");
    std::fs::write(&temp_file, &resume_json.stdout)
        .context("Failed to write temp file")?;

    // Upload to KV
    let mut cmd = Command::new("npx");
    cmd.arg("wrangler")
        .arg("kv")
        .arg("key")
        .arg("put")
        .arg("resume")
        .arg("--namespace-id")
        .arg(&kv_namespace_id)
        .arg("--path")
        .arg(&temp_file);

    if remote {
        cmd.arg("--remote");
    }

    cmd.current_dir(workspace_root.join("api"));

    let kv_status = cmd.status()
        .context("Failed to run wrangler")?;

    if !kv_status.success() {
        anyhow::bail!("Failed to upload resume to KV");
    }

    status!("Finished", "resume update");
    Ok(())
}

fn migrate(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "remote" } else { "local" };
    status!("Migrating", "database ({})", mode);

    // Load wrangler config
    let config = load_wrangler_config(workspace_root)?;
    let db_name = config.d1_databases
        .and_then(|dbs| {
            dbs.iter()
                .find(|db| db.binding == "DB")
                .map(|db| db.database_name.clone())
        })
        .context("DB database not found in wrangler.toml")?;

    let mut cmd = Command::new("npx");
    cmd.arg("wrangler")
        .arg("d1")
        .arg("migrations")
        .arg("apply")
        .arg(&db_name);

    if remote {
        cmd.arg("--remote");
    }

    cmd.current_dir(workspace_root.join("api"));

    let migrate_status = cmd.status()
        .context("Failed to run migrations")?;

    if !migrate_status.success() {
        anyhow::bail!("Migration failed");
    }

    status!("Finished", "migrations");
    Ok(())
}

fn publish_post(
    workspace_root: &Path,
    file: &str,
    slug: &str,
    title: &str,
    summary: Option<&str>,
    tags: Option<&str>,
    external_url: Option<&str>,
    remote: bool,
) -> Result<()> {
    let content_id = uuid::Uuid::new_v4();

    status!("Publishing", "{}", title);
    println!("             slug: {}", slug);
    println!("       content_id: {}", content_id);

    // Load wrangler config
    let config = load_wrangler_config(workspace_root)?;
    let db_name = config.d1_databases
        .as_ref()
        .and_then(|dbs| {
            dbs.iter()
                .find(|db| db.binding == "DB")
                .map(|db| db.database_name.clone())
        })
        .context("DB database not found in wrangler.toml")?;

    let bucket_name = config.r2_buckets
        .as_ref()
        .and_then(|buckets| {
            buckets.iter()
                .find(|b| b.binding == "CONTENT_BUCKET")
                .map(|b| b.bucket_name.clone())
        })
        .context("CONTENT_BUCKET not found in wrangler.toml")?;

    // 1. Upload markdown to R2
    let r2_key = format!("posts/{}.md", content_id);
    let r2_path = format!("{}/{}", bucket_name, r2_key);

    // Resolve file path relative to workspace root before changing directories
    let file_path = if Path::new(file).is_absolute() {
        PathBuf::from(file)
    } else {
        workspace_root.join(file)
    };

    let mut r2_cmd = Command::new("npx");
    r2_cmd.args(["wrangler", "r2", "object", "put", &r2_path]);

    if remote {
        r2_cmd.arg("--remote");
    }

    r2_cmd.arg("--file")
        .arg(&file_path)
        .current_dir(workspace_root.join("api"));

    let r2_status = r2_cmd.status()
        .context("Failed to upload to R2")?;

    if !r2_status.success() {
        anyhow::bail!("Failed to upload content to R2");
    }

    // 2. Insert metadata into D1
    let summary_str = summary.unwrap_or("");
    let external_url_str = external_url.map(|s| format!(", '{}'", s)).unwrap_or_else(|| ", NULL".to_string());

    // Insert post without tags (tags will be handled separately)
    // Escape single quotes in SQL strings by doubling them
    let escaped_title = title.replace('\'', "''");
    let escaped_summary = summary_str.replace('\'', "''");

    let sql = format!(
        "INSERT INTO posts (content_id, slug, title, summary, published_at, external_url) \
         VALUES ('{}', '{}', '{}', '{}', datetime('now'){});",
        content_id, slug, escaped_title, escaped_summary, external_url_str
    );

    let mut db_cmd = Command::new("npx");
    db_cmd.args(["wrangler", "d1", "execute", &db_name]);

    if remote {
        db_cmd.arg("--remote");
    }

    db_cmd.arg("--command")
        .arg(&sql)
        .current_dir(workspace_root.join("api"));

    let db_status = db_cmd.status()
        .context("Failed to insert into D1")?;

    if !db_status.success() {
        anyhow::bail!("Failed to insert post metadata");
    }

    // 3. Handle tags if provided
    if let Some(tags_str) = tags {
        let tag_names: Vec<&str> = tags_str.split(',').map(str::trim).collect();

        for tag_name in tag_names {
            if tag_name.is_empty() {
                continue;
            }

            // Insert tag (ignore if exists)
            let insert_tag_sql = format!(
                "INSERT OR IGNORE INTO tags (name) VALUES ('{}');",
                tag_name
            );

            let mut tag_cmd = Command::new("npx");
            tag_cmd.args(["wrangler", "d1", "execute", &db_name]);

            if remote {
                tag_cmd.arg("--remote");
            }

            tag_cmd.arg("--command")
                .arg(&insert_tag_sql)
                .current_dir(workspace_root.join("api"));

            tag_cmd.status()
                .context("Failed to insert tag")?;

            // Get tag ID and insert into post_tags junction table
            let link_tag_sql = format!(
                "INSERT INTO post_tags (post_id, tag_id) \
                 SELECT '{}', id FROM tags WHERE name = '{}';",
                content_id, tag_name
            );

            let mut link_cmd = Command::new("npx");
            link_cmd.args(["wrangler", "d1", "execute", &db_name]);

            if remote {
                link_cmd.arg("--remote");
            }

            link_cmd.arg("--command")
                .arg(&link_tag_sql)
                .current_dir(workspace_root.join("api"));

            link_cmd.status()
                .context("Failed to link tag to post")?;
        }
    }

    status!("Finished", "publishing");
    println!("              url: /posts/{}", slug);
    Ok(())
}

fn list_posts(workspace_root: &Path, remote: bool) -> Result<()> {
    let mode = if remote { "--remote" } else { "--local" };
    let location = if remote { "remote" } else { "local" };

    status!("Listing", "posts ({})", location);

    // Load wrangler config
    let config = load_wrangler_config(workspace_root)?;
    let db_name = config.d1_databases
        .and_then(|dbs| {
            dbs.iter()
                .find(|db| db.binding == "DB")
                .map(|db| db.database_name.clone())
        })
        .context("DB database not found in wrangler.toml")?;

    // Query posts with tags from junction table
    let query = "SELECT p.slug, p.title, p.published_at, \
                 GROUP_CONCAT(t.name, ', ') as tags \
                 FROM posts p \
                 LEFT JOIN post_tags pt ON p.content_id = pt.post_id \
                 LEFT JOIN tags t ON pt.tag_id = t.id \
                 GROUP BY p.content_id \
                 ORDER BY p.published_at DESC;";

    let output = Command::new("npx")
        .args(["wrangler", "d1", "execute", &db_name, mode])
        .arg("--command")
        .arg(query)
        .current_dir(workspace_root.join("api"))
        .output()
        .context("Failed to query posts")?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

fn delete_post(workspace_root: &Path, slug: &str, remote: bool) -> Result<()> {
    let mode = if remote { "--remote" } else { "--local" };

    status!("Deleting", "post: {}", slug);

    // Load wrangler config
    let config = load_wrangler_config(workspace_root)?;
    let db_name = config.d1_databases
        .as_ref()
        .and_then(|dbs| {
            dbs.iter()
                .find(|db| db.binding == "DB")
                .map(|db| db.database_name.clone())
        })
        .context("DB database not found in wrangler.toml")?;

    let bucket_name = config.r2_buckets
        .as_ref()
        .and_then(|buckets| {
            buckets.iter()
                .find(|b| b.binding == "CONTENT_BUCKET")
                .map(|b| b.bucket_name.clone())
        })
        .context("CONTENT_BUCKET not found in wrangler.toml")?;

    // 1. Get content_id from D1 (for future R2 deletion)
    let _output = Command::new("npx")
        .args(["wrangler", "d1", "execute", &db_name, mode])
        .arg("--command")
        .arg(format!("SELECT content_id FROM posts WHERE slug = '{}';", slug))
        .current_dir(workspace_root.join("api"))
        .output()
        .context("Failed to get content_id")?;

    // TODO: Parse content_id from output for automatic R2 deletion
    // For now, we just delete the DB entry and note that R2 cleanup is manual

    // 2. Delete from D1 (cascades to post_tags due to foreign key)
    let delete_status = Command::new("npx")
        .args(["wrangler", "d1", "execute", &db_name, mode])
        .arg("--command")
        .arg(format!("DELETE FROM posts WHERE slug = '{}';", slug))
        .current_dir(workspace_root.join("api"))
        .status()
        .context("Failed to delete from D1")?;

    if !delete_status.success() {
        anyhow::bail!("Failed to delete post from D1");
    }

    status!("Finished", "deletion");
    println!("        Note: R2 content not deleted automatically");
    println!("              Use: npx wrangler r2 object delete {} posts/<content_id>.md", bucket_name);

    Ok(())
}
