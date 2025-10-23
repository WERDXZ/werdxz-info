# Tasks: implement-resume-page

## Implementation Tasks

### Phase 1: Project Setup
1. **Create resume app directory structure**
   - [ ] Create `resume/` directory at monorepo root
   - [ ] Create `resume/public/` for static files
   - [ ] Create `resume/public/js/` for JavaScript modules
   - [ ] Create `resume/public/css/` for styles
   - **Validation**: Directory structure exists

2. **Configure package.json and wrangler.toml**
   - [ ] Create `resume/package.json` with dev and deploy scripts
   - [ ] Add wrangler as dependency
   - [ ] Create `resume/wrangler.toml` with Worker configuration
   - [ ] Set name = "werdxz-resume" and configure assets directory
   - [ ] Set compatibility_date to current date
   - **Validation**: Files exist and are properly formatted

3. **Setup development environment**
   - [ ] Run `npm install` in resume/ directory
   - [ ] Create `.gitignore` for resume/public/ files
   - [ ] Test that `npm run dev` starts local server
   - **Validation**: Development server runs successfully

### Phase 2: Search Engine Blocking
4. **Create robots.txt**
   - [ ] Create `resume/public/robots.txt`
   - [ ] Add `User-agent: *` and `Disallow: /`
   - [ ] Verify no sitemap reference
   - **Validation**: robots.txt exists and blocks all crawlers

### Phase 3: HTML and Basic Structure
5. **Create base HTML structure**
   - [ ] Create `resume/public/index.html` with semantic HTML
   - [ ] Add proper DOCTYPE, meta tags (charset, viewport)
   - [ ] Add `<meta name="robots" content="noindex, nofollow">`
   - [ ] Add `<title>` tag
   - [ ] Import shared design system CSS
   - [ ] Add placeholder custom element tags
   - **Validation**: HTML is valid and well-formed

6. **Create base CSS file**
   - [ ] Create `resume/public/css/main.css`
   - [ ] Import shared variables: `@import url('https://cloud.werdxz.info/shared/styles/variables.css');`
   - [ ] Add reset/base styles
   - [ ] Add container and layout styles
   - [ ] Use CSS custom properties for all design values
   - **Validation**: CSS loads and uses shared variables

### Phase 4: Custom Elements Implementation
7. **Define custom element: resume-header**
   - [ ] Create `resume/public/js/resume-header.js`
   - [ ] Define class extending HTMLElement
   - [ ] Implement connectedCallback to render header (name, title, contact)
   - [ ] Register element with customElements.define()
   - [ ] Add styles (inline or via CSS)
   - **Validation**: Element renders correctly in page

8. **Define custom element: resume-experience**
   - [ ] Create `resume/public/js/resume-experience.js`
   - [ ] Define class extending HTMLElement
   - [ ] Render experience entries (company, role, dates, description)
   - [ ] Support multiple experience items
   - [ ] Register element
   - **Validation**: Experience section renders with data

9. **Define custom element: resume-education**
   - [ ] Create `resume/public/js/resume-education.js`
   - [ ] Define class extending HTMLElement
   - [ ] Render education entries (institution, degree, dates)
   - [ ] Support multiple education items
   - [ ] Register element
   - **Validation**: Education section renders with data

10. **Define custom element: resume-skills**
    - [ ] Create `resume/public/js/resume-skills.js`
    - [ ] Define class extending HTMLElement
    - [ ] Render skills by category
    - [ ] Support multiple skill categories
    - [ ] Register element
    - **Validation**: Skills section renders with data

11. **Create main JavaScript entry point**
    - [ ] Create `resume/public/js/main.js`
    - [ ] Import and register all custom elements
    - [ ] Define resume data structure (embedded or separate JSON)
    - [ ] Pass data to custom elements via attributes or properties
    - [ ] Add error handling
    - **Validation**: All elements load and render without errors

