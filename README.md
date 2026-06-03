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

## Backup

The SQLite database is stored in the `app-data` Docker volume. To back it up:

```bash
# Copy database out of the volume
docker run --rm -v news-site_app-data:/data -v $(pwd):/backup alpine \
  cp /data/news.db /backup/news-$(date +%Y%m%d).db
```

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CLI_TOKEN` | *(required)* | Secret token for CLI authentication |
| `DATABASE_URL` | `sqlite:///data/news.db` | SQLite database path |
| `VALKEY_URL` | `redis://valkey:6379` | Valkey/Redis connection URL |
| `SERVER_PORT` | `3000` | Port the server listens on |
| `RUST_LOG` | `info` | Log level |
