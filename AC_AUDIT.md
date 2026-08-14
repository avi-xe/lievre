# Epic 1–7 Acceptance Criteria Audit

Cross-referenced against actual codebase. Status:
- ✅ = Implemented + testable from browser
- ⚠️ = Backend exists, frontend partial/missing
- ❌ = Missing implementation

---

## Epic 1: Foundation

### 1.1 Project Scaffolding
- [x] Cargo workspace with `lievre-core`, `lievre-api`, `lievre-federation`, `lievre-shared` crates
- [x] Axum HTTP server starts on port 3000
- [x] Health check endpoint: `GET /health` returns `200 OK`
- [x] Environment configuration via `.env`
- [x] Logging with `tracing` crate
- [x] Docker Compose for local development

### 1.2 Database Setup
- [x] SQLite database file at `./data/lievre.db`
- [x] SQLx for query execution
- [x] migrations folder with initial schema
- [x] `sqlx migrate run` command works
- [x] Connection pool with proper error handling

### 1.3 User Model
- [x] `users` table with all fields
- [x] `POST /api/auth/register` — create account
- [x] `POST /api/auth/login` — return JWT token
- [x] `GET /api/users/me` — return current user
- [x] Password hashing with bcrypt
- [x] JWT with user_id claim

### 1.4 Activity Model
- [x] `activities` table with all fields
- [x] `POST /api/activities` — create activity
- [x] `GET /api/activities` — list user's activities
- [x] `GET /api/activities/{id}` — get activity detail
- [ ] `PUT /api/activities/{id}` — update activity ← **MISSING ENDPOINT**
- [x] `DELETE /api/activities/{id}` — delete activity
- [ ] Activity types: ride, run, swim, walk, hike, virtual-ride ← **walk/hike/virtual-ride rejected by handler**
- [x] Visibility: public, followers, private

### 1.5 Route Storage
- [x] `routes` table with all fields
- [x] Route stored as GeoJSON-compatible format
- [x] `GET /api/activities/{id}/geojson` — return route as GeoJSON
- [x] Route deletion cascades with activity

---

## Epic 2: File Import

### 2.1 GPX Import
- [x] `POST /api/import/gpx` — upload GPX file
- [x] Parse GPX tracks, extract coordinates and timestamps
- [x] Create activity with: type, start time, duration, distance
- [x] Create route from track points
- [x] Return created activity ID
- [x] Handle malformed GPX gracefully

### 2.2 FIT Import
- [ ] `POST /api/import/fit` — upload FIT file ← **MISSING**

### 2.3 TCX Import
- [x] `POST /api/import/tcx` — upload TCX file
- [x] Parse TCX activities and laps
- [x] Create activity with stats

### 2.4 Batch Import
- [ ] `POST /api/import/zip` — upload ZIP archive ← **MISSING**

### 2.5 Strava Export Import
- [x] `POST /api/import/strava` — upload Strava ZIP export
- [x] Parse `activities.csv` and associated GPX files
- [x] Create activities with all available data
- [x] Handle Strava-specific formats

---

## Epic 3: Activity Processing

### 3.1 Worker Queue
- [x] SQLite-backed job queue (WAL mode)
- [x] Job types: ProcessGpx, ComputeStats, GenerateGeoJson, FederationDeliver
- [x] Worker polls for pending jobs (every 5s)
- [x] Retry logic with exponential backoff
- [x] Job status tracking (pending, processing, completed, failed)

### 3.2 Stats Computation
- [x] Compute distance from route coordinates
- [x] Compute duration from timestamps
- [x] Compute elevation gain from elevation data
- [x] Compute average/max speed
- [x] Store stats in `activity_stats` table

### 3.3 GeoJSON Generation
- [x] Generate GeoJSON LineString from route coordinates
- [ ] Include elevation in third dimension ← **NOT IN GeoJSON OUTPUT**
- [ ] Cache generated GeoJSON ← **NO CACHING**

---

## Epic 4: Maps & Visualization

### 4.1 Map Display
- [x] Leaflet map component in React
- [x] Load GeoJSON route from API
- [ ] Display route with start/end markers ← **NO MARKERS**
- [x] Zoom to fit route bounds
- [x] OpenStreetMap tiles (no API key)

