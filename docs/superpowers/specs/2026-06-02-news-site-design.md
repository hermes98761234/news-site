# News Site — Design Spec

**Date:** 2026-06-02  
**Status:** Approved  
**Stack:** Next.js (TypeScript) + Rust (Axum) + SQLite + Valkey + Docker Compose

---

## Overview

A self-hosted news site with a modern minimal design. Public readers browse articles, pages, tags and categories with no authentication required. A small editorial team manages all content via `news-cli` — a plain Rust CLI binary — driven by a Claude Code skill that acts as the natural language agent layer.

---

## Architecture

**Approach:** Monorepo, single Rust binary, Next.js static export.

- Rust binary serves both the REST API and the Next.js static output via `tower-http`
- Next.js builds to static files (`next export`) at build time
- CLI is a separate Rust binary in the same Cargo workspace, calling the REST API
- Claude Code skill (`~/.claude/skills/news-site/SKILL.md`) documents all CLI commands; Claude acts as the intelligent agent on top of the dumb CLI

```
User (natural language) → Claude + news-site skill → news-cli commands → REST API → SQLite + Valkey
Public readers → Next.js static pages → REST API → SQLite + Valkey (cached)
```

---

## Repository Structure

```
news-site/
├── backend/
│   ├── Cargo.toml              # workspace root
│   ├── server/                 # axum HTTP server + REST API
│   │   └── src/
│   │       ├── main.rs
│   │       ├── api/            # route handlers (articles, pages, tags, settings)
│   │       ├── db/             # SQLite queries via sqlx
│   │       ├── cache/          # Valkey integration (redis-rs)
│   │       └── models/         # shared domain types
│   ├── cli/                    # news-cli binary
│   │   └── src/
│   │       ├── main.rs
│   │       └── commands/       # article, page, tag, category, settings, cache
│   └── common/                 # types shared by server and cli
│       └── src/lib.rs
├── frontend/                   # Next.js + TypeScript
│   └── src/
│       ├── app/                # App Router pages
│       ├── components/         # UI components
│       └── lib/                # API client, types
├── docker/
│   ├── Dockerfile              # multi-stage: Node → Rust → alpine runtime
│   └── docker-compose.yml
├── migrations/                 # sqlx migration files
└── tests/                      # integration tests
```

---

## Database Schema (SQLite)

```sql
articles (
  id, slug UNIQUE, title, excerpt, body TEXT,  -- body stored as markdown
  author_name, status CHECK(status IN ('draft','published','archived')),
  category_id REFERENCES categories(id),
  published_at, created_at, updated_at
)

pages (
  id, slug UNIQUE, title, body TEXT,
  status CHECK(status IN ('draft','published')),
  created_at, updated_at
)

categories (
  id, slug UNIQUE, name, description
)

tags (
  id, slug UNIQUE, name
)

article_tags (
  article_id REFERENCES articles(id),
  tag_id REFERENCES tags(id),
  PRIMARY KEY (article_id, tag_id)
)

settings (
  key TEXT UNIQUE, value TEXT  -- site_name, site_description, etc.
)
```

Migrations managed with `sqlx migrate`.

---

## Cache Strategy (Valkey)

| Key pattern              | TTL      | Invalidated by                        |
|--------------------------|----------|---------------------------------------|
| `articles:list:*`        | 5 min    | any article publish/update/archive    |
| `articles:slug:<slug>`   | 1 hour   | article update/archive for that slug  |
| `pages:slug:<slug>`      | 1 hour   | page update for that slug             |
| `tags:list`              | 5 min    | tag create/delete                     |
| `categories:list`        | 5 min    | category create/delete                |
| `feed:homepage`          | 5 min    | any article publish                   |

CLI publish/update/delete commands flush relevant keys immediately via the management API.

---

## REST API (Axum)

### Public endpoints
```
GET /api/articles                   # list published, paginated (?page=&limit=&tag=&category=)
GET /api/articles/:slug             # single article
GET /api/pages/:slug                # single static page
GET /api/categories                 # all categories
GET /api/tags                       # all tags
GET /api/tags/:slug/articles        # articles by tag
GET /api/settings                   # public site settings
```

### Management endpoints (X-CLI-Token header required)
```
POST   /api/manage/articles
PUT    /api/manage/articles/:id
DELETE /api/manage/articles/:id
POST   /api/manage/articles/:id/publish
POST   /api/manage/articles/:id/archive

POST   /api/manage/pages
PUT    /api/manage/pages/:id
DELETE /api/manage/pages/:id
POST   /api/manage/pages/:id/publish

POST   /api/manage/categories
PUT    /api/manage/categories/:id
DELETE /api/manage/categories/:id

POST   /api/manage/tags
DELETE /api/manage/tags/:id

GET    /api/manage/settings
PUT    /api/manage/settings
```

Management endpoints are protected by a shared secret token (`X-CLI-Token: <token>`) set via `CLI_TOKEN` environment variable. No authentication on public endpoints.

