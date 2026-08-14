# Setting Up a Private Instance

Deploy your own Lièvre server and connect it to the fediverse.

## Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| **CPU** | 1 vCPU | 2 vCPU |
| **RAM** | 1 GB | 2 GB |
| **Disk** | 10 GB SSD | 20 GB SSD |
| **OS** | Ubuntu 22.04+ / Debian 12+ | Same |
| **Docker** | ≥ 24.0 | Latest |

## 1. Server Setup

### Install Docker

```bash
# Ubuntu/Debian
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
newgrp docker
```

### Install Docker Compose

```bash
sudo apt install docker-compose-plugin
```

### Open Firewall Ports

```bash
# HTTP and HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

## 2. Deploy Lièvre

### Clone the Repository

```bash
git clone https://github.com/avi-xe/lievre.git
cd lievre
```

### Configure Environment

Create a `.env` file:

```bash
# Required: Change these for production!
JWT_SECRET=$(openssl rand -hex 32)
DOMAIN=lievre.yourdomain.com
DATABASE_URL=sqlite:///app/data/lievre.db

# Optional
RUST_LOG=lievre=info
```

### Configure Caddy (Reverse Proxy + TLS)

Edit `Caddyfile`:

```
{$DOMAIN:localhost} {
    reverse_proxy app:3000
}
```

### Start Services

```bash
docker compose up -d
```

## 3. Point Your Domain

Create DNS records:

```
A     lievre.yourdomain.com     → YOUR_SERVER_IP
AAAA  lievre.yourdomain.com     → YOUR_SERVER_IPV6 (optional)
```

Caddy will automatically obtain a Let's Encrypt TLS certificate.

## 4. Verify Federation

### Test WebFinger

```bash
curl -s "https://lievre.yourdomain.com/.well-known/webfinger?resource=acct:alice@lievre.yourdomain.com"
```

You should see a JSON response with the actor URL.

### Test Actor Endpoint

```bash
curl -s -H "Accept: application/activity+json" "https://lievre.yourdomain.com/users/alice"
```

You should see a Person actor JSON-LD.

### Test from Mastodon

1. Go to your Mastodon instance
2. Search for: `@alice@lievre.yourdomain.com`
3. Click **Follow**

Your Mastodon will receive Exercise objects from Lièvre!

## 5. Private vs Public Instances

| Feature | Private Instance | Public Instance |
|---------|-----------------|-----------------|
| **Registration** | Open / Invite / Closed | Open |
| **Federation** | Full (all servers) | Full |
| **Discoverability** | Optional (opt-in to Fedi.Directory) | Visible |
| **Data** | Your server only | Shared across fediverse |
| **Admin** | You | Instance admin |

### Making Your Instance Private

Set `REGISTRATION_MODE=closed` in `.env` to prevent new registrations:

```bash
REGISTRATION_MODE=closed
```

Or use `invite` mode to require invite codes.

## 6. Connecting to the Public Lièvre Instance

If you don't want to self-host, you can join the public Lièvre instance:

1. Visit **https://lievre.example.com** (public instance URL)
2. Register an account
3. Start tracking activities

### Following from Your Existing Fediverse Account

You don't need a Lièvre account to follow athletes! From Mastodon, Lemmy, or any ActivityPub server:

1. Search for: `@username@lievre.example.com`
2. Click **Follow**
3. Activities appear in your timeline

See [Federation Guide](Federation-Guide.md) for details.

## 7. Backup & Recovery

### Backup the Database

```bash
# Copy the SQLite database
docker compose exec app cp /app/data/lievre.db /app/data/lievre.db.bak
docker cp lievre-app-1:/app/data/lievre.db.bak ./backups/lievre-$(date +%Y%m%d).db
```

### Restore

```bash
docker compose down
docker cp ./backups/lievre-20260814.db lievre-app-1:/app/data/lievre.db
docker compose up -d
```

---

**Next:** [Federation Guide](Federation-Guide.md) — How to connect with the fediverse
