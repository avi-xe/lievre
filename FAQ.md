# Lièvre — Frequently Asked Questions

## Getting Started

### How do I install Lièvre?

**Option 1: Docker Compose (Recommended)**

```bash
git clone https://github.com/your-org/lievre.git
cd lievre
cp .env.example .env
docker compose up -d
```

Lièvre will be available at `http://localhost:3000`.

**Option 2: Local Development**

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/your-org/lievre.git
cd lievre
cargo build

# Run
cargo run
```

### What are the system requirements?

- **Minimum:** 1 CPU, 512MB RAM, 1GB disk
- **Recommended:** 2 CPU, 2GB RAM, 10GB disk
- **OS:** Linux, macOS, Windows (via WSL)

---

## User Account

### How do I create an account?

**Via API:**

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "you@example.com",
    "username": "yourname",
    "password": "securepassword"
  }'
```

**Response:**
```json
{
  "id": "abc123",
  "email": "you@example.com",
  "username": "yourname",
  "display_name": null,
  "avatar_url": null,
  "created_at": "2024-01-15T10:30:00Z"
}
```

### How do I log in?

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "you@example.com",
    "password": "securepassword"
  }'
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### How do I use the JWT token?

Include the token in the `Authorization` header for protected endpoints:

```bash
curl http://localhost:3000/api/users/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

---

## Activities

### How do I create an activity?

```bash
curl -X POST http://localhost:3000/api/activities \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "activity_type": "ride",
    "title": "Morning Ride",
    "started_at": "2024-01-15T08:00:00Z",
    "duration_seconds": 3600,
    "distance_meters": 50000,
    "elevation_gain_meters": 500,
    "visibility": "public"
  }'
```

**Activity Types:**
- `ride` — Road cycling
- `run` — Running
- `swim` — Swimming
- `walk` — Walking
- `hike` — Hiking
- `virtual_ride` — Indoor cycling (trainer)

**Visibility Options:**
- `public` — Visible to everyone
- `followers` — Visible to followers only (default)
- `private` — Visible only to you

### How do I list my activities?

```bash
curl "http://localhost:3000/api/activities?limit=10&offset=0" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### How do I get a specific activity?

```bash
curl http://localhost:3000/api/activities/ACTIVITY_ID \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### How do I update an activity?

```bash
curl -X PUT http://localhost:3000/api/activities/ACTIVITY_ID \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title",
    "visibility": "public"
  }'
```

### How do I delete an activity?

```bash
curl -X DELETE http://localhost:3000/api/activities/ACTIVITY_ID \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

## Routes

### How do I add a route to an activity?

When creating an activity, you can include route coordinates:

```bash
curl -X POST http://localhost:3000/api/activities \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "activity_type": "ride",
    "title": "Ride with Route",
    "started_at": "2024-01-15T08:00:00Z",
    "route": {
      "coordinates": [
        [13.404954, 52.520008],
        [13.405101, 52.520212],
        [13.405200, 52.520300]
      ],
      "elevation_data": [34.0, 35.2, 36.0]
    }
  }'
```

**Coordinate Format:** `[longitude, latitude]` or `[longitude, latitude, elevation]`

### How do I get a route as GeoJSON?

```bash
curl http://localhost:3000/api/activities/ACTIVITY_ID/route \
  -H "Authorization: Bearer YOUR_TOKEN"
```

**Response:**
```json
{
  "type": "LineString",
  "coordinates": [
    [13.404954, 52.520008],
    [13.405101, 52.520212],
    [13.405200, 52.520300]
  ]
}
```

This is standard GeoJSON that can be displayed on any map (Leaflet, Mapbox, etc.).

---

## API Reference

### Authentication Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Create account |
| POST | `/api/auth/login` | Get JWT token |

### User Endpoints

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/users/me` | Yes | Get current user |

### Activity Endpoints

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/activities` | Yes | Create activity |
| GET | `/api/activities` | Yes | List user's activities |
| GET | `/api/activities/:id` | Yes | Get activity |
| PUT | `/api/activities/:id` | Yes | Update activity |
| DELETE | `/api/activities/:id` | Yes | Delete activity |
| GET | `/api/activities/:id/route` | Yes | Get route as GeoJSON |

### Health Check

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/health` | No | Server health check |

---

## Development

### How do I run tests?

```bash
cargo test --workspace
```

### How do I add a new migration?

```bash
# Create new migration file
touch migrations/$(date +%Y%m%d%H%M%S)_description.sql

# Run migrations
cargo run -- migrate
```

### How do I check code coverage?

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --workspace
```

### Project structure?

```
lievre/
├── crates/
│   ├── core/          # Domain logic, repositories
│   ├── api/           # Axum HTTP server
│   ├── federation/    # ActivityPub (planned)
│   └── shared/        # Common types, error handling
├── migrations/        # SQL migrations
├── docker-compose.yml
└── Cargo.toml         # Workspace config
```

---

## Troubleshooting

### "database is locked" error

SQLite allows only one writer at a time. If you see this error:

1. Ensure WAL mode is enabled (it is by default)
2. Check for long-running transactions
3. Reduce concurrent write operations

### "invalid credentials" on login

- Ensure you're using the email, not username
- Passwords are case-sensitive
- Check for typos

### Server won't start

1. Check if port 3000 is already in use
2. Verify `.env` file exists (copy from `.env.example`)
3. Check logs: `RUST_LOG=debug cargo run`

### How do I reset the database?

```bash
rm data/lievre.db
cargo run  # Will recreate on startup
```

---

## Community

### Where can I get help?

- **GitHub Issues:** Report bugs or request features
- **Discord:** Join our community server (link in README)
- **Matrix:** #lievre:matrix.org

### How do I contribute?

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Write tests (TDD)
4. Ensure all tests pass (`cargo test --workspace`)
5. Submit a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

*Last updated: 2026-08-10*
