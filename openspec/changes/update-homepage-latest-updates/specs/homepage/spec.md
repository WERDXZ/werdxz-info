## RENAMED Requirements
- FROM: `### Requirement: The page MUST include a section explaining the website's design philosophy and monorepo structure`
- TO: `### Requirement: The page MUST include a section showing latest updates`

## MODIFIED Requirements
### Requirement: The page MUST include a section showing latest updates

The page SHALL contain a dedicated "Latest Updates" section that surfaces recent entries from configured sources in a concise, scannable list.

#### Scenario: User sees recent work at a glance
- **Given** the home page is loaded
- **When** the user reaches the updates section
- **Then** a "Latest Updates" heading should be visible
- **And** the section should render entries sorted by date in descending order
- **And** the section should show up to 2 entries by default
- **And** each entry should include type, title, date, and destination link

#### Scenario: Update sources are extensible
- **Given** future data sources may be added
- **When** the page fetches updates
- **Then** each source should be defined through a source configuration and mapping step
- **And** fetched items from all sources should be normalized into a shared shape before sorting

#### Scenario: Upstream data is unavailable
- **Given** one or more update sources fail to load
- **When** the updates section is rendered
- **Then** the page should display a fallback message instead of an empty block
- **And** the section should provide links to the blog and projects pages
