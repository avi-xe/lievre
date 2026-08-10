# Lièvre — Product Backlog

## Overview

This backlog contains all planned work for Lièvre, organized by epic. Each story has acceptance criteria and technical notes.

**Priority Scale:**
- P0 — Must have for MVP
- P1 — Should have for MVP
- P2 — Nice to have for MVP
- P3 — Post-MVP

**Estimate Scale:** S (1-2 days) | M (3-5 days) | L (1-2 weeks) | XL (2-4 weeks)

---

## Epic 1: Foundation

> Core infrastructure, project setup, and basic API.

### 1.1 Project Scaffolding
**Priority:** P0 | **Estimate:** M

**Story:** As a developer, I want a runnable Rust project with basic structure so that I can start building features.

**Acceptance Criteria:**
- [ ] Cargo workspace with `lievre-core`, `lievre-api`, `lievre-federation`, `lievre-shared` crates
- [ ] Axum HTTP server starts on port 3000
- [ ] Health check endpoint: `GET /health` returns `200 OK`
- [ ] Environment configuration via `.env` or `config.toml`
- [ ] Logging with `tracing` crate
- [ ] Docker Compose for local development

**Technical Notes:**
```
lievre/
├── Cargo.toml          # workspace
├── crates/
│   ├── core/           # domain logic, no HTTP
│   ├── api/            # Axum handlers, middleware
│   ├── federation/     # ActivityPub logic
│   └── shared/         # common types, error handling
├── migrations/         # SQL migrations
├── static/             # PWA assets
└── docker-compose.yml
```

---

### 1.2 Database Setup
**Priority:** P0 | **Estimate:** M

**Story:** As a developer, I want SQLite with migrations so that I can persist data.

**Acceptance Criteria:**
- [ ] SQLite database file at `./data/lievre.db`
- [ ] SQLx for query execution
- [ ] migrations folder with initial schema
- [ ] `sqlx migrate run` command works
- [ ] Connection pool with proper error handling

**Technical Notes:**
- Use `sqlx` with `sqlite` feature
- WAL mode enabled by default
- Migrations in `migrations/` directory

---

### 1.3 User Model
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to create an account and log in so that I can own my activities.

**Acceptance Criteria:**
- [ ] `users` table with: id, email, username, password_hash, display_name, avatar_url, created_at, updated_at
- [ ] `POST /api/auth/register` — create account (email verification deferred)
- [ ] `POST /api/auth/login` — return JWT token
- [ ] `GET /api/users/me` — return current user (requires auth)
- [ ] Password hashing with `argon2` or `bcrypt`
- [ ] JWT with user_id claim

**Technical Notes:**
```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);
```

---

### 1.4 Activity Model
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to create, read, update, and delete activities so that I can manage my training log.

**Acceptance Criteria:**
- [ ] `activities` table with: id, user_id, activity_type, title, description, started_at, duration_seconds, distance_meters, elevation_gain_meters, visibility, created_at, updated_at
- [ ] `POST /api/activities` — create activity
- [ ] `GET /api/activities` — list user's activities (paginated)
- [ ] `GET /api/activities/{id}` — get activity detail
- [ ] `PUT /api/activities/{id}` — update activity
- [ ] `DELETE /api/activities/{id}` — delete activity
- [ ] Activity types: ride, run, swim, walk, hike, virtual-ride
- [ ] Visibility: public, followers, private

**Technical Notes:**
```sql
CREATE TABLE activities (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
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
```

---

### 1.5 Route Storage
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want my activities to include a route so that I can visualize where I went.

**Acceptance Criteria:**
- [ ] `routes` table with: id, activity_id, coordinates (JSON), elevation_data (JSON)
- [ ] Route stored as GeoJSON-compatible format
- [ ] `GET /api/activities/{id}/route` — return route as GeoJSON LineString
- [ ] Route deletion cascades with activity

**Technical Notes:**
```sql
CREATE TABLE routes (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    coordinates TEXT NOT NULL,  -- JSON array of [lon, lat] or [lon, lat, ele]
    elevation_data TEXT,        -- JSON array of elevation values
    created_at TEXT DEFAULT (datetime('now'))
);
```

---

## Epic 2: File Import

> Import activities from GPS files.

