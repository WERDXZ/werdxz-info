# Spec: Blog Content Display and Rendering

## ADDED Requirements

### Requirement: Markdown Rendering
The blog MUST render markdown content as formatted HTML.

#### Scenario: Parse and render markdown
**GIVEN** a blog post with markdown content
**WHEN** displaying the post
**THEN** parse markdown to HTML using markdown-it or remark
**AND** preserve formatting (headings, lists, emphasis, links)
**AND** sanitize HTML output to prevent XSS attacks
**AND** apply styles matching design system

#### Scenario: Supported markdown features
**GIVEN** markdown content with various syntax
**WHEN** rendering the content
**THEN** support headings (h1-h6)
**AND** support bold, italic, strikethrough
**AND** support ordered and unordered lists
**AND** support blockquotes
**AND** support inline code and code blocks
**AND** support links and images
**AND** support tables
**AND** support horizontal rules

#### Scenario: Markdown security
**GIVEN** markdown content may contain unsafe HTML
**WHEN** rendering to HTML
**THEN** sanitize output to remove dangerous tags (<script>, <iframe>)
**AND** escape user-generated content
**AND** allow safe HTML tags (strong, em, a, img)
**AND** prevent XSS attacks

### Requirement: Code Syntax Highlighting
The blog MUST provide syntax highlighting for code blocks.

**Implementation Note**: Originally attempted server-side highlighting with Shiki, but this caused Cloudflare Workers CPU time limit errors (10ms+ per request). Migrated to client-side highlight.js to eliminate server CPU cost and prevent production errors.

#### Scenario: Highlight code blocks
**GIVEN** a markdown code block with language specifier
**WHEN** rendering the post
**THEN** apply syntax highlighting using highlight.js (client-side)
**AND** support common languages (JavaScript, TypeScript, Rust, Python, Go, HTML, CSS)
**AND** use base16-gruvbox-dark-medium theme
**AND** dynamically import highlight.js to avoid SSR issues
**AND** run highlighting on document-ready to ensure DOM is available

#### Scenario: Code block styling
**GIVEN** a highlighted code block
**WHEN** displaying the code
**THEN** use monospace font from design system (--font-family-mono)
**AND** apply background color from highlight.js theme
**AND** override default padding with custom spacing (--spacing-md vertical, --spacing-lg horizontal)
**AND** ensure code is readable with gruvbox-dark-medium theme
**AND** support horizontal scrolling for long lines
**AND** apply border-radius for visual polish

#### Scenario: Copy code button
**GIVEN** a code block is displayed
**WHEN** user hovers over the code
**THEN** show "Copy" button in top-right corner
**AND** copy code to clipboard on click
**AND** show "Copied!" confirmation message
**AND** handle copy errors gracefully

#### Scenario: Inline code styling
**GIVEN** inline code in markdown (backticks)
**WHEN** rendering the text
**THEN** wrap in <code> tag
**AND** apply monospace font
**AND** apply subtle background color
**AND** do not apply syntax highlighting (just styling)

### Requirement: Image Handling
The blog MUST handle images in markdown content.

#### Scenario: Render markdown images
**GIVEN** markdown with image syntax ![alt](url)
**WHEN** rendering the content
**THEN** output <img> tag with src and alt attributes
**AND** use responsive image sizing (max-width: 100%)
**AND** include alt text for accessibility
**AND** lazy-load images below the fold

#### Scenario: Image optimization
**GIVEN** images in blog posts
**WHEN** displaying them
**THEN** serve images from R2 bucket or CDN
**AND** use WebP format with JPEG fallback (when possible)
**AND** apply lazy loading with loading="lazy" attribute
**AND** include width and height to prevent layout shift

#### Scenario: Broken image handling
**GIVEN** an image fails to load
**WHEN** displaying the post
**THEN** show alt text in place of image
**AND** apply broken image styling
**AND** do not break page layout

### Requirement: Typography and Readability
The blog MUST ensure content is readable and well-formatted.

#### Scenario: Reading width constraint
**GIVEN** a blog post is displayed
**WHEN** viewing on desktop (> 768px)
**THEN** constrain content width to 65-75 characters per line (optimal readability)
**AND** center content on page
**AND** add padding on sides for breathing room

#### Scenario: Heading hierarchy
**GIVEN** markdown with headings
**WHEN** rendering the post
**THEN** use h1 for post title only
**AND** use h2-h6 for content headings
**AND** apply consistent spacing between headings and paragraphs
**AND** use design system font sizes (--font-size-h2, --font-size-h3, etc.)

#### Scenario: Link styling
**GIVEN** markdown with links
**WHEN** rendering the content
**THEN** style links with underline and color from design system (--color-text-link)
**AND** show hover state (change color or underline style)
**AND** open external links in new tab (target="_blank")
**AND** add rel="noopener noreferrer" for security

#### Scenario: List formatting
**GIVEN** markdown with lists
**WHEN** rendering the content
**THEN** apply proper indentation for nested lists
**AND** use design system spacing (--spacing-sm, --spacing-md)
**AND** style list markers (bullets, numbers)
**AND** ensure lists are readable in both light and dark mode

