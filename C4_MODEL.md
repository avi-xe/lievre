# Lièvre C4 Architecture Model

This document describes the current architecture using the [C4 model](https://c4model.com/) — four levels of abstraction for software architecture.

**Current State:** Epic 1 (Foundation) + Epic 2 (File Import) + Epic 3 (Activity Processing) complete.

---

## Level 1: System Context

The big picture — who uses Lièvre and what it connects to.

```mermaid
graph TB
    subgraph "External Users"
        User["👤 User<br/><i>Cyclist / Runner</i>"]
        Admin["🔧 Admin<br/><i>Instance Operator</i>"]
    end

    subgraph "External Systems"
        Mastodon["🐘 Mastodon<br/><i>Fediverse Server</i>"]
        PeerTube["📺 PeerTube<br/><i>Fediverse Server</i>"]
        Strava["📱 Strava<br/><i>Export Source</i>"]
        Garmin["⌚ Garmin<br/><i>Device Export</i>"]
    end

    subgraph "Lièvre Instance"
        Lievre["🦌 Lièvre<br/><i>Federated Fitness Platform</i>"]
    end

    User -->|"Uploads activities<br/>GPX/FIT/TCX/ZIP"| Lievre
    User -->|"Views feed,<br/>analytics, routes"| Lievre
    Admin -->|"Manages instance<br/>via CLI/config"| Lievre
    Lievre -->|"ActivityPub<br/>(Federation)"| Mastodon
    Lievre -->|"ActivityPub<br/>(Federation)"| PeerTube
    Strava -->|"CSV export<br/>(import)"| Lievre
    Garmin -->|"FIT/TCX export<br/>(import)"| Lievre
```

### Key Relationships

| Relationship | Protocol | Description |
|-------------|----------|-------------|
| User → Lièvre | HTTP/HTTPS | REST API for activity upload, viewing, management |
| Lièvre → Mastodon/PeerTube | ActivityPub S2S | Federate exercises to followers across instances |
| Strava/Garmin → Lièvre | File Import | Users export data and upload to Lièvre |

---

## Level 2: Container Diagram

The high-level technical building blocks inside Lièvre.

```mermaid
graph TB
    subgraph "Client Layer"
        Browser["🌐 Browser<br/><i>Web UI (Future PWA)</i>"]
        CLI["⌨️ CLI Tool<br/><i>Future: lievre-cli</i>"]
    end

    subgraph "Lièvre System"
        subgraph "Infrastructure"
            Caddy["🌍 Caddy<br/><i>Reverse Proxy<br/>TLS Termination</i>"]
        end

        subgraph "Application"
            API["🦀 API Server<br/><i>Axum + Tokio</i><br/>:3000"]
        end

        subgraph "Data"
            SQLite[("💾 SQLite<br/><i>lievre.db</i>")]
            Redis[("⚡ Redis<br/><i>Cache/Queue (Future)</i>")]
            FileSystem["📁 File System<br/><i>Uploaded files</i>"]
        end
    end

    subgraph "Federation Network"
        RemoteInstance["🌐 Remote AP Server<br/><i>Mastodon, PeerTube, etc.</i>"]
    end

    Browser -->|"HTTP :80/443"| Caddy
    CLI -->|"HTTP :3000"| API
    Caddy -->|"Proxy"| API
    API -->|"SQLx Queries"| SQLite
    API -->|"Future: Sessions"| Redis
    API -->|"File Storage"| FileSystem
    API -->|"ActivityPub S2S"| RemoteInstance
```

### Container Responsibilities

| Container | Technology | Purpose |
|-----------|------------|---------|
| **Caddy** | Go, Docker | Reverse proxy, automatic TLS, static file serving |
| **API Server** | Rust, Axum, Tokio | REST API, file import, business logic, federation |
| **SQLite** | SQLite 3 | Persistent storage for users, activities, routes |
| **Redis** | Redis 7 | Session cache, job queue (not yet integrated) |
| **File System** | Host mount | Stored GPX/FIT/TCX files, future map tiles |

### Data Flow: Activity Import

```mermaid
sequenceDiagram
    participant User
    participant Caddy
    participant API
    participant Parser
    participant SQLite

    User->>Caddy: POST /api/import/gpx (multipart)
    Caddy->>API: Forward request
    API->>Parser: Parse GPX/FIT/TCX
    Parser-->>API: CreateActivity + CreateRoute
    API->>SQLite: INSERT activity
    API->>SQLite: INSERT route
    API-->>User: 201 Created {activity_id}
```

---

## Level 3: Component Diagram

Inside the API Server — the core components we've built.

```mermaid
graph TB
    subgraph "API Server (crates/api)"
        Main["main.rs<br/><i>Server bootstrap, routing</i>"]
        ImportAPI["import.rs<br/><i>POST /api/import/gpx</i>"]
        GeoJsonAPI["geojson.rs<br/><i>GET /api/activities/:id/geojson</i>"]
    end

    subgraph "Core Library (crates/core)"
        subgraph "Domain"
            Activity["activity.rs<br/><i>Activity CRUD<br/>ActivityType, Visibility</i>"]
            Route["route.rs<br/><i>Route CRUD<br/>GeoJSON conversion</i>"]
            User["user.rs<br/><i>User model<br/>UserRepository</i>"]
            Auth["auth.rs<br/><i>JWT auth<br/>Claims, AuthService</i>"]
        end

        subgraph "File Import"
            GPX["gpx.rs<br/><i>GPX parser<br/>XML → Activity + Route</i>"]
            FIT["fit.rs<br/><i>FIT parser<br/>Binary → Activity + Route</i>"]
            TCX["tcx.rs<br/><i>TCX parser<br/>XML → Activity + Route</i>"]
            Batch["batch.rs<br/><i>Batch importer<br/>ZIP detection, format routing</i>"]
            Strava["strava.rs<br/><i>Strava CSV parser<br/>Unit conversion (mph→m/s)</i>"]
        end

        subgraph "Processing"
            Job["job.rs<br/><i>Job queue<br/>SQLite-backed, retry logic</i>"]
            Stats["stats.rs<br/><i>Stats computation<br/>Distance, speed, elevation</i>"]
        end
    end

    subgraph "Shared Library (crates/shared)"
        DB["db.rs<br/><i>SQLite pool<br/>Migrations</i>"]
        Error["error.rs<br/><i>Error types<br/>LievreError</i>"]
    end

    subgraph "Federation Library (crates/federation)"
        Federation["lib.rs<br/><i>Stub<br/>(placeholder)</i>"]
    end

    Main --> ImportAPI
    Main --> GeoJsonAPI
    Main --> Activity
    Main --> Route
    ImportAPI --> GPX
    ImportAPI --> FIT
    ImportAPI --> TCX
    ImportAPI --> Batch
    ImportAPI --> Strava
    GeoJsonAPI --> Route
    Activity --> DB
    Route --> DB
    User --> DB
    Job --> DB
    Stats --> DB
    Auth -.->|"Future"| DB
```

### Component Details

#### Domain Components

| Component | File | Responsibilities | Dependencies |
|-----------|------|------------------|--------------|
| **Activity** | `activity.rs` | CRUD operations, type filtering, user scoping | SQLite via SQLx |
| **Route** | `route.rs` | GeoJSON LineString storage, coordinate management | SQLite via SQLx |
| **User** | `user.rs` | User model, email/username lookup | SQLite via SQLx |
| **Auth** | `auth.rs` | JWT generation/validation, password hashing | argon2, jsonwebtoken |

#### File Import Components

| Component | File | Input Format | Output | Dependencies |
|-----------|------|--------------|--------|--------------|
| **GpxParser** | `gpx.rs` | `.gpx` (XML) | CreateActivity + CreateRoute | quick-xml |
| **FitParser** | `fit.rs` | `.fit` (Binary) | CreateActivity + CreateRoute | fitparser |
| **TcxParser** | `tcx.rs` | `.tcx` (XML) | CreateActivity + CreateRoute | quick-xml |
| **BatchImporter** | `batch.rs` | `.zip` | Vec\<ParsedActivity\> | zip crate |
| **StravaParser** | `strava.rs` | `.csv` | Vec\<StravaActivity\> | csv crate |

### Internal Data Flow

```mermaid
flowchart LR
    A[Upload File] --> B{Format Detection}
    B -->|.gpx| C[GpxParser]
    B -->|.fit| D[FitParser]
    B -->|.tcx| E[TcxParser]
    B -->|.zip| F[BatchImporter]
    B -->|.csv| G[StravaParser]

    C --> H[CreateActivity]
    D --> H
    E --> H
    F --> I{Per-file format}
    I --> C
    I --> D
    I --> E
    G --> H

    H --> J[ActivityRepository]
    J --> K[(SQLite)]
```

---

## Level 4: Code (Current Implementation)

The actual module structure and key interfaces.

```
lievre/
├── Cargo.toml              # Workspace root
├── docker-compose.yml      # Container orchestration
├── Dockerfile              # Multi-stage Rust build
├── crates/
│   ├── api/                # HTTP layer (Axum)
│   │   └── src/
│   │       ├── main.rs     # Server entry, routing
│   │       └── import.rs   # GPX upload endpoint
│   ├── core/               # Business logic
│   │   └── src/
│   │       ├── lib.rs      # Module re-exports
│   │       ├── activity.rs # Activity model + repo
│   │       ├── route.rs    # Route model + GeoJSON
│   │       ├── user.rs     # User model
│   │       ├── auth.rs     # JWT authentication
│   │       └── import/
│   │           ├── mod.rs  # Parser exports
│   │           ├── gpx.rs  # GPX parser
│   │           ├── fit.rs  # FIT parser
│   │           ├── tcx.rs  # TCX parser
│   │           ├── batch.rs # Batch importer
│   │           └── strava.rs # Strava CSV
│   ├── shared/             # Common utilities
│   │   └── src/
│   │       ├── db.rs       # SQLite pool, migrations
│   │       └── error.rs    # Error types
│   └── federation/         # ActivityPub (placeholder)
│       └── src/
│           └── lib.rs      # Stub
```

### Key Interfaces

```rust
// Activity Repository
pub struct ActivityRepository { pool: SqlitePool }
impl ActivityRepository {
    pub async fn create(&self, user_id: &str, activity: CreateActivity) -> Result<Activity>;
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Activity>>;
    pub async fn find_by_user(&self, user_id: &str) -> Result<Vec<Activity>>;
    pub async fn delete(&self, id: &str) -> Result<()>;
}

// File Parsers (trait-like pattern)
pub struct GpxParser;
impl GpxParser {
    pub fn parse(&self, content: &str) -> Result<ParsedGpx>;
    pub fn to_create_activity(&self, gpx: &ParsedGpx) -> CreateActivity;
    pub fn to_create_route(&self, activity_id: &str, gpx: &ParsedGpx) -> CreateRoute;
}
```

---

## Technology Decisions

| Decision | Choice | Rationale | Trade-off |
|----------|--------|-----------|-----------|
| Language | **Rust** | Performance, safety, type-safety | Steeper learning curve |
| Web Framework | **Axum** | Tower-based, async, good ergonomics | Less mature than Actix |
| Database | **SQLite** | Zero-config, single-file, great for dev | Limited concurrency at scale |
| XML Parsing | **quick-xml** | Fast, low memory, event-based | SAX-style (not DOM) |
| FIT Parsing | **fitparser** | Garmin-compatible, serde-based | Limited documentation |
| Federation | **activitypub-federation** | Battle-tested by Lemmy | Adds complexity |

---

## What's Built vs. What's Planned

### ✅ Built (Epic 1 + 2 + 3)

| Layer | Status | Components |
|-------|--------|------------|
| Database | ✅ SQLite | Pool, migrations, CRUD tables, jobs, stats |
| Auth | ✅ Local | Register, login, JWT |
| Activities | ✅ CRUD | Create, read, delete |
| Routes | ✅ Storage | GeoJSON, coordinates |
| Import | ✅ Multi-format | GPX, FIT, TCX, ZIP, Strava CSV |
| Processing | ✅ Background | Job queue with retry, stats computation |
| API | ✅ REST | Health, import, GeoJSON endpoint |

### 🔨 Not Yet Built

| Layer | Status | Components |
|-------|--------|------------|
| Federation | 🔲 Stub | ActivityPub C2S/S2S, WebFinger |
| Social | 🔲 Planned | Follow, like, comment |
| Analytics | 🔲 Planned | Stats, PRs, charts |
| Frontend | 🔲 Planned | React PWA, maps, charts |
| Background Jobs | 🔲 Planned | Async processing, queue |
| Gear Tracking | 🔲 Planned | Equipment management |

---

## Evolution Path

```mermaid
graph LR
    A[Epic 1: Foundation] --> B[Epic 2: File Import]
    B --> C[Epic 3: Activity Processing]
    C --> D[Epic 4: Maps & Visualization]
    D --> E[Epic 5: Social Features]
    E --> F[Epic 6: Federation]
    F --> G[Epic 7: PWA Frontend]

    style A fill:#90EE90
    style B fill:#90EE90
    style C fill:#90EE90
    style D fill:#FFD700
    style E fill:#FFD700
    style F fill:#FFD700
    style G fill:#FFD700
```

| Epic | Focus | Key Deliverables |
|------|-------|------------------|
| **1** ✅ | Foundation | DB, Auth, API scaffold |
| **2** ✅ | File Import | GPX/FIT/TCX parsers, batch import |
| **3** ✅ | Activity Processing | Stats computation, job queue, GeoJSON |
| **4** | Maps & Visualization | Leaflet maps, elevation charts |
| **5** | Social Features | Follow, like, comment, feed |
| **6** | Federation | ActivityPub, WebFinger, federation with Mastodon |
| **7** | Frontend PWA | React app, offline support |

---

*Last updated: 2026-08-10*
