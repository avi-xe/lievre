# Database Management

Lièvre uses SQLite for persistent storage. This guide covers common database operations.

## Location

The database file is at:

```
./data/lievre.db
```

Inside Docker, it's at `/app/data/lievre.db`.

## Migrations

### Running Migrations

Migrations run automatically on app startup. No manual action needed.

### Migration Files

Located in `migrations/`:

```
migrations/
├── 20240101000000_create_users.sql
├── 20240101000001_create_activities.sql
├── 20240101000002_create_routes.sql
├── 20260810190000_create_jobs.sql
├── 20260810191000_create_activity_stats.sql
├── 20260810200000_create_follows.sql
├── 20260811000000_add_federation_fields.sql
├── 20260812200000_create_notifications.sql
└── 20260813000000_add_like_federation_fields.sql
```

### Creating a New Migration

1. Create a new file: `migrations/YYYYMMDDHHMMSS_description.sql`
2. Write the SQL
3. The migration runs on next app startup

Example:

```sql
-- Add new column to activities
ALTER TABLE activities ADD COLUMN gear_id TEXT REFERENCES gear(id);
```

## Backup

### Full Backup

```bash
# Stop the app first (prevents corruption)
docker compose stop app

# Copy the database
docker cp lievre-app-1:/app/data/lievre.db ./backups/lievre-$(date +%Y%m%d).db

# Restart
docker compose start app
```

### Hot Backup (WAL mode)

SQLite WAL mode allows hot backups without stopping:

```bash
# Use sqlite3 to backup
docker compose exec app sqlite3 /app/data/lievre.db ".backup /app/data/lievre-backup.db"
docker cp lievre-app-1:/app/data/lievre-backup.db ./backups/
```

### Automated Backup

Cron job example:

```bash
# /etc/cron.d/lievre-backup
0 2 * * * root docker compose -H /path/to/lievre exec -T app sqlite3 /app/data/lievre.db ".backup /app/data/lievre-backup.db" && docker cp lievre-app-1:/app/data/lievre-backup.db /backups/lievre-$(date +\%Y\%m\%d).db
```

## Restore

```bash
# Stop the app
docker compose stop app

# Replace the database
docker cp ./backups/lievre-20260814.db lievre-app-1:/app/data/lievre.db

# Restart
docker compose start app
```

## Common Queries

### Check Database Size

```bash
docker compose exec app du -h /app/data/lievre.db
```

### List All Users

```bash
docker compose exec app sqlite3 /app/data/lievre.db "SELECT id, username, email FROM users;"
```

### Count Activities

```bash
docker compose exec app sqlite3 /app/data/lievre.db "SELECT COUNT(*) FROM activities;"
```

### Check Federation Status

```bash
docker compose exec app sqlite3 /app/data/lievre.db "SELECT COUNT(*) FROM actor_follows;"
```

### Find Orphaned Records

```bash
# Activities without users
docker compose exec app sqlite3 /app/data/lievre.db \
  "SELECT a.id FROM activities a LEFT JOIN users u ON a.user_id = u.id WHERE u.id IS NULL;"
```

## Schema

### Core Tables

```sql
-- Users
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Activities
CREATE TABLE activities (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    activity_type TEXT NOT NULL,
    title TEXT,
    description TEXT,
    started_at TEXT NOT NULL,
    duration_seconds INTEGER,
    distance_meters REAL,
    elevation_gain_meters REAL,
    visibility TEXT DEFAULT 'followers',
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Routes
CREATE TABLE routes (
    id TEXT PRIMARY KEY,
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    coordinates TEXT NOT NULL,  -- JSON array
    elevation_data TEXT,        -- JSON array
    created_at TEXT DEFAULT (datetime('now'))
);

-- Likes (with federation fields)
CREATE TABLE likes (
    id TEXT PRIMARY KEY,
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    remote_actor_url TEXT,
    object_url TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(activity_id, user_id),
    UNIQUE(activity_id, remote_actor_url)
);
```

## Troubleshooting

### Database Locked

```bash
# Check for busy connections
docker compose exec app sqlite3 /app/data/lievre.db "PRAGMA wal_checkpoint(FULL);"
```

### Corrupt Database

```bash
# Try to recover
docker compose exec app sqlite3 /app/data/lievre.db ".recover" > recovered.sql
# Create new database
docker compose exec app sqlite3 /app/data/lievre-new.db < recovered.sql
```

### Migration Failed

Check the migration file for syntax errors. SQLite is strict about certain operations.

---

**See also:** [Configuration](Configuration.md) | [Docker Deployment](Docker-Deployment.md)
