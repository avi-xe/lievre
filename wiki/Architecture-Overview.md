# Architecture Overview

Lièvre is built with a layered architecture using Rust for the backend and React for the frontend.

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Backend** | Rust + Axum | REST API, federation, business logic |
| **Frontend** | React + TypeScript | Web UI |
| **Database** | SQLite | Persistent storage |
| **Proxy** | Caddy | Reverse proxy, TLS, static files |
| **Cache** | Redis | Sessions, job queue |

## Crate Structure

```
lievre/
├── crates/
│   ├── api/              # HTTP handlers, routing
│   │   └── src/
│   │       ├── main.rs         # Server entry, route registration
│   │       ├── auth.rs         # Register, login, JWT
│   │       ├── activities.rs   # CRUD for activities
│   │       ├── social.rs       # Follow, like, comment
│   │       ├── feed.rs         # Activity feed
│   │       ├── federation.rs   # WebFinger, inbox, outbox
│   │       ├── geojson.rs      # GeoJSON endpoint
│   │       ├── import.rs       # File upload handlers
│   │       ├── notifications.rs # Notification endpoints
│   │       └── worker.rs       # Background job processing
│   │
│   ├── core/             # Domain logic, no HTTP
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── activity.rs     # Activity model + queries
│   │       ├── route.rs        # Route model + GeoJSON
│   │       ├── user.rs         # User model
│   │       ├── social.rs       # Follow, like, comment queries
│   │       ├── job.rs          # Job queue
│   │       ├── notification.rs # Notification model
│   │       └── import/
│   │           ├── gpx.rs      # GPX parser
│   │           ├── tcx.rs      # TCX parser
│   │           ├── fit.rs      # FIT parser
│   │           ├── batch.rs    # ZIP batch import
│   │           └── strava.rs   # Strava CSV parser
│   │
│   ├── federation/       # ActivityPub protocol
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs       # FederationDb, domain config
│   │       ├── activity.rs     # ActivityPub activity types
│   │       ├── exercise.rs     # Exercise object (fedisport)
│   │       ├── context.rs      # JSON-LD context endpoint
│   │       ├── keys.rs         # RSA key generation
│   │       └── delivery.rs     # Async delivery queue
│   │
│   └── shared/           # Common utilities
│       └── src/
│           ├── db.rs           # SQLite pool, migrations
│           └── error.rs        # Error types
│
├── frontend/             # React web app
│   └── src/
│       ├── pages/        # Route components
│       ├── components/   # Reusable UI
│       ├── contexts/     # React contexts (auth)
│       └── lib/          # API client, types
│
├── migrations/           # SQL migrations
├── e2e/                  # Playwright tests
└── docker-compose.yml    # Container orchestration
```

## Request Flow

```
Browser → Caddy (:80/:443) → Axum (:3000) → Handler → Core → SQLite
                                      ↓
                              Federation Worker → Remote Inbox
```

## Key Design Decisions

### SQLite over PostgreSQL (for now)

- Zero configuration for development
- Single-file database, easy to backup
- WAL mode for concurrent reads
- Migration path to PostgreSQL when needed

### Core/API Separation

- `core` has no HTTP dependencies — pure business logic
- `api` handles HTTP concerns (routing, serialization, auth)
- `federation` handles ActivityPub protocol
- This allows testing each layer independently

### Worker Queue in SQLite

- No external queue dependency for MVP
- Polls every 5 seconds for pending jobs
- Handles: GPX processing, stats computation, federation delivery
- Migration path to Redis Streams when needed

## Database Schema (Key Tables)

```
users           → id, email, username, password_hash
activities      → id, user_id, type, title, started_at, visibility
routes          → id, activity_id, coordinates (JSON)
likes           → id, activity_id, user_id, remote_actor_url, object_url
comments        → id, activity_id, user_id, content
follows         → follower_id, following_id, status
notifications   → id, user_id, type, data, read
jobs            → id, job_type, payload, status, attempts
actor_follows   → follower_actor_url, following_actor_url
```

## Federation Architecture

```
Lièvre Instance
     │
     ├── WebFinger Discovery
     │   └── /.well-known/webfinger
     │
     ├── Actor Endpoints
     │   ├── /users/{username}           (Person JSON-LD)
     │   ├── /users/{username}/inbox     (Receive activities)
     │   └── /users/{username}/outbox    (Activity feed)
     │
     ├── Exercise Endpoints
     │   ├── /api/exercises/{id}/route   (GeoJSON)
     │   ├── /api/exercises/{id}/stats   (Metrics)
     │   └── /ns/fedisport               (JSON-LD context)
     │
     └── Delivery Queue
         └── Worker signs + sends to remote inboxes
```

---

**See also:** [API Reference](API-Reference.md) | [How Federation Works](How-Federation-Works.md)
