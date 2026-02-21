## Why
The current homepage still focuses on design philosophy text and can feel empty for returning visitors. We need the center section to highlight recent work in a lightweight way while keeping the page simple.

## What Changes
- Replace the homepage middle section with a "Latest Updates" feed instead of design-philosophy copy.
- Fetch updates from existing API sources (`/v1/posts` and `/v1/projects`), normalize items, sort by date descending, and render a small list.
- Default to showing 2 updates on the homepage to keep the section concise.
- Keep implementation extensible via a source-adapter configuration so future sources can be added without restructuring the page.
- Add resilient fallback/empty states that still provide useful navigation links when API data is unavailable.

## Impact
- Affected specs: `homepage`
- Affected code: `www/public/index.html`
- User-visible behavior: homepage now shows recent updates instead of the design philosophy section
