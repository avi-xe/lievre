# Configuration

Lièvre is configured via environment variables. Create a `.env` file in the project root.

## Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `JWT_SECRET` | Secret key for JWT tokens | `openssl rand -hex 32` |
| `DOMAIN` | Your server's domain | `lievre.example.com` |
| `DATABASE_URL` | SQLite database path | `sqlite:///app/data/lievre.db` |

## Optional Variables

### Server

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `lievre=info` | Log level (`debug`, `info`, `warn`, `error`) |
| `PORT` | `3000` | API server port (internal) |
| `BIND_ADDR` | `0.0.0.0:3000` | API server bind address |

### Federation

| Variable | Default | Description |
|----------|---------|-------------|
| `FEDERATION_ENABLED` | `true` | Enable/disable federation |
| `INSTANCE_NAME` | `Lièvre` | Display name for your instance |
| `INSTANCE_DESCRIPTION` | | Short description for discovery |
| `REGISTRATION_MODE` | `open` | `open`, `invite`, or `closed` |

### Database

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:///app/data/lievre.db` | Database connection string |
| `DATABASE_POOL_SIZE` | `10` | Connection pool size |

### Worker

| Variable | Default | Description |
|----------|---------|-------------|
| `WORKER_POLL_INTERVAL` | `5` | Seconds between job polls |
| `WORKER_MAX_RETRIES` | `3` | Max retries for failed jobs |

## Example `.env` File

```bash
# Security
JWT_SECRET=a1b2c3d4e5f6...

# Domain
DOMAIN=lievre.example.com
DATABASE_URL=sqlite:///app/data/lievre.db

# Logging
RUST_LOG=lievre=info

# Federation
FEDERATION_ENABLED=true
INSTANCE_NAME=My Lièvre
REGISTRATION_MODE=open
```

## Docker Compose Variables

Override Docker-specific settings in `docker-compose.yml`:

```yaml
services:
  app:
    environment:
      - JWT_SECRET=${JWT_SECRET}
      - DOMAIN=${DOMAIN}
      - DATABASE_URL=${DATABASE_URL}
    volumes:
      - ./data:/app/data

  caddy:
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile
```

## Caddy Configuration

Edit `Caddyfile` for your domain:

```
{$DOMAIN:localhost} {
    reverse_proxy app:3000
}
```

For multiple domains or custom TLS:

```
lievre.example.com {
    reverse_proxy app:3000
}

api.lievre.example.com {
    reverse_proxy app:3000
}
```

## Environment-Specific Configs

### Development

```bash
RUST_LOG=lievre=debug
DOMAIN=localhost
DATABASE_URL=sqlite:///app/data/lievre.db
```

### Production

```bash
RUST_LOG=lievre=info
DOMAIN=lievre.example.com
DATABASE_URL=sqlite:///app/data/lievre.db
REGISTRATION_MODE=open
JWT_SECRET=<generate with openssl rand -hex 32>
```

### Private Instance

```bash
RUST_LOG=lievre=info
DOMAIN=private-lievre.example.com
DATABASE_URL=sqlite:///app/data/lievre.db
REGISTRATION_MODE=closed
FEDERATION_ENABLED=true
```

---

**See also:** [Private Instance](Private-Instance.md) | [Docker Deployment](Docker-Deployment.md)