### 2.1 GPX Import
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to upload GPX files so that I can import my rides and runs.

**Acceptance Criteria:**
- [ ] `POST /api/import/gpx` — upload GPX file
- [ ] Parse GPX tracks, extract coordinates and timestamps
- [ ] Create activity with: type, start time, duration, distance
- [ ] Create route from track points
- [ ] Return created activity ID
- [ ] Handle malformed GPX gracefully

**Technical Notes:**
- Use `quick-xml` for GPX parsing
- Calculate distance from coordinates (Haversine)
- Calculate duration from first/last timestamp

---

### 2.2 FIT Import
**Priority:** P1 | **Estimate:** L

**Story:** As a user, I want to upload FIT files so that I can import activities from Garmin devices.

**Acceptance Criteria:**
- [ ] `POST /api/import/fit` — upload FIT file
- [ ] Parse FIT sessions, extract metrics (power, HR, cadence)
- [ ] Create activity with full stats
- [ ] Create route from GPS data in FIT

**Technical Notes:**
- Use `fit-rs` or custom parser
- FIT contains more data than GPX (power zones, training load)

---

### 2.3 TCX Import
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to upload TCX files so that I can import from older Garmin exports.

**Acceptance Criteria:**
- [ ] `POST /api/import/tcx` — upload TCX file
- [ ] Parse TCX activities and laps
- [ ] Create activity with stats

---

### 2.4 Batch Import
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to upload a ZIP of multiple files so that I can bulk-import my history.

**Acceptance Criteria:**
- [ ] `POST /api/import/zip` — upload ZIP archive
- [ ] Extract and process each GPX/FIT/TCX file
- [ ] Return summary of imported activities
- [ ] Handle partial failures gracefully

---

### 2.5 Strava Export Import
**Priority:** P2 | **Estimate:** L

**Story:** As a user, I want to import my Strava export so that I can migrate my history.

**Acceptance Criteria:**
- [ ] `POST /api/import/strava` — upload Strava ZIP export
- [ ] Parse `activities.csv` and associated GPX files
- [ ] Create activities with all available data
- [ ] Handle Strava-specific formats

---

## Epic 3: Activity Processing

> Background jobs for computing stats and generating content.

### 3.1 Worker Queue
**Priority:** P0 | **Estimate:** L

**Story:** As a system, I want a job queue so that I can process activities asynchronously.

**Acceptance Criteria:**
- [ ] SQLite-backed job queue (WAL mode)
- [ ] Job types: ProcessGpx, ProcessFit, ComputeStats, GenerateRoute
- [ ] Worker polls for pending jobs
- [ ] Retry logic with exponential backoff
- [ ] Job status tracking (pending, processing, completed, failed)

**Technical Notes:**
```sql
CREATE TABLE jobs (
    id TEXT PRIMARY KEY,
    job_type TEXT NOT NULL,
    payload TEXT NOT NULL,  -- JSON
    status TEXT DEFAULT 'pending',
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    created_at TEXT DEFAULT (datetime('now')),
    started_at TEXT,
    completed_at TEXT,
    error TEXT
);
```

---

### 3.2 Stats Computation
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want my activities to have computed stats so that I can analyze my performance.

**Acceptance Criteria:**
- [ ] Compute distance from route coordinates
- [ ] Compute duration from timestamps
- [ ] Compute elevation gain from elevation data
- [ ] Compute average/max speed
- [ ] Store stats in `activity_stats` table

---

### 3.3 GeoJSON Generation
**Priority:** P0 | **Estimate:** S

**Story:** As a user, I want my routes to be served as GeoJSON so that I can display them on maps.

**Acceptance Criteria:**
- [ ] Generate GeoJSON LineString from route coordinates
- [ ] Include elevation in third dimension
- [ ] Cache generated GeoJSON

---

### 3.4 Gear Mileage Tracking
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to track mileage on my bikes and shoes so that I know when to replace them.

**Acceptance Criteria:**
- [ ] `gear` table with: id, user_id, name, type, brand, model, distance_meters
- [ ] Assign gear to activity
- [ ] Update gear mileage on activity creation
- [ ] `GET /api/gear` — list user's gear with mileage

---

## Epic 4: Maps & Visualization

> Interactive maps and charts.

