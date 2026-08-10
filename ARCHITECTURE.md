# Lièvre: Federated Fitness Platform Architecture

## Executive Summary

Lièvre (French for "the hare") is a free, open-source, fediverse-native alternative to Strava that lets users track, analyze, and share cycling (and future running/swimming) activities. Named after the cycling term for a pacemaker — the rider who sets the pace for the peloton — Lièvre leverages the **ActivityPub protocol** to federate activities across instances, enabling users to follow athletes on Mastodon, PeerTube, or other ActivityPub-compatible servers.

---

## 1. Competitive Landscape

### 1.1 Direct Competitors (Federated Fitness)

| Project | Status | Stack | Federation | Key Features |
|---------|--------|-------|------------|--------------|
| **[FitPub](https://codeberg.org/fitpub/fitpub)** | Active | Java 25, Spring Boot 4, PostgreSQL/PostGIS | ActivityPub | GPX/FIT/TCX import, route maps, analytics, privacy zones |
| **[Open Pace](https://github.com/myfear/open-pace)** | Sprint-based | Java, Quarkus, Vert.x | ActivityPub | Strava-like features, GPX, segments, leaderboards |
| **[Wanderer](https://wanderer.to/)** | Active | JavaScript/TypeScript | ActivityPub (v0.17+) | Trail database, GPS tracks, route planning, hiking focus |
| **[FitTrackee](https://github.com/SamR1/FitTrackee)** | Active | Python, Flask | None (self-hosted only) | GPX import, statistics, no federation |

### 1.2 Adjacent Projects

| Project | Focus | Relevance |
|---------|-------|-----------|
| **[Fedisport Vocabulary](https://github.com/fedisport/vocabulary)** | ActivityPub extensions for sports | Standardized `Exercise` object type, activity types, metrics |
| **[TrailFed](https://github.com/trailfed/trailfed)** | Federated geo-social protocol | Van-lifers, overlanders, sailors, cyclists |

### 1.3 Key Differentiators for Lièvre

1. **Cycling-first design** — deeper power/FTP analysis, segment leaderboards, trainer integration
2. **Multi-sport from day one** — running and swimming in roadmap
3. **Private + public instance support** — users can join existing instances or self-host
4. **Fedisport vocabulary adoption** — first-class `Exercise` object type for federation
5. **Modern stack** — performance, type safety, developer experience

---

## 2. Technical Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLIENT LAYER                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  PWA        │  │ Desktop     │  │ CLI/Import  │            │
│  │  (React)    │  │ (Browser)   │  │ Tool        │            │
│  │             │  │             │  │             │            │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘            │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          │                                      │
│                    ┌─────▼─────┐                                │
│                    │  API GW   │                                │
│                    │ (Caddy)   │                                │
│                    └─────┬─────┘                                │
└──────────────────────────┼──────────────────────────────────────┘
```
┌──────────────────────────┼──────────────────────────────────────┐
│                    APPLICATION LAYER                            │
│  ┌───────────────────────▼───────────────────────┐              │
│  │              Core Service (Rust)              │              │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────┐│              │
│  │  │ Auth        │  │ Activities  │  │ Social ││              │
│  │  │ (JWT/OAuth) │  │ (CRUD+Import│  │ (Follow││              │
│  │  │             │  │  GPX/FIT)   │  │  Like) ││              │
│  │  └─────────────┘  └─────────────┘  └────────┘│              │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────┐│              │
│  │  │ Analytics   │  │ Gear        │  │ Maps   ││              │
│  │  │ (Stats,PRs) │  │ (Tracking)  │  │ (OSM)  ││              │
│  │  └─────────────┘  └─────────────┘  └────────┘│              │
│  └───────────────────────┬───────────────────────┘              │
│                          │                                      │
│  ┌───────────────────────▼───────────────────────┐              │
│  │           Federation Service (Rust)            │              │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────┐│              │
│  │  │ WebFinger   │  │ ActivityPub │  │ HTTP   ││              │
│  │  │ Discovery   │  │ C2S/S2S     │  │ Sigs   ││              │
│  │  └─────────────┘  └─────────────┘  └────────┘│              │
│  └───────────────────────────────────────────────┘              │
└──────────────────────────┬──────────────────────────────────────┘
                           │
┌──────────────────────────┼──────────────────────────────────────┐
│                     DATA LAYER                                  │
│  ┌───────────────────────▼───────────────────────┐              │
│  │           PostgreSQL + PostGIS                 │              │
│  │  - Users/Actors    - Activities                │              │
│  │  - Routes (LineString)  - Gear                 │              │
│  │  - Social Graph    - Analytics                 │              │
│  └───────────────────────────────────────────────┘              │
│  ┌───────────────────────┐  ┌─────────────────────┐            │
│  │  Redis (Cache/Queue)  │  │ S3/MinIO (Files)    │            │
│  └───────────────────────┘  └─────────────────────┘            │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Technology Stack Decision

| Layer | Choice | Rationale |
|-------|--------|-----------|
| **Core Service** | Rust (Axum/Actix-web) | Performance, safety, type-safety, excellent ecosystem |
| **Federation** | Rust (activitypub-federation crate) | Battle-tested by Lemmy, handles HTTP sigs |
| **Database** | SQLite (dev/plan) → PostgreSQL (prod) | Zero-config for prototyping, seamless migration |
| **Queue** | SQLite WAL (dev) → Redis/NATS (prod) | Same API, scale when needed |
| **Object Storage** | Local FS (dev) → S3/MinIO (prod) | No setup, migrate later |
| **Frontend** | React + TypeScript | Rich interactive maps, charts, modern UX |
| **Mobile** | PWA | No native app, web-first |
| **Maps** | Leaflet + OpenStreetMap tiles | Free, open, no API key required |
| **Charts** | Recharts / D3.js | Performance analytics, elevation profiles |

### 2.3 Phased Storage Strategy

**Planning/Dev Phase (SQLite):**
```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "uuid", "chrono"] }
rusqlite = { version = "0.31", features = ["bundled"] }
```

- Zero configuration, single-file database
- SQLite WAL mode for concurrent reads
- Same SQL dialect (mostly) as PostgreSQL
- Easy to share, backup, and debug

**Migration Path to PostgreSQL:**
```sql
-- SQLite-specific → PostgreSQL equivalents
-- 1. AUTOINCREMENT → SERIAL/BIGSERIAL
-- 2. DATETIME → TIMESTAMPTZ
-- 3. TEXT → VARCHAR(n) where needed
-- 4. Add PostGIS for geospatial (post-migration)
```

**Production Stack (PostgreSQL + PostGIS):**
- Geospatial queries (route storage, nearby activities)
- Full-text search
- Better concurrency for federation

### 2.4 Async Worker Queue Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLIENT LAYER                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  Web App    │  │ PWA         │  │ CLI/Import  │            │
│  │  (React)    │  │ (React)     │  │ Tool        │            │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘            │
│         └────────────────┼────────────────┘                     │
└──────────────────────────┼──────────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────────┐
│                      API LAYER                                  │
│  ┌───────────────────────────────────────────────────┐         │
│  │                  Axum Server                      │         │
│  │  ┌─────────────┐  ┌─────────────┐  ┌───────────┐│         │
│  │  │ Auth        │  │ Activities  │  │ Federation││         │
│  │  │ Handler     │  │ Handler     │  │ Handler   ││         │
│  │  └──────┬──────┘  └──────┬──────┘  └───────────┘│         │
│  │         │                │                        │         │
│  │         │     ┌──────────▼──────────┐             │         │
│  │         │     │  Job Dispatcher     │             │         │
│  │         │     │  (enqueue jobs)     │             │         │
│  │         │     └──────────┬──────────┘             │         │
│  └─────────┼────────────────┼────────────────────────┘         │
└────────────┼────────────────┼──────────────────────────────────┘
             │                │
             │    ┌───────────▼───────────┐
             │    │      MESSAGE QUEUE     │
             │    │                        │
             │    │  Dev: SQLite WAL       │
             │    │  Prod: Redis Streams   │
             │    │         or NATS        │
             │    │                        │
             │    └───────────┬───────────┘
             │                │
┌────────────┼────────────────┼──────────────────────────────────┐
│            │    WORKER POOL │                                  │
│  ┌─────────▼────────────────▼─────────────────────────┐       │
│  │              Job Workers                            │       │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐│       │
│  │  │ GPX Parser  │  │ FIT Parser  │  │ TCX Parser  ││       │
│  │  │ Worker      │  │ Worker      │  │ Worker      ││       │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘│       │
│  │         │                │                │        │       │
│  │         └────────────────┼────────────────┘        │       │
│  │                          │                          │       │
│  │              ┌───────────▼───────────┐              │       │
│  │              │   Activity Processor  │              │       │
│  │              │  - Extract route      │              │       │
│  │              │  - Compute stats      │              │       │
│  │              │  - Generate map       │              │       │
│  │              │  - Update gear        │              │       │
│  │              │  - Federation         │              │       │
│  │              └───────────┬───────────┘              │       │
│  └──────────────────────────┼──────────────────────────┘       │
└─────────────────────────────┼───────────────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────────────┐
│                     NOTIFICATION LAYER                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  WebSocket Server (for real-time UI updates)            │   │
│  │  - Activity processed → notify user                     │   │
│  │  - New follower → notify user                           │   │
│  │  - Like/comment → notify user                           │   │
│  └─────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Federation Outbox (async delivery)                     │   │
│  │  - Create activity → deliver to followers' inboxes      │   │
│  │  - Like activity → deliver to activity author           │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 2.5 Job Types and Flow

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    // File Processing
    ProcessGpx { activity_id: Uuid, file_path: String },
    ProcessFit { activity_id: Uuid, file_path: String },
    ProcessTcx { activity_id: Uuid, file_path: String },
    ProcessZip { activity_id: Uuid, file_path: String },
    
    // Activity Processing
    ComputeStats { activity_id: Uuid },
    GenerateRouteGeoJson { activity_id: Uuid },
    UpdateGearMileage { activity_id: Uuid },
    
    // Federation
    DeliverActivity { activity_id: Uuid, recipient_inbox: String },
    DeliverLike { like_id: Uuid, recipient_inbox: String },
    DeliverFollow { follow_id: Uuid, recipient_inbox: String },
    
    // Notifications
    NotifyUser { user_id: Uuid, notification_type: String, data: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub job_type: JobType,
    pub status: JobStatus,
    pub attempts: i32,
    pub max_attempts: i32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Retrying,
}
```

### 2.6 Queue Implementation (SQLite for Dev)

```sql
-- jobs table
CREATE TABLE jobs (
    id TEXT PRIMARY KEY,
    job_type TEXT NOT NULL,
    payload TEXT NOT NULL,          -- JSON
    status TEXT DEFAULT 'pending',
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    created_at TEXT DEFAULT (datetime('now')),
    started_at TEXT,
    completed_at TEXT,
    error TEXT,
    next_retry_at TEXT
);

CREATE INDEX idx_jobs_status ON jobs(status, next_retry_at);
CREATE INDEX idx_jobs_pending ON jobs(status, created_at) WHERE status = 'pending';
```

**Worker Polling (dev mode):**
```rust
pub async fn poll_jobs(db: &SqlitePool) -> Result<()> {
    loop {
        // Get next pending job
        let job = sqlx::query_as::<_, Job>(
            "SELECT * FROM jobs 
             WHERE status = 'pending' 
             AND (next_retry_at IS NULL OR next_retry_at <= datetime('now'))
             ORDER BY created_at 
             LIMIT 1 
             FOR UPDATE SKIP LOCKED"
        )
        .fetch_optional(db)
        .await?;

        if let Some(job) = job {
            process_job(db, &job).await?;
        } else {
            // No jobs, wait before polling again
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
```

### 2.7 Why Rust over Java/Go/Python?

| Criterion | Rust | Java | Go | Python |
|-----------|------|------|----|----|
| Performance | ★★★★★ | ★★★★ | ★★★★ | ★★★ |
| Memory Safety | ★★★★★ | ★★★ | ★★★★ | ★★ |
| Type Safety | ★★★★★ | ★★★★ | ★★★★ | ★★★ |
| AP Library | activitypub-federation (Lemmy) | Custom | go-fed | bovine |
| Dev Experience | ★★★ | ★★★★ | ★★★★ | ★★★★★ |
| Binary Size | ★★★★★ | ★★ | ★★★★ | ★ |

Rationale: The `activitypub-federation` crate from Lemmy is battle-tested and handles the complex parts of federation (HTTP signatures, inbox processing, actor resolution). Rust's performance characteristics are ideal for processing large GPX files and serving geospatial queries.

---

## 3. ActivityPub Integration

### 3.1 Core ActivityPub Objects

Following the **fedisport vocabulary** standard:

#### Actor (Person)

```json
{
  "@context": [
    "https://www.w3.org/ns/activitystreams",
    "https://w3id.org/security/v1",
    "https://fedisport.github.io/vocabulary/context.jsonld"
  ],
  "type": "Person",
  "id": "https://example.com/users/alice",
  "inbox": "https://example.com/users/alice/inbox",
  "outbox": "https://example.com/users/alice/outbox",
  "preferredUsername": "alice",
  "name": "Alice Cyclist",
  "summary": "Riding bikes, chasing PRs 🚴‍♀️",
  "icon": {
    "type": "Image",
    "mediaType": "image/jpeg",
    "url": "https://example.com/avatars/alice.jpg"
  },
  "attachment": [
    {
      "type": "PropertyValue",
      "name": "Strava",
      "value": "<a href=\"https://strava.com/athletes/alice\">@alice</a>"
    }
  ],
  "manuallyApprovesFollowers": false,
  "discoverable": true,
  "publicKey": {
    "id": "https://example.com/users/alice#main-key",
    "owner": "https://example.com/users/alice",
    "publicKeyPem": "-----BEGIN PUBLIC KEY-----\n..."
  }
}
```

#### Exercise Object (Fedisport)

```json
{
  "@context": [
    "https://www.w3.org/ns/activitystreams",
    "https://fedisport.github.io/vocabulary/context.jsonld"
  ],
  "type": "Exercise",
  "id": "https://example.com/exercises/01932c4a-5f8e-7000-8a1b-f3e2d1c0b9a8",
  "attributedTo": "https://example.com/users/alice",
  "activityType": "ride",
  "startedAt": "2025-04-10T08:15:00Z",
  "name": "Morning Ride through the Alps",
  "content": "<p>Perfect weather, legs felt strong!</p>",
  "routeUrl": "https://example.com/api/exercises/01932c4a/route",
  "statsUrl": "https://example.com/api/exercises/01932c4a/stats",
  "published": "2025-04-10T09:30:00Z",
  "to": ["https://www.w3.org/ns/activitystreams#Public"],
  "cc": ["https://example.com/users/alice/followers"]
}
```

#### Create Activity (Delivery)

```json
{
  "@context": "https://www.w3.org/ns/activitystreams",
  "type": "Create",
  "actor": "https://example.com/users/alice",
  "object": { "...Exercise object above..." },
  "to": ["https://www.w3.org/ns/activitystreams#Public"],
  "cc": ["https://example.com/users/alice/followers"]
}
```

### 3.2 Federation Endpoints

| Endpoint | Purpose |
|----------|---------|
| `/.well-known/webfinger` | User discovery (`acct:user@domain`) |
| `/users/{username}` | Actor profile (JSON-LD) |
| `/users/{username}/inbox` | S2S: Receive activities |
| `/users/{username}/outbox` | C2S/S2S: Activity feed |
| `/users/{username}/followers` | Follower collection |
| `/users/{username}/following` | Following collection |
| `/api/exercises/{id}/route` | GeoJSON route data |
| `/api/exercises/{id}/stats` | Fitness metrics |

### 3.3 Supported Activity Types

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ActivityType {
    // Cycling
    Ride,
    GravelRide,
    MountainBikeRide,
    EBikeRide,
    VirtualRide,
    
    // Running
    Run,
    TrailRun,
    VirtualRun,
    
    // Swimming
    Swim,
    
    // Future
    Walk,
    Hike,
    Workout,
}
```

### 3.4 Stats Object (Fedisport)

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExerciseStats {
    pub distance: Option<f64>,        // metres
    pub duration: Option<i64>,        // seconds
    pub elevation_gain: Option<f64>,  // metres
    pub device: Option<String>,
    pub avg_pace: Option<i32>,        // seconds/km
    pub avg_heart_rate: Option<i32>,  // bpm
    pub max_heart_rate: Option<i32>,  // bpm
    pub avg_power: Option<f64>,       // watts
    pub max_power: Option<f64>,       // watts
    pub normalized_power: Option<f64>,// watts
    pub avg_cadence: Option<f64>,     // rpm
    pub avg_speed: Option<f64>,       // m/s
    pub max_speed: Option<f64>,       // m/s
    pub calories: Option<i32>,
}
```

---

## 4. Data Model

### 4.1 Core Entities

```
┌─────────────────┐       ┌─────────────────┐
│     Users       │       │    Actors       │
├─────────────────┤       ├─────────────────┤
│ id (UUID)       │──────▶│ id (UUID)       │
│ email           │       │ user_id         │
│ username        │       │ ap_id (URL)     │
│ password_hash   │       │ inbox_url       │
│ created_at      │       │ outbox_url      │
│ updated_at      │       │ public_key      │
└─────────────────┘       │ private_key     │
                          │ followers_url   │
                          │ following_url   │
                          └────────┬────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
              ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
              │ Followers │  │ Following │  │Activities │
              ├───────────┤  ├───────────┤  ├───────────┤
              │ actor_id  │  │ actor_id  │  │ id (UUID) │
              │ target_id │  │ target_id │  │ user_id   │
              │ accepted  │  │ accepted  │  │ type      │
              └───────────┘  └───────────┘  │ title     │
                                            │ started_at│
                                            │ ...       │
                                            └─────┬─────┘
                                                  │
                                    ┌─────────────┼─────────────┐
                                    │             │             │
                              ┌─────▼─────┐ ┌─────▼─────┐ ┌─────▼─────┐
                              │  Routes   │ │   Stats   │ │   Gear    │
                              ├───────────┤ ├───────────┤ ├───────────┤
                              │ id        │ │ id        │ │ id        │
                              │ activity  │ │ activity  │ │ user_id   │
                              │ geom      │ │ data (JS) │ │ name      │
                              │ (PostGIS) │ │           │ │ type      │
                              └───────────┘ └───────────┘ │ mileage   │
                                                          └───────────┘
```

### 4.2 Database Schema (Key Tables)

```sql
-- Users (local accounts)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(32) UNIQUE NOT NULL,
    display_name VARCHAR(128),
    password_hash VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ActivityPub Actors (both local and remote)
CREATE TABLE actors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    ap_id TEXT UNIQUE NOT NULL,          -- https://domain/users/name
    preferred_username VARCHAR(32),
    name VARCHAR(128),
    summary TEXT,
    inbox_url TEXT NOT NULL,
    outbox_url TEXT,
    followers_url TEXT,
    following_url TEXT,
    public_key TEXT,
    private_key TEXT,                     -- only for local actors
    avatar_url TEXT,
    is_local BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Activities
CREATE TABLE activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    ap_id TEXT UNIQUE,                    -- federated ID
    activity_type VARCHAR(32) NOT NULL,   -- ride, run, swim, etc.
    title VARCHAR(255),
    description TEXT,
    started_at TIMESTAMPTZ NOT NULL,
    duration_seconds INTEGER,
    distance_meters DECIMAL(10,2),
    elevation_gain_meters DECIMAL(8,2),
    avg_heart_rate INTEGER,
    max_heart_rate INTEGER,
    avg_power DECIMAL(6,2),
    max_power DECIMAL(6,2),
    avg_speed DECIMAL(6,2),
    max_speed DECIMAL(6,2),
    calories INTEGER,
    gear_id UUID REFERENCES gear(id),
    visibility VARCHAR(16) DEFAULT 'followers', -- public, followers, private
    privacy_zone_start GEOMETRY(Point, 4326),
    privacy_zone_end GEOMETRY(Point, 4326),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Routes (PostGIS LineString)
CREATE TABLE routes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    activity_id UUID NOT NULL REFERENCES activities(id),
    geom GEOMETRY(LineStringZ, 4326) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_routes_geom ON routes USING GIST(geom);

-- Social
CREATE TABLE follows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID NOT NULL REFERENCES actors(id),
    target_actor_id UUID NOT NULL REFERENCES actors(id),
    status VARCHAR(16) DEFAULT 'pending', -- pending, accepted, rejected
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(actor_id, target_actor_id)
);

CREATE TABLE likes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    activity_id UUID NOT NULL REFERENCES activities(id),
    actor_id UUID NOT NULL REFERENCES actors(id),
    ap_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(activity_id, actor_id)
);

-- Gear
CREATE TABLE gear (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(128) NOT NULL,
    gear_type VARCHAR(32) NOT NULL, -- bike, shoe, wetsuit
    brand VARCHAR(64),
    model VARCHAR(128),
    distance_meters DECIMAL(12,2) DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 5. Key Features

### 5.1 Activity Tracking

- **Import**: GPX, FIT, TCX, ZIP archives
- **Manual Entry**: Duration, distance, elevation, metrics
- **Auto-Detect**: Activity type from file metadata
- **Batch Import**: From Strava/Komoot/other platforms

### 5.2 Analytics

- **Personal Records**: Fastest 5K, longest ride, highest power
- **Training Load**: TSS, CTL, ATL calculations
- **Power Analysis**: FTP estimation, power zones
- **Heart Rate Zones**: Custom zone configuration
- **Elevation Profiles**: Interactive charts
- **Heatmaps**: Activity distribution over time

### 5.3 Social

- **Follow/Unfollow**: Standard ActivityPub flow
- **Kudos/Likes**: `Like` activity federation
- **Comments**: `Create` with `inReplyTo`
- **Timelines**: Home feed, public feed
- **Notifications**: Likes, follows, comments

### 5.4 Maps

- **Route Visualization**: OpenStreetMap + Leaflet
- **Privacy Zones**: Auto-blur start/end points
- **Segment Highlighting**: Highlight specific segments
- **Heatmaps**: Personal activity density

### 5.5 Gear Tracking

- **Bike/Shoe Tracking**: Mileage per item
- **Maintenance Reminders**: Based on distance/time
- **Gear Assignment**: Auto or manual
- **Retirement**: Archive old gear

---

## 6. Privacy & Security

### 6.1 Privacy Controls

- **Activity Visibility**: Public / Followers / Private
- **Privacy Zones**: Auto-blur start/end (configurable radius)
- **Profile Visibility**: Public or followers-only
- **Federation Scope**: Instance-level policies

### 6.2 Security

- **HTTP Signatures**: Required for all S2S requests
- **JWT Tokens**: For C2S authentication
- **Rate Limiting**: Per-user and per-IP
- **Input Validation**: Strict file parsing
- **CORS**: Configurable allowed origins

---

## 7. Hosting Model

### 7.1 Instance Types

```
┌─────────────────────────────────────────────────────────────────┐
│                     INSTANCE TOPOLOGY                           │
│                                                                 │
│  ┌─────────────────────┐       ┌─────────────────────┐        │
│  │   Official Instance  │◄─────►│  Community Instance  │        │
│  │   peloton.social     │       │  cycling.club        │        │
│  │                      │       │                      │        │
│  │  - Free accounts     │       │  - Club/community    │        │
│  │  - Open registration │       │  - Invite-only?      │        │
│  │  - Moderated         │       │  - Custom branding   │        │
│  └──────────┬───────────┘       └──────────┬───────────┘        │
│             │                              │                    │
│             └──────────────┬───────────────┘                    │
│                            │                                    │
│                            ▼                                    │
│               ┌────────────────────────┐                        │
│               │    FEDERATION MESH     │                        │
│               │                        │                        │
│               │  ┌──────┐  ┌──────┐   │                        │
│               │  │Masto │  │Peer  │   │                        │
│               │  │don   │  │Tube  │   │                        │
│               │  └──────┘  └──────┘   │                        │
│               │                        │                        │
│               └────────────────────────┘                        │
└─────────────────────────────────────────────────────────────────┘
```

### 7.2 Hosting Options

| Option | Description | Best For |
|--------|-------------|----------|
| **Official Instance** | `peloton.social` - hosted by core team | New users, testing, community |
| **Community Instance** | Self-hosted by clubs, orgs, individuals | Privacy-conscious, communities |
| **Personal Instance** | Single-user self-hosting | Data ownership, experimentation |
| **Shared Instance** | Multi-user, community-moderated | Friend groups, cycling clubs |

### 7.3 Registration Models

**Open Registration (Official Instance):**
```yaml
# instance-config.yaml
registration:
  mode: open              # open | invite | closed
  require_email: true
  email_verification: true
  default_visibility: followers
  max_activities_per_day: 50  # rate limit for new accounts
```

**Invite-Only (Community Instance):**
```yaml
registration:
  mode: invite
  invite_expiry_days: 7
  max_invites_per_user: 5
  require_approval: false
```

**Closed (Personal Instance):**
```yaml
registration:
  mode: closed
  allowed_users:          # whitelist
    - alice@example.com
    - bob@example.com
```

### 7.4 Federation Policies

```yaml
federation:
  # Who can follow your users?
  inbound_follow:
    policy: open          # open | approved | closed
  
  # Who can your users follow?
  outbound_follow:
    policy: open          # open | approved | closed
  
  # Activity delivery scope
  delivery:
    public_activities: true
    follower_activities: true
    
  # Blocked instances
  blocked_instances:
    - spam.example.com
    - abusive.example.com
    
  # Allowed instances (if using allowlist)
  # allowed_instances:
  #   - mastodon.social
  #   - cycling.club
```

### 7.5 Data Portability

Users should be able to:
1. **Export all data** (GDPR compliance, personal backup)
2. **Migrate between instances** (ActivityPub account migration)
3. **Delete account and data** (right to be forgotten)

```yaml
# Export formats
export:
  formats:
    - gpx          # Routes and activities
    - fit          # Full fitness data
    - json         # All data, including social graph
    - csv          # Tabular data for spreadsheets
  
# Migration (ActivityPub)
migration:
  enabled: true
  redirect_old_actor: true  # Maintain followers during migration
```

### 7.6 Moderation

```yaml
moderation:
  # Report handling
  reports:
    email: reports@peloton.social
    auto_block_threshold: 3  # Auto-block after N reports
  
  # Content filtering
  content_filter:
    require_spoiler_for: []  # No content warnings needed
    auto_hide: false
  
  # Instance-wide
  instance:
    rules:
      - "Be respectful"
      - "No spam"
      - "No hate speech"
    blocklist_sync: true  # Sync with community blocklists
```

---

## 8. Implementation Phases

### Phase 1: Foundation (Months 1-3)

- [ ] Project setup (Rust, Axum, PostgreSQL)
- [ ] User authentication (signup, login, JWT)
- [ ] Basic ActivityPub (WebFinger, Actor, Inbox/Outbox)
- [ ] GPX/FIT file import
- [ ] Activity CRUD (create, read, update, delete)
- [ ] Basic web UI (activity list, detail view)

### Phase 2: Social (Months 4-6)

- [ ] Follow/Unfollow (ActivityPub)
- [ ] Likes/Kudos federation
- [ ] Comments federation
- [ ] Activity feed (home, public)
- [ ] Notifications

### Phase 3: Analytics (Months 7-9)

- [ ] Personal records
- [ ] Training load calculations
- [ ] Power analysis (FTP, zones)
- [ ] Elevation profiles
- [ ] Charts and visualizations

### Phase 4: Maps & Gear (Months 10-12)

- [ ] Interactive map view (Leaflet)
- [ ] Privacy zones
- [ ] Gear tracking
- [ ] Maintenance reminders
- [ ] Segment support

### Phase 5: Mobile & Polish (Months 13-15)

- [ ] React Native mobile app
- [ ] Push notifications
- [ ] Data export (GPX, FIT)
- [ ] Instance federation policies
- [ ] Performance optimization

---

## 9. Deployment Architecture

### 9.1 Development (SQLite)

```bash
# No setup needed - SQLite is zero-config
cargo run

# Database file created at: ./data/peloton.db
# WAL mode enabled automatically for concurrency
```

**Development Stack:**
```
┌─────────────────────────────────────────┐
│           Development Machine            │
│                                         │
│  ┌─────────────┐  ┌─────────────┐      │
│  │ Axum Server │  │ React PWA   │      │
│  │ (port 3000) │  │ (port 5173) │      │
│  └──────┬──────┘  └─────────────┘      │
│         │                               │
│  ┌──────▼──────┐  ┌─────────────┐      │
│  │ SQLite DB   │  │ Local Files │      │
│  │ (./data/)   │  │ (./uploads/)│      │
│  └─────────────┘  └─────────────┘      │
└─────────────────────────────────────────┘
```

### 9.2 Small Instance (Docker Compose)

```yaml
# docker-compose.yml
services:
  app:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data          # SQLite database
      - ./uploads:/app/uploads    # Activity files
    environment:
      - DATABASE_URL=sqlite:///app/data/peloton.db
      - RUST_LOG=peloton=info
    depends_on:
      - redis

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redisdata:/data

  caddy:
    image: caddy:2-alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile
      - caddydata:/data
      - caddyconfig:/config

volumes:
  redisdata:
  caddydata:
  caddyconfig:
```

### 9.3 Production (Kubernetes)

```yaml
# Simplified - see docs/ for full manifests
apiVersion: apps/v1
kind: Deployment
metadata:
  name: peloton
spec:
  replicas: 3
  selector:
    matchLabels:
      app: peloton
  template:
    spec:
      containers:
        - name: app
          image: ghcr.io/peloton/peloton:latest
          ports:
            - containerPort: 3000
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: peloton-secrets
                  key: database-url
          volumeMounts:
            - name: data
              mountPath: /app/data
      volumes:
        - name: data
          persistentVolumeClaim:
            claimName: peloton-data
```

### 9.4 Migration: SQLite → PostgreSQL

**When to migrate:**
- Multiple concurrent users (10+)
- Need geospatial queries (PostGIS)
- Need better concurrency
- Need replication/backups

**Migration steps:**
```bash
# 1. Export from SQLite
peloton-cli export --format postgres --output migration.sql

# 2. Import to PostgreSQL
psql -d peloton -f migration.sql

# 3. Update config
DATABASE_URL=postgresql://user:pass@localhost/peloton

# 4. Run migrations (adds PostGIS, indexes)
cargo run -- migrate
```

---

## 10. Implementation Phases

---

## 9. Competitive Advantages

### vs. Strava

| Feature | Strava | Lièvre |
|---------|--------|---------|
| Cost | $$$$ subscription | Free |
| Data Ownership | Vendor-locked | Full ownership |
| Federation | None | ActivityPub |
| Self-Hosting | No | Yes |
| Open Source | No | Yes |
| Privacy Controls | Limited | Granular |

### vs. FitTrackee

| Feature | FitTrackee | Lièvre |
|---------|------------|---------|
| Federation | None | ActivityPub |
| Multi-sport | Limited | Cycling-first, expandable |
| Analytics | Basic | Advanced (power, FTP) |
| Social Features | Minimal | Full federation |
| Maps | Basic | Interactive Leaflet |

### vs. FitPub

| Feature | FitPub | Lièvre |
|---------|--------|---------|
| Stack | Java/Spring | Rust/Axum |
| Focus | General fitness | Cycling-first |
| Performance | Good | Excellent |
| Maturity | Active | New |
| Federation | ActivityPub | ActivityPub + Fedisport |

---

## 12. Open Questions

1. **Hosting Model**: Official instance + community instances, or community-only?
2. **Moderation**: Centralized vs. distributed moderation? (deferred to post-MVP)

---

## 13. Related Documents

| Document | Purpose |
|----------|---------|
| [GLOSSARY.md](GLOSSARY.md) | Ubiquitous language and domain terms |
| [BACKLOG.md](BACKLOG.md) | Detailed user stories and milestones |
| [FAQ.md](FAQ.md) | User-facing FAQs and API reference |

---

## 14. References

- [ActivityPub W3C Spec](https://www.w3.org/TR/activitypub/)
- [ActivityStreams 2.0](https://www.w3.org/TR/activitystreams-core/)
- [Fedisport Vocabulary](https://github.com/fedisport/vocabulary)
- [activitypub-federation-rust](https://github.com/LemmyNet/activitypub-federation-rust)
- [FitPub](https://codeberg.org/fitpub/fitpub)
- [Open Pace](https://github.com/myfear/open-pace)
- [Wanderer](https://wanderer.to/)
- [SocialDocs](https://socialdocs.org/)

---

*Last updated: 2026-08-10*