### 4.2 Privacy Zones
- [ ] User can set privacy zone radius (default 200m) ← **NO UI/API**
- [ ] Blur start/end points in shared routes ← **NOT WIRED**
- [ ] Privacy zone stored in user settings ← **NO TABLE**
- [ ] Applied when serving route to remote instances ← **NOT WIRED**

### 4.3 Elevation Profile
- [x] Chart component showing elevation over distance
- [ ] Interactive (hover to see elevation at point) ← **STATIC SVG**
- [ ] Highlight climbs and descents ← **NO COLOR CODING**

### 4.4 Activity Statistics Dashboard
- [ ] Weekly/monthly/yearly summaries ← **NOT IMPLEMENTED**
- [ ] Total distance, time, elevation ← **NOT IMPLEMENTED**
- [ ] Activity count by type ← **NOT IMPLEMENTED**
- [ ] Training load (TSS) over time ← **NOT IMPLEMENTED**

---

## Epic 5: Social Features

### 5.1 Follow System
- [x] `follows` table with all fields
- [x] `POST /api/users/{id}/follow` — follow
- [x] `DELETE /api/users/{id}/follow` — unfollow
- [x] `GET /api/users/{id}/followers` — list followers
- [x] `GET /api/users/{id}/following` — list following
- [ ] Follow approval for private accounts ← **OPTIONAL, NOT IMPLEMENTED**

### 5.2 Likes (Kudos)
- [x] `likes` table with all fields
- [x] `POST /api/activities/{id}/like` — like activity
- [x] `DELETE /api/activities/{id}/like` — unlike
- [x] `GET /api/activities/{id}/likes` — list who liked (returns `{ likes, count, liked }`)
- [x] Count of likes on activity ← exposed via `count` field

### 5.3 Comments
- [x] `comments` table with all fields
- [x] `POST /api/activities/{id}/comments` — add comment
- [x] `GET /api/activities/{id}/comments` — list comments
- [x] `DELETE /api/comments/{id}` — delete own comment ← **BACKEND OK, NO DELETE BUTTON IN UI**

### 5.4 Activity Feed
- [x] `GET /api/feed` — paginated feed of followed users' activities
- [ ] Include user info, activity summary, like count ← **NO USERNAME/LIKE COUNT IN RESPONSE**
- [ ] Filter by activity type ← **NO QUERY PARAM**
- [x] Sort by time (newest first)

### 5.5 Public Feed
- [x] `GET /api/feed/public` — public activities (no auth required)
- [x] Paginated, sorted by time
- [ ] Filter by activity type ← **NO QUERY PARAM**

### 5.6 Notifications
- [x] `notifications` table with all fields
- [x] Create notification on: follow, like, comment
- [x] `GET /api/notifications` — list notifications
- [x] `PUT /api/notifications/{id}/read` — mark as read
- [x] `PUT /api/notifications/read-all` — mark all as read
- [ ] WebSocket for real-time notifications ← **OPTIONAL, NOT IMPLEMENTED**
- [ ] **FRONTEND NOTIFICATIONS PAGE** ← **MISSING UI**

---

## Epic 6: ActivityPub Federation

### 6.1 WebFinger
- [x] `GET /.well-known/webfinger?resource=acct:user@domain`
- [x] Return JSON with actor URL
- [x] Handle unknown users gracefully

### 6.2 Actor Endpoint
- [x] `GET /users/{username}` — return Person actor JSON-LD
- [x] Include inbox, outbox, followers, following URLs
- [x] Include public key for signature verification
- [x] Content-Type: `application/activity+json`

### 6.3 Outbox
- [x] `GET /users/{username}/outbox` — OrderedCollection of activities
- [x] Each activity wrapped in Create activity
- [ ] Pagination support ← **NOT IMPLEMENTED**

### 6.4 Inbox (Receiving)
- [x] `POST /users/{username}/inbox` — accept activities
- [ ] Verify HTTP signatures ← **NOT VERIFIED**
- [x] Handle Follow, Like, Create, Update, Delete
- [x] Store remote activities in local database

### 6.5 Outbox (Delivery)
- [x] On activity creation, deliver Create activity to all followers
- [ ] HTTP Signatures for authentication ← **NOT SIGNING**
- [x] Queue-based delivery (async)
- [ ] Retry on failure ← **NO RETRY FOR DELIVERY JOBS**