### 4.1 Map Display
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to see my activity routes on an interactive map so that I can visualize my rides.

**Acceptance Criteria:**
- [ ] Leaflet map component in React
- [ ] Load GeoJSON route from API
- [ ] Display route with start/end markers
- [ ] Zoom to fit route bounds
- [ ] OpenStreetMap tiles (no API key)

---

### 4.2 Privacy Zones
**Priority:** P1 | **Estimate:** M

**Story:** As a user, I want to hide my home address from shared routes so that I stay safe.

**Acceptance Criteria:**
- [ ] User can set privacy zone radius (default 200m)
- [ ] Blur start/end points in shared routes
- [ ] Privacy zone stored in user settings
- [ ] Applied when serving route to remote instances

---

### 4.3 Elevation Profile
**Priority:** P1 | **Estimate:** M

**Story:** As a user, I want to see an elevation chart of my activity so that I can understand the terrain.

**Acceptance Criteria:**
- [ ] Chart component showing elevation over distance
- [ ] Interactive (hover to see elevation at point)
- [ ] Highlight climbs and descents

---

### 4.4 Activity Statistics Dashboard
**Priority:** P1 | **Estimate:** L

**Story:** As a user, I want a dashboard showing my training statistics so that I can track progress.

**Acceptance Criteria:**
- [ ] Weekly/monthly/yearly summaries
- [ ] Total distance, time, elevation
- [ ] Activity count by type
- [ ] Training load (TSS) over time

---

## Epic 5: Social Features

> Following, likes, comments, and feeds.

### 5.1 Follow System
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to follow other users so that I can see their activities in my feed.

**Acceptance Criteria:**
- [ ] `follows` table: follower_id, following_id, status, created_at
- [ ] `POST /api/users/{id}/follow` — request to follow
- [ ] `DELETE /api/users/{id}/follow` — unfollow
- [ ] `GET /api/users/{id}/followers` — list followers
- [ ] `GET /api/users/{id}/following` — list following
- [ ] Follow approval for private accounts (optional)

---

### 5.2 Likes (Kudos)
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to give kudos to activities so that I can show appreciation.

**Acceptance Criteria:**
- [ ] `likes` table: activity_id, user_id, created_at
- [ ] `POST /api/activities/{id}/like` — like activity
- [ ] `DELETE /api/activities/{id}/like` — unlike
- [ ] `GET /api/activities/{id}/likes` — list who liked
- [ ] Count of likes on activity

---

### 5.3 Comments
**Priority:** P1 | **Estimate:** M

**Story:** As a user, I want to comment on activities so that I can discuss with friends.

**Acceptance Criteria:**
- [ ] `comments` table: id, activity_id, user_id, content, created_at
- [ ] `POST /api/activities/{id}/comments` — add comment
- [ ] `GET /api/activities/{id}/comments` — list comments
- [ ] `DELETE /api/comments/{id}` — delete own comment

---

### 5.4 Activity Feed
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to see a feed of activities from people I follow so that I can stay connected.

**Acceptance Criteria:**
- [ ] `GET /api/feed` — paginated feed of followed users' activities
- [ ] Include user info, activity summary, like count
- [ ] Filter by activity type
- [ ] Sort by time (newest first)

---

### 5.5 Public Feed
**Priority:** P1 | **Estimate:** M

**Story:** As a visitor, I want to see public activities so that I can discover users.

**Acceptance Criteria:**
- [ ] `GET /api/feed/public` — public activities (no auth required)
- [ ] Paginated, sorted by time
- [ ] Filter by activity type

---

### 5.6 Notifications
**Priority:** P1 | **Estimate:** L

**Story:** As a user, I want to be notified when someone follows me, likes my activity, or comments so that I can engage.

**Acceptance Criteria:**
- [ ] `notifications` table: id, user_id, type, data, read, created_at
- [ ] Create notification on: follow, like, comment
- [ ] `GET /api/notifications` — list notifications (paginated)
- [ ] `PUT /api/notifications/{id}/read` — mark as read
- [ ] `PUT /api/notifications/read-all` — mark all as read
- [ ] WebSocket for real-time notifications (optional)

---

## Epic 6: ActivityPub Federation

> Federation with the fediverse.

### 6.1 WebFinger
**Priority:** P0 | **Estimate:** S

