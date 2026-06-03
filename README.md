# News Site

A self-hosted news content management system with a Next.js frontend, Rust (Axum) API backend, SQLite database, and Valkey/Redis cache.

## What It Does

News Site is a lightweight, single-server news management platform for publishing articles, static pages, and organizing content with tags and categories. It provides:

- A public-facing news site with article listings, category/tag browsing, and static pages
- A management API (authenticated via token) for full CRUD on articles, categories, tags, pages, and site settings
- A CLI tool (`news-cli`) for managing content from the terminal
- Valkey/Redis caching for fast public API responses
- Docker-based deployment with a single `docker compose up`

## Tech Stack

| Layer      | Technology                                      |
|------------|-------------------------------------------------|
| Frontend   | Next.js 14 (React 18), Tailwind CSS, TypeScript |
| Backend    | Rust, Axum 0.7, SQLx 0.7 (SQLite), redis 0.25  |
| CLI        | Rust, Clap 4, Reqwest 0.12                      |
| Database   | SQLite (via SQLx migrations)                    |
| Cache      | Valkey 7 (Redis-compatible)                     |
| Runtime    | Docker (multi-stage build: Node → Rust → Alpine) |
| Testing    | Vitest + React Testing Library (frontend unit), Playwright (E2E), Rust integration tests (backend) |

## Quick Start (Docker)

```bash
git clone https://github.com/hermes98761234/news-site.git && cd news-site
cp .env.example .env
# Edit .env: set CLI_TOKEN to a secure random string
docker compose up -d --build
```

The site is available at `http://your-server:3000`.

## Install from Source

Prerequisites: Node.js 20+, Rust 1.88+, SQLite3.

```bash
# Backend
cd backend
cargo build --release -p server -p cli

# Frontend
cd ../frontend
npm ci
npm run build

# Run the server
DATABASE_URL=sqlite://news.db STATIC_DIR=frontend/out ./backend/target/release/server
```

## CLI Usage (`news-cli`)

The CLI communicates with the management API. Initialize it first:

```bash
news-cli init --token YOUR_CLI_TOKEN
# Optionally specify --base-url if the server is not on localhost:3000
```

### Articles

```bash
news-cli article create --title "Hello World" --author "Alice" --body "# Hello\nFirst post." --category-id 1
news-cli article list                          # list all articles
news-cli article list --status draft           # list drafts only
news-cli article edit <id> --title "New Title" # update an article
news-cli article publish <id>                  # publish a draft
news-cli article archive <id>                  # archive a published article
news-cli article delete <id>                   # delete an article
```

### Categories

```bash
news-cli category create --name "Technology" --slug tech
news-cli category list
news-cli category edit <id> --name "Tech" --description "Tech news"
news-cli category delete tech                  # delete by slug
```

### Tags

```bash
news-cli tag create --name "Rust"
news-cli tag list
news-cli tag delete rust                       # delete by slug
```

### Static Pages

```bash
news-cli page create --title "About" --slug about --body "# About us"
news-cli page list
news-cli page edit <id> --body "Updated content"
news-cli page publish <id>
news-cli page delete <id>
```

### Settings

```bash
news-cli settings show
news-cli settings set site_name "My News"
news-cli settings set site_description "A great news site"
```

### Cache

```bash
news-cli cache flush              # flush all cached responses
news-cli cache flush --key articles  # flush entries matching a pattern
```

### Using the CLI inside Docker

```bash
docker compose exec app news-cli init --token $CLI_TOKEN
docker compose exec app news-cli article list
```

## Project Structure