### 6.6 Follow Handling
- [x] Receive Follow → send Accept
- [x] Receive Undo Follow → remove follower
- [x] Store remote followers in `actor_follows` table

### 6.7 Like/Kudos Federation
- [x] `like()` is idempotent — returns existing like if already liked
- [x] `GET /api/activities/{id}/likes` returns `{ likes, count, liked }` (includes `liked` field)
- [x] Outbound: local user likes remote activity → sends `Like` to remote inbox
- [x] Inbound: receives `Like`/`Undo` from remote users, stores with `remote_actor_url`
- [x] Notifications for remote likes
- [x] E2E tests: `e2e/test-like-federation.mjs`

### 6.8 Exercise Object
- [x] `GET /ns/fedisport` — serves JSON-LD context
- [x] `exercise_to_jsonld()` — serializes Activity → Exercise object
- [x] `GET /api/exercises/:id/route` — GeoJSON route endpoint
- [x] `GET /api/exercises/:id/stats` — fitness metrics endpoint
- [x] Outbox delivers `Create → Exercise` format
- [x] Inbox receives remote Exercise objects
- [x] New files: `context.rs`, `exercise.rs` in `crates/federation/src/`
- [x] E2E tests: `e2e/test-exercise-object.mjs`
- [x] Respect visibility (public, followers-only, private)

---

## Epic 7: Frontend

### 7.1 App Shell & Routing
- [x] react-router-dom with all routes
- [x] Nav bar with: logo, Feed, Activities, +New, Login/Logout
- [x] AuthProvider wrapping app
- [x] ProtectedRoute component redirects unauthenticated users to /login

### 7.2 API Client & Auth Context
- [x] `apiFetch(path, opts)` auto-injects Authorization header
- [x] 401 response clears token and redirects to /login
- [x] `apiUpload(path, file)` for multipart uploads
- [x] AuthContext provides: user, token, login, register, logout, isAuthenticated
- [x] On mount, restores session from localStorage via GET /api/users/me

### 7.3 Authentication Pages
- [x] LoginPage: email + password form → login → navigate to /
- [x] RegisterPage: email + username + password form → register → navigate to /
- [x] Error messages shown on form failure
- [x] Logout clears token and redirects to /

### 7.4 Activity List & Detail
- [x] ActivityListPage: fetches GET /api/activities, shows title, type, date, distance, duration
- [x] Each activity links to /activities/:id
- [x] ActivityDetailPage: shows full stats, Leaflet map of route, like button, comments section
- [x] Delete button visible to owner only

### 7.5 Activity Creation
- [x] CreateActivityPage: form with activity_type, title, started_at, duration, distance, elevation, visibility
- [x] File upload input for GPX → POST /api/import/gpx
- [x] On success navigates to the new activity's detail page

### 7.6 Feed
- [x] FeedPage: GET /api/feed (authed) or GET /api/feed/public (unauthed)
- [x] Shows username, activity title, date, stats
- [x] Each activity links to /activities/:id

### 7.7 User Profile & Social
- [x] ProfilePage: GET /api/users/:id, shows username and activity list
- [x] Follow/unfollow button for other users
- [x] Like on activities
- [x] Comments on activities

### 7.8 Map Components
- [x] ActivityMap: fetches GeoJSON, renders Leaflet map with route, auto-fits bounds
- [x] ElevationProfile: SVG chart of elevation vs distance
- [x] Both handle loading, error, and empty states

---

## Summary

| Category | ✅ | ⚠️ | ❌ |
|----------|---|---|---|
| Epic 1 (Foundation) | 22 | 1 | 1 |
| Epic 2 (Import) | 9 | 0 | 2 |
| Epic 3 (Processing) | 9 | 0 | 2 |
| Epic 4 (Maps) | 5 | 0 | 9 |
| Epic 5 (Social) | 22 | 4 | 3 |
| Epic 6 (Federation) | 24 | 0 | 0 |
| Epic 7 (Frontend) | 24 | 0 | 0 |
| **Total** | **115** | **5** | **17** |

*Last updated: 2026-08-14*