**Story:** As a system, I want to implement WebFinger so that other servers can discover users.

**Acceptance Criteria:**
- [ ] `GET /.well-known/webfinger?resource=acct:user@domain`
- [ ] Return JSON with actor URL
- [ ] Handle unknown users gracefully

---

### 6.2 Actor Endpoint
**Priority:** P0 | **Estimate:** M

**Story:** As a system, I want to serve actor profiles in ActivityPub format so that other servers can follow users.

**Acceptance Criteria:**
- [ ] `GET /users/{username}` — return Person actor JSON-LD
- [ ] Include inbox, outbox, followers, following URLs
- [ ] Include public key for signature verification
- [ ] Content-Type: `application/activity+json`

---

### 6.3 Outbox
**Priority:** P0 | **Estimate:** M

**Story:** As a system, I want to publish activities to an outbox so that followers can retrieve them.

**Acceptance Criteria:**
- [ ] `GET /users/{username}/outbox` — OrderedCollection of activities
- [ ] Each activity wrapped in Create activity
- [ ] Pagination support

---

### 6.4 Inbox (Receiving)
**Priority:** P0 | **Estimate:** L

**Story:** As a system, I want to receive activities from other servers so that I can display federated content.

**Acceptance Criteria:**
- [ ] `POST /users/{username}/inbox` — accept activities
- [ ] Verify HTTP signatures
- [ ] Handle Follow, Like, Create, Update, Delete
- [ ] Store remote activities in local database

---

### 6.5 Outbox (Delivery)
**Priority:** P0 | **Estimate:** L

**Story:** As a system, I want to deliver activities to followers' inboxes so that my content is federated.

**Acceptance Criteria:**
- [ ] On activity creation, deliver Create activity to all followers
- [ ] HTTP Signatures for authentication
- [ ] Queue-based delivery (async)
- [ ] Retry on failure

---

### 6.6 Follow Handling
**Priority:** P0 | **Estimate:** M

**Story:** As a system, I want to handle follow/unfollow activities so that the social graph works across instances.

**Acceptance Criteria:**
- [ ] Receive Follow → send Accept
- [ ] Receive Undo Follow → remove follower
- [ ] Store remote followers in `actor_follows` table

---

### 6.7 Like/Kudos Federation
**Priority:** P1 | **Estimate:** M

**Story:** As a system, I want to federate likes so that users can give kudos across instances.

**Acceptance Criteria:**
- [ ] Send Like activity to activity author's inbox
- [ ] Receive Like → store as local like
- [ ] Receive Undo Like → remove like

---

### 6.8 Exercise Object
**Priority:** P0 | **Estimate:** L

**Story:** As a system, I want to publish Exercise objects (fedisport vocabulary) so that activities are correctly represented in the fediverse.

**Acceptance Criteria:**
- [ ] Exercise object with: type, attributedTo, activityType, startedAt, name, content, routeUrl, statsUrl
- [ ] Serve routeUrl as GeoJSON
- [ ] Serve statsUrl as JSON metrics
- [ ] Respect visibility (public, followers-only)

---

## Epic 7: PWA Frontend

> React-based Progressive Web App.

### 7.1 PWA Setup
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to install the app on my device so that I can access it like a native app.

**Acceptance Criteria:**
- [ ] React + TypeScript + Vite
- [ ] Service worker for offline support
- [ ] Web app manifest with icons
- [ ] Install prompt handling
- [ ] Responsive design (mobile-first)

---

### 7.2 Authentication UI
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to register and log in via the web interface.

**Acceptance Criteria:**
- [ ] Registration form (email, username, password)
- [ ] Login form
- [ ] JWT token storage
- [ ] Protected routes
- [ ] Logout

---

### 7.3 Activity List
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to see my activities in a list so that I can browse my history.

**Acceptance Criteria:**
- [ ] List of activities with: title, type, date, distance, duration
- [ ] Pagination
- [ ] Filter by type
- [ ] Link to activity detail

---

### 7.4 Activity Detail
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to see full details of an activity including map and stats.

**Acceptance Criteria:**
- [ ] Activity title, description, type, date
- [ ] Map showing route (Leaflet)
- [ ] Stats: distance, duration, elevation, speed
- [ ] Like button with count
- [ ] Comments section
- [ ] Edit/delete buttons (owner only)

