# Quick Start

Get Lièvre running locally in 5 minutes with Docker.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) ≥ 24.0
- [Docker Compose](https://docs.docker.com/compose/install/) ≥ 2.20

## 1. Clone and Start

```bash
git clone https://github.com/avi-xe/lievre.git
cd lievre
docker compose up -d
```

Wait for the build to finish (first run takes ~5 minutes). You'll see:

```
Container lievre-redis-1   Healthy
Container lievre-app-1     Started
Container lievre-caddy-1   Started
```

## 2. Open the App

Visit **http://localhost** in your browser.

## 3. Create an Account

1. Click **Register**
2. Enter your email, username, and password
3. You're in!

## 4. Upload Your First Activity

### Option A: Manual Entry

1. Click **+ New** in the nav bar
2. Fill in the form: activity type, title, start time, distance, duration
3. Click **Create Activity**

### Option B: Upload a GPX File

1. Click **+ New**
2. Drag and drop a `.gpx` file onto the upload area
3. Click **Upload**
4. Your activity appears with a map and stats

## 5. Explore

- **Feed** — See activities from people you follow
- **Users** — Discover other athletes
- **Notifications** — Check for follows, likes, and comments

## What's Next?

- [Setting Up a Private Instance](Private-Instance.md) — Deploy your own server
- [Connecting from the Fediverse](Federation-Guide.md) — Follow Lièvre users from Mastodon
- [Creating Activities](Creating-Activities.md) — Detailed activity creation guide

---

**Troubleshooting:**

- **Build fails?** Run `docker compose down -v && docker compose build --no-cache && docker compose up -d`
- **Port 80 in use?** Change the Caddy port in `docker-compose.yml`: `ports: ["8080:80"]`
- **Database errors?** Delete the `data/` folder and restart: `rm -rf data/ && docker compose up -d`