### Requirement: Post Metadata Display
The blog MUST display post metadata prominently.

#### Scenario: Post header component
**GIVEN** a post detail page
**WHEN** displaying the post
**THEN** show post title as h1
**AND** show published date in readable format (e.g., "October 24, 2025")
**AND** show estimated reading time (e.g., "5 min read")
**AND** show tags as clickable links to tag pages

#### Scenario: Tag display
**GIVEN** a post has tags ["rust", "webdev", "cloudflare"]
**WHEN** displaying tags
**THEN** render each tag as flat text with middot separator (·)
**AND** link each tag to `/?tags={tag}` for filtering
**AND** apply subtle color (--color-text-secondary) with hover state (--color-text-link)
**AND** add padding around tag links (--spacing-xs vertical, --spacing-sm horizontal)
**AND** add padding to separators (--spacing-xs horizontal)
**AND** ensure tags are keyboard accessible
**AND** make tags clickable with pointer cursor

#### Scenario: Date formatting
**GIVEN** a post published date
**WHEN** displaying the date
**THEN** format as "Month Day, Year" (e.g., "October 24, 2025")
**AND** include <time> tag with datetime attribute for machine-readability
**AND** use relative dates for recent posts (e.g., "2 days ago") - optional

### Requirement: Post Summary Cards
The blog index MUST display post summaries as cards.

#### Scenario: Post card layout
**GIVEN** the blog index displays posts
**WHEN** rendering each post
**THEN** show post title as h2 with link to full post
**AND** show summary text (truncated to ~150 characters)
**AND** show published date and reading time
**AND** show tags
**AND** include "Read more →" link

#### Scenario: Post card styling
**GIVEN** a post card component
**WHEN** displaying on the page
**THEN** apply card styling (border, padding, background from design system)
**AND** add hover effect (subtle shadow or border color change)
**AND** ensure consistent spacing between cards
**AND** make entire card clickable (not just title)

#### Scenario: Post card grid layout
**GIVEN** the blog index displays multiple posts
**WHEN** viewing on desktop (> 768px)
**THEN** display posts in single column (not grid) for readability
**AND** stack posts vertically with consistent spacing
**AND** ensure mobile-responsive (full-width on mobile)

### Requirement: Table of Contents
The blog MUST support table of contents generation for long posts.

#### Scenario: Generate TOC from headings
**GIVEN** a post with multiple h2 and h3 headings
**WHEN** rendering the post
**THEN** extract headings to generate table of contents
**AND** display TOC at top of post (optional, based on post length)
**AND** link each TOC item to corresponding heading (with anchor)
**AND** highlight current section on scroll (optional enhancement)

#### Scenario: TOC anchor links
**GIVEN** a table of contents is displayed
**WHEN** user clicks a TOC link
**THEN** scroll smoothly to the corresponding heading
**AND** update URL hash (#heading-slug)
**AND** maintain scroll position on page reload

### Requirement: Blockquote Styling
The blog MUST style blockquotes distinctively.

#### Scenario: Blockquote rendering
**GIVEN** markdown with blockquote (> text)
**WHEN** rendering the content
**THEN** wrap in <blockquote> tag
**AND** apply left border accent color (design system)
**AND** apply padding and italic font style
**AND** ensure contrast for readability

### Requirement: Horizontal Rule Styling
The blog MUST style horizontal rules for section breaks.

#### Scenario: Horizontal rule rendering
**GIVEN** markdown with horizontal rule (---)
**WHEN** rendering the content
**THEN** output <hr> element
**AND** apply subtle border color from design system
**AND** add vertical spacing above and below

### Requirement: Responsive Design
The blog content MUST be mobile-responsive.

#### Scenario: Mobile content width
**GIVEN** viewing on mobile device (< 768px)
**WHEN** displaying post content
**THEN** use full width minus padding
**AND** ensure text is readable (min font-size: 16px to prevent zoom)
**AND** allow horizontal scroll for wide code blocks
**AND** optimize line length for mobile (45-75 characters)

#### Scenario: Touch-friendly interactions
**GIVEN** viewing on mobile device
**WHEN** interacting with elements
**THEN** ensure tap targets are at least 44x44px
**AND** provide adequate spacing between links
**AND** make code copy button large enough to tap

### Requirement: Performance Optimization
The blog content rendering MUST be performant.

#### Scenario: Lazy-load markdown renderer
**GIVEN** the blog is built for production
**WHEN** loading a post page
**THEN** code-split markdown rendering library (marked)
**AND** dynamically import syntax highlighter (highlight.js) client-side
**AND** load highlighting only when DOM is ready (useVisibleTask$)
**AND** avoid SSR for syntax highlighting to prevent CPU time issues

#### Scenario: Avoid layout shift
**GIVEN** a post is loading
**WHEN** content appears
**THEN** reserve space for images (width/height attributes)
**AND** use skeleton loaders matching content structure
**AND** minimize Cumulative Layout Shift (CLS < 0.1)
