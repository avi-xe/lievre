# Docker Deployment

Production deployment using Docker Compose.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Caddy     │────▶│    App      │────▶│   SQLite    │
│  :80/:443   │     │   :3000     │     │  lievre.db  │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                    ┌──────▼──────┐
                    │    Redis    │
                    │   :6379     │
                    └─────────────┘
```

## Services

| Service | Image | Purpose |
|---------|-------|---------|
| **caddy** | `lievre-caddy` | Reverse proxy, TLS, static files |
| **app** | `lievre-app` | Rust API server |
| **redis** | `redis:7-alpine` | Cache, future job queue |

## Quick Deploy

```bash
# Clone
git clone https://github.com/avi-xe/lievre.git
cd lievre

# Configure
cp .env.example .env
# Edit .env with your settings

# Build and start
docker compose up -d
```

## Building Images

```bash
# Build all
docker compose build

# Build specific service
docker compose build app
docker compose build caddy

# No cache (rebuild from scratch)
docker compose build --no-cache
```

## Managing Services

```bash
# Start
docker compose up -d

# Stop
docker compose down

# Stop and remove volumes (⚠️ destroys data)
docker compose down -v

# View logs
docker compose logs -f app
docker compose logs -f caddy

# Restart
docker compose restart app

# Check status
docker compose ps
```

## Environment Variables

See [Configuration](Configuration.md) for all options.

Key variables:

```bash
# .env
JWT_SECRET=<your-secret>
DOMAIN=lievre.example.com
DATABASE_URL=sqlite:///app/data/lievre.db
RUST_LOG=lievre=info
```

## Volumes

| Volume | Purpose |
|--------|---------|
| `lievre-app-data` | SQLite database |
| `lievre-caddy-data` | TLS certificates |
| `lievre-caddyconfig` | Caddy configuration |

### Backup

```bash
# Backup database
docker cp lievre-app-1:/app/data/lievre.db ./backups/lievre-$(date +%Y%m%d).db
```

### Restore

```bash
docker compose down
docker cp ./backups/lievre-20260814.db lievre-app-1:/app/data/lievre.db
docker compose up -d
```

## Networking

### Ports

| Port | Service | Purpose |
|------|---------|---------|
| 80 | Caddy | HTTP (redirects to HTTPS) |
| 443 | Caddy | HTTPS |
| 3000 | App | Internal API (not exposed) |
| 6379 | Redis | Internal cache (not exposed) |

### Exposing Internally

To access the API directly (development):

```yaml
# docker-compose.yml
services:
  app:
    ports:
      - "3000:3000"
```

## Health Checks

```bash
# App health
curl http://localhost/health

# Expected response
OK
```

## TLS/HTTPS

Caddy automatically obtains and renews Let's Encrypt certificates when:

1. Your domain points to the server
2. Ports 80 and 443 are open
3. The `Caddyfile` uses your domain

### Custom Certificates

For custom/self-signed certificates:

```
lievre.example.com {
    tls /path/to/cert.pem /path/to/key.pem
    reverse_proxy app:3000
}
```

## Scaling

### Single Server (Current)

One server handles all traffic. Sufficient for most instances.

### Future: Multi-Server

- Separate API and worker processes
- PostgreSQL for better concurrency
- Redis for job queue
- Load balancer in front

## Monitoring

### Logs

```bash
# Follow all logs
docker compose logs -f

# App-specific logs
docker compose logs -f app

# Search logs
docker compose logs app | grep "error"
```

### Metrics

Currently no metrics endpoint. Planned for future:

- Request latency
- Activity count
- Federation delivery rate
- Error rates

---

**See also:** [Configuration](Configuration.md) | [Private Instance](Private-Instance.md)
