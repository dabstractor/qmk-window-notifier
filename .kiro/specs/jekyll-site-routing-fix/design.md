# Design Document

## Overview

The Jekyll documentation site for QMKonnect has critical routing issues that prevent the home page from loading at the root URL `/`. This design addresses the Jekyll configuration, file structure, and navigation setup needed to create a properly functioning documentation site.

## Architecture

### Jekyll Site Structure
```
docs/
├── _config.yml          # Jekyll configuration with proper baseurl and navigation
├── index.md            # Home page (nav_order: 1, layout: default)
├── installation.md     # Installation guide (nav_order: 2)
├── configuration.md    # Configuration guide (nav_order: 3)
├── usage.md           # Usage guide (nav_order: 4)
├── qmk-integration.md # QMK integration (nav_order: 5)
├── troubleshooting.md # Troubleshooting (nav_order: 6)
├── advanced.md        # Advanced usage (nav_order: 7)
├── examples.md        # Examples (nav_order: 8)
└── _site/             # Generated site (contains index.html at root)
```

### URL Structure
- Root: `/` → `index.html` (Home page)
- Installation: `/installation` → `installation.html`
- Configuration: `/configuration` → `configuration.html`
- Usage: `/usage` → `usage.html`
- QMK Integration: `/qmk-integration` → `qmk-integration.html`
- Troubleshooting: `/troubleshooting` → `troubleshooting.html`
- Advanced: `/advanced` → `advanced.html`
- Examples: `/examples` → `examples.html`

## Components and Interfaces

### Jekyll Configuration (_config.yml)
```yaml
title: QMKonnect
description: Cross-platform window activity notifier for QMK keyboards
remote_theme: just-the-docs/just-the-docs

# Site settings for GitHub Pages
url: https://dabstractor.github.io
baseurl: /qmkonnect

# Navigation structure
nav_external_links:
  - title: GitHub Repository
    url: https://github.com/dabstractor/qmkonnect
  - title: Download Releases
    url: https://github.com/dabstractor/qmkonnect/releases

# Just the Docs theme settings
search_enabled: true
color_scheme: dark
enable_copy_code_button: true

# Footer
footer_content: "Copyright &copy; 2024 QMKonnect. Distributed under the MIT License."
```

### Home Page (index.md)
- **Layout**: `default` (compatible with Just the Docs theme)
- **Navigation Order**: 1 (first in navigation)
- **Content**: Project overview, key features, quick start guide
- **Call-to-Action**: Prominent "Get Started" and "View on GitHub" buttons

### Page Front Matter Structure
Each documentation page should have consistent front matter:
```yaml
---
title: Page Title
layout: default
nav_order: X
---
```

## Data Models

### Navigation Data Model
```yaml
pages:
  - title: "Home"
    url: "/"
    nav_order: 1
  - title: "Installation"
    url: "/installation"
    nav_order: 2
  - title: "Configuration"
    url: "/configuration"
    nav_order: 3
  # ... etc
```

### Site Configuration Model
```yaml
site:
  title: "QMKonnect"
  description: "Cross-platform window activity notifier for QMK keyboards"
  baseurl: "/qmkonnect"
  url: "https://dabstractor.github.io"
  theme: "just-the-docs/just-the-docs"
```

## Error Handling

### Jekyll Build Errors
- **Missing Layout Warning**: Acceptable for remote themes (layouts exist in theme)
- **Broken Links**: Validate all internal links use proper `{{ site.baseurl }}` prefix
- **Missing Files**: Ensure all referenced pages exist and have proper front matter

### Runtime Errors
- **404 Errors**: Implement proper fallback for missing pages
- **Navigation Errors**: Ensure all navigation links are properly formatted
- **Asset Loading**: Verify CSS/JS assets load correctly with baseurl

### GitHub Pages Deployment
- **Build Failures**: Monitor GitHub Actions for build errors
- **Routing Issues**: Test all URLs work correctly with baseurl
- **Theme Loading**: Verify remote theme loads properly

## Testing Strategy

### Local Testing
1. **Build Test**: `bundle exec jekyll build` should complete successfully
2. **Serve Test**: `bundle exec jekyll serve` should serve site without errors
3. **Navigation Test**: All navigation links should work locally
4. **Content Test**: All pages should display content correctly

### GitHub Pages Testing
1. **Deployment Test**: GitHub Actions should build and deploy successfully
2. **URL Test**: All URLs should work with the `/qmkonnect` baseurl
3. **Navigation Test**: Navigation should work on deployed site
4. **External Links**: GitHub and release links should work correctly

### Validation Checklist
- [ ] Root URL `/` displays home page
- [ ] All navigation links work correctly
- [ ] GitHub Pages deployment succeeds
- [ ] All pages have proper front matter
- [ ] Internal links use correct baseurl
- [ ] External links work properly
- [ ] Search functionality works
- [ ] Mobile responsiveness works
- [ ] Dark theme displays correctly

## Implementation Notes

### Critical Fixes Needed
1. **Index Page**: Ensure `index.md` has `nav_order: 1` and `layout: default`
2. **Navigation Links**: All internal links must use `{{ site.baseurl }}/page-name` format
3. **Jekyll Config**: Verify baseurl and url are correctly set for GitHub Pages
4. **Page Order**: Ensure all pages have proper `nav_order` values

### Just the Docs Theme Requirements
- Use `layout: default` for all pages
- Set `nav_order` for navigation ordering
- Use theme-compatible front matter
- Follow theme's link formatting conventions

### GitHub Pages Deployment
- Site will be available at `https://dabstractor.github.io/qmkonnect/`
- All internal links must account for the `/qmkonnect` baseurl
- GitHub Actions workflow handles automatic deployment
- Changes to `docs/` directory trigger rebuilds