---

### 7.5 Activity Creation
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to create activities manually or upload files.

**Acceptance Criteria:**
- [ ] Manual entry form (type, title, date, distance, duration)
- [ ] File upload (drag & drop)
- [ ] Progress indicator for upload
- [ ] Redirect to activity detail on success

---

### 7.6 User Profile
**Priority:** P0 | **Estimate:** M

**Story:** As a user, I want to view and edit my profile.

**Acceptance Criteria:**
- [ ] Profile page with: display name, avatar, bio
- [ ] Activity count, total distance
- [ ] Edit profile form
- [ ] Follow/unfollow button (for other users)

---

### 7.7 Feed
**Priority:** P0 | **Estimate:** L

**Story:** As a user, I want to see a feed of activities from people I follow.

**Acceptance Criteria:**
- [ ] Feed page showing followed users' activities
- [ ] Each item: user avatar, activity summary, map thumbnail
- [ ] Like/comment actions
- [ ] Infinite scroll or pagination

---

### 7.8 Settings
**Priority:** P1 | **Estimate:** M

**Story:** As a user, I want to configure my account and preferences.

**Acceptance Criteria:**
- [ ] Profile settings (name, bio, avatar)
- [ ] Privacy settings (default visibility, privacy zone)
- [ ] Account settings (email, password)
- [ ] Danger zone (delete account)

---

## Epic 8: Analytics

> Training analysis and personal records.

### 8.1 Personal Records
**Priority:** P1 | **Estimate:** M

**Story:** As a user, I want to see my personal records so that I can track improvements.

**Acceptance Criteria:**
- [ ] Fastest time for common distances (5K, 10K, half marathon, marathon)
- [ ] Longest ride/run
- [ ] Highest elevation gain
- [ ] Best power (if available)
- [ ] Display on profile or dashboard

---

### 8.2 Training Zones
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to see my heart rate and power zones so that I can train effectively.

**Acceptance Criteria:**
- [ ] Configure FTP (for power zones)
- [ ] Configure max heart rate (for HR zones)
- [ ] Calculate zones based on settings
- [ ] Show zone distribution per activity

---

### 8.3 Training Load
**Priority:** P2 | **Estimate:** L

**Story:** As a user, I want to see my training load (TSS, CTL, ATL) so that I can manage fatigue.

**Acceptance Criteria:**
- [ ] Calculate TSS for each activity
- [ ] Calculate CTL (Chronic Training Load)
- [ ] Calculate ATL (Acute Training Load)
- [ ] Show Training Stress Balance (TSB)
- [ ] Chart over time

---

### 8.4 Heatmap
**Priority:** P3 | **Estimate:** M

**Story:** As a user, I want to see a heatmap of my activities so that I can visualize my riding areas.

**Acceptance Criteria:**
- [ ] Calendar heatmap (like GitHub contributions)
- [ ] Activity density map (geographic)
- [ ] Filter by date range

---

## Epic 9: Gear Tracking

> Equipment management.

### 9.1 Gear CRUD
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to add and manage my gear (bikes, shoes) so that I can track usage.

**Acceptance Criteria:**
- [ ] `POST /api/gear` — create gear
- [ ] `GET /api/gear` — list gear
- [ ] `PUT /api/gear/{id}` — update gear
- [ ] `DELETE /api/gear/{id}` — delete gear
- [ ] Gear types: bike, shoe, wetsuit, other

---

### 9.2 Gear Assignment
**Priority:** P2 | **Estimate:** S

**Story:** As a user, I want to assign gear to activities so that mileage is tracked.

**Acceptance Criteria:**
- [ ] Select gear when creating activity
- [ ] Auto-assign default gear by activity type
- [ ] Update gear mileage on activity save

---

### 9.3 Gear Statistics
**Priority:** P3 | **Estimate:** S

**Story:** As a user, I want to see mileage per gear item so that I know when to replace.

**Acceptance Criteria:**
- [ ] Total distance per gear
- [ ] Activity count per gear
- [ ] Last used date
- [ ] Maintenance reminder (optional)

---

## Epic 10: Segments & Leaderboards

> Competitive features.

### 10.1 Segment Definition
**Priority:** P3 | **Estimate:** L