---

## CLI (`news-cli`)

Plain Rust binary with explicit subcommands. No AI inside the binary itself.

```bash
# Articles
news-cli article create --title "..." --author "..." --category <slug> --body "..."
news-cli article edit <slug> --title "..." --body "..."
news-cli article publish <slug>
news-cli article archive <slug>
news-cli article delete <slug>
news-cli article list
news-cli article list --status draft|published|archived

# Pages
news-cli page create --title "..." --slug <slug> --body "..."
news-cli page edit <slug> --title "..." --body "..."
news-cli page publish <slug>
news-cli page delete <slug>
news-cli page list

# Categories
news-cli category create --name "..." --slug <slug> --description "..."
news-cli category list
news-cli category delete <slug>

# Tags
news-cli tag create --name "..."
news-cli tag list
news-cli tag delete <slug>

# Settings
news-cli settings set <key> <value>
news-cli settings show

# Cache
news-cli cache flush
news-cli cache flush --key <pattern>
```

Config at `~/.config/news-cli/config.toml`:
```toml
api_url = "http://localhost:3000"
token   = "your-secret-token"
```

### Claude Code Skill

`~/.claude/skills/news-site/SKILL.md` documents every `news-cli` command. Claude Code acts as the agent:

```
User: "publish the rust article from yesterday"
Claude: news-cli article list --status draft
        → finds match
        → news-cli article publish rust-2025-edition ✓
```

Invocation: user talks to Claude naturally; Claude runs `news-cli` commands.

---

## Frontend (Next.js + TypeScript)

### Pages
```
/                        # homepage — hero + recent articles grid
/articles                # paginated listing, filterable by tag/category
/articles/[slug]         # single article
/categories/[slug]       # articles in category
/tags/[slug]             # articles by tag
/[slug]                  # static pages (about, contact, etc.)
```

### Component Structure
```
components/
├── layout/              # Header, Footer, Nav
├── article/             # ArticleCard, ArticleList, ArticleBody
├── common/              # Pagination, TagBadge, CategoryBadge
└── ui/                  # shadcn/ui base components
```

### Key Decisions
- **App Router** with `output: 'export'` in `next.config.js` for static export; `fetch` + `revalidate` for ISR-style caching at build time
- **Static generation** (`generateStaticParams`) for article and page slugs
- **shadcn/ui + Tailwind CSS** — modern minimal aesthetic
- **react-markdown + rehype-highlight** — markdown rendering with code syntax highlighting
- No client-side auth; all pages fully public

---

## Testing Strategy

### Backend (Rust)
- **Unit tests** (`#[cfg(test)]`): DB query functions, cache key generation, slug utilities
- **Integration tests** (`sqlx::test` macro): each test gets a fresh in-memory SQLite DB
- **API tests**: spin up axum `TestServer`, test all endpoints including management routes
- **Valkey**: mocked with `mockall` in unit tests; real Valkey container in integration tests

### Frontend (Next.js)
- **Component tests**: React Testing Library — ArticleCard, Pagination, TagBadge, etc.
- **Page tests**: full page render tests with mocked API (msw)
- **E2E tests**: Playwright — homepage load, article read flow, tag/category filter

### CI Test Profile
```yaml
# docker-compose.test.yml
services:
  valkey:    # real Valkey for integration tests
  test:      # runs: cargo test + vitest + playwright
```

---

## Docker & Deployment

### Dockerfile (multi-stage)
```
Stage 1 (node:20-alpine):   npm ci + next build → /out
Stage 2 (rust:1.78-alpine): cargo build --release → server + news-cli binaries
Stage 3 (alpine:3.19):      copy /out + binaries → ~50MB final image
```

### docker-compose.yml
```yaml
services:
  app:
    build: .
    ports: ["3000:3000"]
    volumes:
      - ./data:/data          # SQLite DB
    environment:
      DATABASE_URL: /data/news.db
      VALKEY_URL: redis://valkey:6379
      CLI_TOKEN: ${CLI_TOKEN}
    depends_on: [valkey]

  valkey:
    image: valkey/valkey:7-alpine
    volumes:
      - valkey-data:/data

volumes:
  valkey-data:
```

### Deploy Workflow
```bash
git pull
docker compose up -d --build
```

`news-cli` is available on the VPS shell (copied into image, symlinked to `/usr/local/bin`).

---

## Environment Variables

| Variable       | Required | Description                          |
|----------------|----------|--------------------------------------|
| `DATABASE_URL` | yes      | SQLite path, e.g. `/data/news.db`    |
| `VALKEY_URL`   | yes      | Redis-compatible URL for Valkey      |
| `CLI_TOKEN`    | yes      | Shared secret for management API     |
| `SERVER_PORT`  | no       | HTTP port (default: 3000)            |
| `RUST_LOG`     | no       | Log level (default: `info`)          |
