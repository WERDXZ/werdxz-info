# Shared Styles

**Version:** 1.0.0

This directory contains shared design primitives and basic semantic tokens for the werdxz.info monorepo.

**These files ARE committed to git** - they're source code, not content.

## Versioning

This design system follows [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes (e.g., removing variables, changing naming conventions)
- **MINOR**: New features (e.g., adding new primitives, new semantic tokens)
- **PATCH**: Bug fixes and minor tweaks (e.g., color value adjustments)

## Philosophy

- **Primitives**: Raw values (colors, spacing, typography) shared across all apps
- **Base Semantics**: Common semantic mappings that most apps will use
- **Project-Specific**: Each app can define additional semantics or override these

## Files

- `variables.css` - Shared CSS primitives and base semantic tokens

## Usage

### Importing in Your App

You can import the shared styles in two ways:

**1. Via CDN (recommended for deployed apps):**
```css
@import url('https://cloud.werdxz.info/shared/styles/variables.css');
```

**2. Via relative path (for local development):**
```css
@import url('../../../shared/styles/variables.css');
```

Files are automatically synced to the R2 `cloud` bucket via git hooks when committed.

### Using Primitives Directly

```css
.my-component {
  /* Use primitives directly */
  color: var(--blue);
  background-color: var(--gray-50);

  /* Or use base semantics */
  color: var(--color-text-primary);
  background-color: var(--color-bg-secondary);

  /* Spacing and typography (always use these!) */
  padding: var(--spacing-md);
  font-family: var(--font-family-sans);
  font-size: var(--font-size-base);
}
```

### Extending or Overriding in Your App

You can define project-specific semantics by referencing the shared primitives:

```css
/* In your app's CSS file */
:root {
  /* Project-specific semantic tokens */
  --color-accent: var(--blue-light);
  --color-card-bg: var(--gray-100);
  --color-hero-text: var(--gray-900);

  /* Or override base semantics if needed */
  --color-text-link: var(--green); /* use green instead of blue */
}
```

### Responsive Design

Use the breakpoint variables in media queries:

```css
.container {
  max-width: 100%;
}

@media (min-width: 768px) { /* var(--breakpoint-md) */
  .container {
    max-width: 768px;
  }
}

@media (min-width: 1024px) { /* var(--breakpoint-lg) */
  .container {
    max-width: 1024px;
  }
}
```

## Available Tokens

### Color Primitives
- **Blue**: `--blue`, `--blue-light`, `--blue-dark`
- **Grays**: `--gray-50`, `--gray-100`, `--gray-200`, `--gray-400`, `--gray-600`, `--gray-800`, `--gray-900`
- **Accent Colors**: `--green`, `--red`, `--yellow`

### Base Semantic Colors
- **Backgrounds**: `--color-bg-primary`, `--color-bg-secondary`
- **Text**: `--color-text-primary`, `--color-text-secondary`, `--color-text-tertiary`, `--color-text-link`, `--color-text-link-hover`
- **Borders**: `--color-border-default`, `--color-border-subtle`

### Typography
- **Font Families**: `--font-family-{sans|mono|serif}`
- **Font Sizes**: `--font-size-{xs|sm|base|md|lg|xl|2xl|3xl|4xl|5xl}`
- **Heading Sizes**: `--font-size-h{1-6}`
- **Font Weights**: `--font-weight-{light|normal|medium|semibold|bold|extrabold}`
- **Line Heights**: `--line-height-{tight|normal|relaxed|loose}`

### Spacing
- Scale: `--spacing-{xs|sm|md|lg|xl|2xl|3xl|4xl|5xl}`
- Values: 4px, 8px, 16px, 24px, 32px, 48px, 64px, 96px, 128px

### Layout
- **Breakpoints**: `--breakpoint-{xs|sm|md|lg|xl|2xl}`
- **Border Radius**: `--radius-{none|sm|base|md|lg|xl|2xl|full}`
- **Shadows**: `--shadow-{sm|base|md|lg|xl|2xl}`
- **Z-Index**: `--z-index-{dropdown|sticky|fixed|modal|popover|tooltip}`

### Transitions
- `--transition-{fast|base|slow}`

## Naming Convention

**Primitives**: Simple, direct names
- `--blue`, `--gray-600`, `--spacing-md`

**Semantics**: Describe usage, not appearance
- `--color-text-primary` (not `--color-dark-gray`)
- `--color-bg-secondary` (not `--color-light-gray`)
- `--color-text-link` (not `--color-blue`)

## Dark Mode Support

The variables.css file includes automatic dark mode support using `@media (prefers-color-scheme: dark)`. Base semantic colors automatically adjust based on the user's system preference.

## Version History

### 1.0.0 (2025-10-22)
- Initial release
- Base color primitives (blue, gray scale, green, red, yellow)
- Base semantic tokens (bg, text, border colors)
- Typography system (fonts, sizes, weights, line heights)
- Spacing scale (xs to 5xl)
- Breakpoints, border radius, shadows, z-index, transitions

## Adding New Tokens

**To add a new primitive color:**
1. Add it to the "Color Primitives" section in `variables.css`
2. Use simple, descriptive names: `--purple`, `--orange`, etc.
3. Update version in both README.md and variables.css header

**To add a new semantic token:**
1. Consider if it's truly shared across all apps
2. If yes, add to `variables.css`
3. If no, define it in your app-specific CSS instead

**General guidelines:**
- Keep primitives minimal - only add what multiple apps will use
- Prefer app-specific semantics over shared ones
- Spacing, typography, and breakpoints should stay shared
- Follow semantic versioning when updating
- Commit changes to git for version control and code review