**Story:** As a user, I want to define segments (climbs, sprints) so that I can compete.

**Acceptance Criteria:**
- [ ] `segments` table with: id, name, start_lat, start_lon, end_lat, end_lon, type
- [ ] Segment types: climb, sprint, flat
- [ ] Match activities to segments (geospatial query)
- [ ] `POST /api/segments` — create segment

---

### 10.2 Leaderboards
**Priority:** P3 | **Estimate:** M

**Story:** As a user, I want to see segment leaderboards so that I can compare with others.

**Acceptance Criteria:**
- [ ] Leaderboard per segment (sorted by time)
- [ ] Filter by: all time, this year, this month
- [ ] Show rank, time, date
- [ ] Highlight own efforts

---

### 10.3 KOM/QOM
**Priority:** P3 | **Estimate:** S

**Story:** As a user, I want to see if I'm KOM/QOM on a segment so that I can celebrate.

**Acceptance Criteria:**
- [ ] Display KOM/QOM holder on segment page
- [ ] Notification when achieving KOM/QOM
- [ ] Badge on profile

---

## Epic 11: Mobile & Polish

> Final polish and mobile optimization.

### 11.1 Push Notifications
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want push notifications for follows, likes, and comments.

**Acceptance Criteria:**
- [ ] Web Push API integration
- [ ] Service worker handles push events
- [ ] User can enable/disable notifications
- [ ] Notification types: follow, like, comment

---

### 11.2 Offline Support
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to view cached activities offline.

**Acceptance Criteria:**
- [ ] Cache recent activities in IndexedDB
- [ ] Show offline indicator
- [ ] Queue actions for when online

---

### 11.3 Data Export
**Priority:** P2 | **Estimate:** M

**Story:** As a user, I want to export my data so that I can back up or migrate.

**Acceptance Criteria:**
- [ ] Export as GPX
- [ ] Export as FIT
- [ ] Export as JSON (full data)
- [ ] Export as CSV (tabular)

---

### 11.4 Activity Templates
**Priority:** P3 | **Estimate:** S

**Story:** As a user, I want to save activity templates so that I can quickly create similar activities.

**Acceptance Criteria:**
- [ ] Save activity as template
- [ ] Create activity from template
- [ ] Manage templates

---

## Epic 12: Instance Administration

> Managing a Lièvre instance.

### 12.1 Instance Settings
**Priority:** P2 | **Estimate:** M

**Story:** As an admin, I want to configure instance settings so that I can customize my server.

**Acceptance Criteria:**
- [ ] Instance name and description
- [ ] Registration mode (open, invite, closed)
- [ ] Default user settings
- [ ] Federation policies

---

### 12.2 User Management
**Priority:** P2 | **Estimate:** M

**Story:** As an admin, I want to manage users so that I can maintain the community.

**Acceptance Criteria:**
- [ ] List all users
- [ ] Suspend/unsuspend users
- [ ] Delete users
- [ ] View user activity

---

### 12.3 Blocklists
**Priority:** P2 | **Estimate:** S

**Story:** As an admin, I want to block instances and users so that I can prevent abuse.

**Acceptance Criteria:**
- [ ] Block instance (stop all federation)
- [ ] Block user (prevent login and federation)
- [ ] Import/export blocklists
- [ ] Community blocklist sync

---

## Milestones

### MVP (v0.1.0)
**Target:** 3 months

Epics included:
- ✅ Epic 1: Foundation (COMPLETE)
- ⏳ Epic 2: File Import (GPX only)
- ⏳ Epic 3: Activity Processing
- ⏳ Epic 4: Maps (basic)
- ⏳ Epic 5: Social (follow, like, feed)
- ⏳ Epic 6: Federation (basic)
- ⏳ Epic 7: PWA (basic)

### Beta (v0.2.0)
**Target:** 6 months

Add:
- Epic 2: FIT, TCX, Batch import
- Epic 5: Comments, notifications
- Epic 7: Full PWA features
- Epic 8: Basic analytics

### v1.0.0
**Target:** 12 months

Add:
- Epic 9: Gear tracking
- Epic 10: Segments
- Epic 11: Polish, export
- Epic 12: Instance admin

---

*Last updated: 2026-08-10*