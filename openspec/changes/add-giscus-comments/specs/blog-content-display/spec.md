## ADDED Requirements

### Requirement: Blog Post Comments

The blog SHALL display a comment section below each blog post content using Giscus.

#### Scenario: Comments load on post page
- **WHEN** a user views a blog post
- **THEN** a Giscus comment widget SHALL be displayed below the post content
- **AND** the widget SHALL use a custom Gruvbox theme served from the API worker

#### Scenario: Comment mapping
- **WHEN** a blog post is loaded
- **THEN** comments SHALL be mapped to GitHub Discussions by the post's `og:title`

#### Scenario: GitHub authentication
- **WHEN** a user wants to comment
- **THEN** they SHALL be prompted to authenticate via GitHub

#### Scenario: Theme respects color scheme
- **WHEN** the user has dark mode enabled (via `prefers-color-scheme`)
- **THEN** the comment widget SHALL display in Gruvbox Dark colors
- **WHEN** the user has light mode enabled
- **THEN** the comment widget SHALL display in Gruvbox Light colors
