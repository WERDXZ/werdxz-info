# Portfolio Management Scripts

Shell scripts for managing portfolio content in Cloudflare KV.

## Available Scripts

- `hero.sh` - Manage hero section content (subtitle/description)
- `about.sh` - Manage about section content (paragraphs)
- `project.sh` - Manage projects and featured lists
- `experience.sh` - Manage work experience and featured lists

## Modes

Valid portfolio modes:
- `software-engineer` (default, path: `/`)
- `fullstack` (path: `/fullstack`)
- `rust` (path: `/rust`)
- `student` (path: `/student`)

## Hero Content

```bash
# List all hero content
./scripts/portfolio/hero.sh list

# Get hero content for a mode
./scripts/portfolio/hero.sh get software-engineer

# Set hero content for a mode
./scripts/portfolio/hero.sh set rust \
    "Rust Developer" \
    "Systems programming and high-performance tools with Rust, WebAssembly, and native development."
```

## About Content

```bash
# List all about content
./scripts/portfolio/about.sh list

# Get about content for a mode
./scripts/portfolio/about.sh get fullstack

# Set about content for a mode (multiple paragraphs)
./scripts/portfolio/about.sh set fullstack \
    "I'm a full-stack developer specializing in modern web applications." \
    "My focus is on creating seamless user experiences."
```

## Projects

```bash
# List all projects
./scripts/portfolio/project.sh list

# Get a specific project
./scripts/portfolio/project.sh get chico-rs

# Add/update a project from JSON file
./scripts/portfolio/project.sh set chico-rs project.json

# List featured projects for a mode
./scripts/portfolio/project.sh featured rust

# Set featured projects for a mode
./scripts/portfolio/project.sh set-featured rust chico-rs wlrs archenemy
```

## Experience

```bash
# List all experiences
./scripts/portfolio/experience.sh list

# Get a specific experience
./scripts/portfolio/experience.sh get rust-club

# Add/update experience from JSON file
./scripts/portfolio/experience.sh set rust-club experience.json

# List featured experiences for a mode
./scripts/portfolio/experience.sh featured student

# Set featured experiences for a mode
./scripts/portfolio/experience.sh set-featured student cs-tutor rust-club erp-volunteer
```

## JSON Schemas

JSON schemas are available in `portfolio/schemas/`:
- `hero-content.json`
- `about-content.json`
- `project.json`
- `experience.json`

Use these to validate your JSON before uploading.

## Examples

### Add a new project

Create `my-project.json`:
```json
{
  "title": "My Project",
  "description": "Project description",
  "technologies": ["Rust", "TypeScript"],
  "image_url": null,
  "redirect_url": "https://github.com/me/my-project",
  "links": [
    {"label": "GitHub", "url": "https://github.com/me/my-project"}
  ]
}
```

```bash
# Upload to KV
./scripts/portfolio/project.sh set my-project my-project.json

# Add to featured list for rust mode
./scripts/portfolio/project.sh set-featured rust my-project chico-rs wlrs
```

### Update hero content

```bash
# Change rust mode hero text
./scripts/portfolio/hero.sh set rust \
    "Senior Rust Engineer" \
    "Building production-grade systems with 10+ years experience."
```