### Phase 5: Resume Data and Content
12. **Define resume data structure**
    - [ ] Decide on data format (embedded object, JSON file, or HTML data attributes)
    - [ ] Create data structure with: header, experience, education, skills
    - [ ] Add placeholder content for all sections
    - [ ] Ensure data is easily updatable
    - **Validation**: Data structure is complete and accessible

13. **Add actual resume content**
    - [ ] Convert Typst resume content to data format
    - [ ] Fill in work experience details
    - [ ] Fill in education details
    - [ ] Fill in skills and technologies
    - [ ] Review content for accuracy
    - **Validation**: Resume content is complete and accurate

### Phase 6: Styling and Polish
14. **Implement responsive design**
    - [ ] Add mobile styles using shared breakpoint variables
    - [ ] Test layout on different viewport sizes
    - [ ] Ensure readability on small screens
    - [ ] Add print styles for potential PDF generation
    - **Validation**: Resume looks good on all screen sizes

15. **Polish typography and spacing**
    - [ ] Use shared typography variables consistently
    - [ ] Apply shared spacing scale
    - [ ] Ensure visual hierarchy is clear
    - [ ] Add appropriate line heights and margins
    - **Validation**: Typography is professional and readable

16. **Add visual design** (waiting for user design input)
    - [ ] Apply color scheme using shared variables
    - [ ] Add any borders, backgrounds, or visual elements
    - [ ] Ensure consistent with design goals
    - [ ] Review with user for approval
    - **Validation**: Design matches user requirements

### Phase 7: Testing
17. **Test locally**
    - [ ] Run `npm run dev` and test all functionality
    - [ ] Test on different browsers (Chrome, Firefox, Safari)
    - [ ] Test on mobile devices or responsive mode
    - [ ] Verify robots.txt and noindex meta tag
    - [ ] Check browser console for errors
    - **Validation**: No errors, all features work

18. **Test custom elements**
    - [ ] Verify each custom element renders correctly
    - [ ] Test with different data variations
    - [ ] Verify shadow DOM (if used) doesn't break styles
    - [ ] Test progressive enhancement (JS disabled scenario)
    - **Validation**: Elements work as expected

### Phase 8: Deployment and Documentation
19. **Deploy to Cloudflare**
    - [ ] Configure subdomain routing for resume.werdxz.info
    - [ ] Run `npm run deploy` from resume/ directory
    - [ ] Verify deployment succeeds
    - [ ] Test deployed URL in browser
    - [ ] Verify SSL certificate is valid
    - **Validation**: Site is live and accessible at resume.werdxz.info

20. **Update documentation**
    - [ ] Create `resume/README.md` with local dev and deployment instructions
    - [ ] Update root `README.md` to include resume app
    - [ ] Document custom elements architecture
    - [ ] Document data structure and how to update content
    - **Validation**: Documentation is clear and complete

21. **Verify search engine blocking**
    - [ ] Test robots.txt at resume.werdxz.info/robots.txt
    - [ ] Use robots.txt tester tool
    - [ ] Verify noindex meta tag in deployed HTML source
    - [ ] Optional: Submit to search console and verify noindex status
    - **Validation**: Resume is properly blocked from indexing

## Dependencies and Parallelization
- Tasks 1-3 (setup) must be done first
- Task 4 (robots.txt) can be done anytime before deployment
- Tasks 5-6 (HTML/CSS) can start after task 1
- Tasks 7-11 (custom elements) can be done in parallel after task 5
- Task 12 (data structure) should be done before task 13
- Tasks 14-16 (styling) can start after tasks 5-11 are complete
- Tasks 17-18 (testing) depend on all implementation tasks
- Tasks 19-21 (deployment and docs) are final steps

## Notes
- Design details TBD - user will provide specific design requirements
- Custom element names can be refined based on preference
- Data structure format (embedded vs. JSON) can be decided during implementation
- Print styles added for potential future PDF generation feature
- Shadow DOM is optional - can use regular DOM if preferred
