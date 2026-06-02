# News Site — Phase 4: Docker & Deployment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Multi-stage Dockerfile producing a ~50MB Alpine image containing the Rust server binary, news-cli binary, and Next.js static output. docker-compose.yml for VPS deployment with Valkey.

**Architecture:** Three-stage build: Node (Next.js static export) → Rust (server + cli binaries) → Alpine runtime. Single container serves API + static files on port 3000. Valkey as separate container.

**Tech Stack:** Docker multi-stage, Alpine 3.19, Node 20-alpine, Rust 1.78-alpine

**Prerequisite:** Phase 1 (backend) and Phase 3 (frontend) complete.

---

## File Map

```
news-site/
├── Dockerfile
├── docker-compose.yml
├── docker-compose.test.yml
├── .dockerignore
└── .env.example
```

---

### Task 1: Dockerfile

**Files:**
- Create: `Dockerfile`
- Create: `.dockerignore`

- [ ] **Step 1: Write .dockerignore**

```
node_modules
frontend/.next
frontend/out
target
*.md
.git
.env*
docs/
```

- [ ] **Step 2: Write Dockerfile**

```dockerfile
# Stage 1: Build Next.js static output
FROM node:20-alpine AS frontend-build
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
ARG NEXT_PUBLIC_API_URL=/
ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL
RUN npm run build

# Stage 2: Build Rust binaries
FROM rust:1.78-alpine AS backend-build
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY backend/ ./
RUN cargo build --release -p server -p cli

# Stage 3: Minimal runtime image
FROM alpine:3.19
RUN apk add --no-cache ca-certificates sqlite-libs
WORKDIR /app

COPY --from=backend-build /app/target/release/server /usr/local/bin/server
COPY --from=backend-build /app/target/release/news-cli /usr/local/bin/news-cli
COPY --from=frontend-build /app/frontend/out /app/static

RUN mkdir -p /data

ENV DATABASE_URL=/data/news.db
ENV STATIC_DIR=/app/static
ENV SERVER_PORT=3000
ENV RUST_LOG=info

EXPOSE 3000
CMD ["/usr/local/bin/server"]
```

- [ ] **Step 3: Verify Dockerfile syntax**

```bash
docker build --dry-run . 2>&1 || docker build -t news-site:test . --no-cache 2>&1 | head -20
```
Expected: no syntax errors in Dockerfile

- [ ] **Step 4: Commit**

```bash
git add Dockerfile .dockerignore && git commit -m "feat: add multi-stage dockerfile"
```

---

### Task 2: docker-compose files

**Files:**
- Create: `docker-compose.yml`
- Create: `docker-compose.test.yml`
- Create: `.env.example`

- [ ] **Step 1: Write docker-compose.yml**

```yaml
# docker-compose.yml
services:
  app:
    build:
      context: .
      args:
        NEXT_PUBLIC_API_URL: http://localhost:3000
    ports:
      - "3000:3000"
    volumes:
      - ./data:/data
    environment:
      DATABASE_URL: /data/news.db
      VALKEY_URL: redis://valkey:6379
      CLI_TOKEN: ${CLI_TOKEN}
      SERVER_PORT: 3000
      RUST_LOG: ${RUST_LOG:-info}
    depends_on:
      valkey:
        condition: service_healthy
    restart: unless-stopped

  valkey:
    image: valkey/valkey:7-alpine
    volumes:
      - valkey-data:/data
    healthcheck:
      test: ["CMD", "valkey-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5
    restart: unless-stopped

volumes:
  valkey-data:
```

- [ ] **Step 2: Write docker-compose.test.yml**

```yaml
# docker-compose.test.yml
services:
  valkey-test:
    image: valkey/valkey:7-alpine
    ports:
      - "6380:6379"
    healthcheck:
      test: ["CMD", "valkey-cli", "ping"]
      interval: 3s
      timeout: 2s
      retries: 10

  backend-test:
    build:
      context: .
      dockerfile: Dockerfile
      target: backend-build
    command: >
      sh -c "
        cargo test --workspace 2>&1
      "
    working_dir: /app
    environment:
      VALKEY_URL: redis://valkey-test:6379
      CLI_TOKEN: test-token
    depends_on:
      valkey-test:
        condition: service_healthy

  frontend-test:
    image: node:20-alpine
    working_dir: /app/frontend
    volumes:
      - ./frontend:/app/frontend
    command: npm test
```

- [ ] **Step 3: Write .env.example**

```bash
# .env.example
# Copy to .env and fill in values
CLI_TOKEN=change-me-to-a-secure-random-string
RUST_LOG=info
```

- [ ] **Step 4: Commit**

```bash
git add docker-compose.yml docker-compose.test.yml .env.example
git commit -m "feat: add docker-compose for deployment and testing"
```

---

### Task 3: Full build test

- [ ] **Step 1: Build the image**

```bash
docker compose build 2>&1
```
Expected: image builds successfully, final stage ~50MB

- [ ] **Step 2: Start the stack**

```bash
CLI_TOKEN=test-token docker compose up -d
```
Expected: both containers start, `docker compose ps` shows them as healthy

- [ ] **Step 3: Smoke test the API**

```bash
curl -s http://localhost:3000/api/settings | jq .
```
Expected: JSON array with site_name, site_description, site_url keys

- [ ] **Step 4: Smoke test the CLI**

```bash
docker compose exec app news-cli --help
```
Expected: CLI help text printed

- [ ] **Step 5: Smoke test static frontend**

```bash
curl -s http://localhost:3000/ | grep -o '<title>[^<]*</title>'
```
Expected: `<title>News</title>` or the configured site name

- [ ] **Step 6: Tear down**

```bash
docker compose down
```

- [ ] **Step 7: Commit**

```bash
git commit -m "chore: verify full docker build and stack smoke test passes"
```

---

### Task 4: Deploy instructions in README

**Files:**
- Create: `README.md`

- [ ] **Step 1: Write README.md**

```markdown
# News Site

A self-hosted news site. Next.js frontend + Rust API + SQLite + Valkey.

## Quick start (VPS)

```bash
git clone <repo> news-site && cd news-site
cp .env.example .env
# Edit .env: set CLI_TOKEN to a secure random string
docker compose up -d --build
```

The site is now running at `http://your-server:3000`.

## Managing content

From the VPS shell:

```bash
# Initialize CLI config
docker compose exec app news-cli init --token $CLI_TOKEN

# Create a category
docker compose exec app news-cli category create --name "Technology" --slug tech

# Create and publish an article
docker compose exec app news-cli article create \
  --title "Hello World" \
  --author "Alice" \
  --body "# Hello\nFirst post." \
  --category-id 1

docker compose exec app news-cli article list --status draft
docker compose exec app news-cli article publish <id>
```

## Updates

```bash
git pull && docker compose up -d --build
```

## Backups

```bash
# Backup SQLite database
cp data/news.db data/news.db.bak
```
```

- [ ] **Step 2: Commit**

```bash
git add README.md && git commit -m "docs: add deployment readme"
```