```
news-site/
├── backend/                  # Rust workspace (common, server, cli)
│   ├── common/               # Shared types (models, DB schemas)
│   ├── server/               # Axum HTTP server
│   │   └── src/
│   │       ├── api/
│   │       │   ├── public/   # Public read-only endpoints
│   │       │   └── manage/   # Authenticated management endpoints
│   │       ├── cache/        # Valkey/Redis caching layer
│   │       ├── db/           # SQLx queries (articles, categories, tags, pages, settings)
│   │       ├── config.rs     # Server configuration
│   │       └── error.rs      # Error handling
│   ├── cli/                  # news-cli management tool
│   │   └── src/
│   │       ├── commands/     # article, category, tag, page, settings, cache
│   │       ├── client.rs     # HTTP client for the management API
│   │       └── config.rs     # CLI configuration (token, base URL)
│   └── migrations/           # SQLite schema migrations (001–006)
├── frontend/                 # Next.js 14 app
│   └── src/
│       ├── app/              # App Router pages
│       │   ├── page.tsx              # Homepage
│       │   ├── articles/page.tsx     # Article listing
│       │   ├── articles/[slug]/      # Single article
│       │   ├── categories/[slug]/    # Category page
│       │   ├── tags/[slug]/          # Tag page
│       │   └── [slug]/              # Static page
│       ├── components/       # UI components (common, layout)
│       ├── lib/              # API client, types
│       └── styles/           # Tailwind CSS
├── docs/                     # Additional documentation
│   └── superpowers/          # Project specs and plans
├── Dockerfile                # Multi-stage build (Node → Rust → Alpine)
├── docker-compose.yml        # Production stack (app + Valkey)
├── docker-compose.test.yml   # Test profile (adds smoke-test container)
└── .env.example              # Environment variable template
```

## API Endpoints

### Public (read-only)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/articles` | List published articles (paginated) |
| GET | `/api/articles/:slug` | Single article by slug |
| GET | `/api/categories` | List categories |
| GET | `/api/categories/:slug/articles` | Articles in a category |
| GET | `/api/tags` | List tags |
| GET | `/api/tags/:slug/articles` | Articles with a tag |
| GET | `/api/pages/:slug` | Static page by slug |
| GET | `/api/settings` | Site settings |

### Management (authenticated, requires `CLI_TOKEN`)

| Method | Path | Description |
|--------|------|-------------|
| * | `/api/manage/articles/*` | CRUD + publish/archive |
| * | `/api/manage/categories/*` | CRUD |
| * | `/api/manage/tags/*` | CRUD |
| * | `/api/manage/pages/*` | CRUD + publish |
| * | `/api/manage/settings/*` | Read/update |
| POST | `/api/manage/cache/flush` | Flush cache |

Management endpoints require an `Authorization: Bearer <CLI_TOKEN>` header.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CLI_TOKEN` | *(required)* | Secret token for management API / CLI auth |
| `DATABASE_URL` | `sqlite://news.db` | SQLite database path |
| `VALKEY_URL` | `redis://valkey:6379` | Valkey/Redis connection URL |
| `SERVER_PORT` | `3000` | Port the server listens on |
| `STATIC_DIR` | `/app/static` | Path to Next.js static output |
| `RUST_LOG` | `info` | Log level (debug, info, warn, error) |
| `NEXT_PUBLIC_API_URL` | `/` | Frontend API base URL (build-time) |

## Testing

```bash
# Frontend unit tests
cd frontend && npm test

# Frontend E2E tests (Playwright)
cd frontend && npx playwright install && npm run test:e2e

# Backend integration tests (Rust)
cd backend && cargo test -p server

# Docker smoke test
docker compose -f docker-compose.test.yml --profile test up --abort-on-container-exit
```

## Updates

```bash
git pull && docker compose up -d --build
```

## Backup

The SQLite database is stored in the `app-data` Docker volume.

```bash
docker run --rm -v news-site_app-data:/data -v $(pwd):/backup alpine \
  cp /data/news.db /backup/news-$(date +%Y%m%d).db
```

## Database Migrations

Migrations run automatically on server startup (SQLx embedded migrations). Six migrations create the schema:

1. `categories` — id, name, slug, description
2. `tags` — id, name, slug
3. `articles` — id, title, slug, excerpt, body, author, status, category_id, timestamps
4. `article_tags` — join table (article ↔ tag)
5. `pages` — id, title, slug, body, status, timestamps
6. `settings` — key/value store for site configuration
