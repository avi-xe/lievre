# Lièvre — End-to-End Acceptance Criteria

Each scenario is **sandboxed**: creates its own data, cleans up after itself, runs independently.

---

## Test Infrastructure

### Setup
- Base URL: `http://localhost:80`
- Each test creates unique users with suffix: `test_{scenario}_{timestamp}`
- Auth via JWT tokens in `Authorization: Bearer <token>` header

### Cleanup
- Tests delete created resources after assertions
- Test users are marked for cleanup (or use unique emails per run)

### Isolation Rules
1. NO shared state between tests
2. NO ordering dependencies
3. Each test is self-contained
4. Tests can run in parallel safely

---

## 1. Authentication

### AC-AUTH-01: Register + Login Flow
```
Setup:  Generate unique email/username
Steps:
  1. POST /api/auth/register → 200, user created, token returned
  2. POST /api/auth/login with same credentials → 200, token returned
  3. GET /api/users/me with token → 200, user profile matches
Cleanup: None (user persists for test duration)
```

### AC-AUTH-02: Invalid Credentials
```
Setup:  Register a user
Steps:
  1. POST /api/auth/login with wrong password → 401
  2. POST /api/auth/login with non-existent email → 401
Cleanup: None
```

### AC-AUTH-03: Duplicate Registration
```
Setup:  Generate unique email
Steps:
  1. POST /api/auth/register → 200
  2. POST /api/auth/register with same email → 409 Conflict
Cleanup: None
```

---

## 2. Activities

### AC-ACT-01: CRUD Lifecycle
```
Setup:  Register user, get token
Steps:
  1. POST /api/activities → 201, activity created
  2. GET /api/activities/{id} → 200, matches creation data
  3. GET /api/activities → 200, list includes activity
  4. DELETE /api/activities/{id} → 200
  5. GET /api/activities/{id} → 404
Cleanup: Delete activity if test fails
```

### AC-ACT-02: List User Activities
```
Setup:  Register user, create 3 activities
Steps:
  1. GET /api/activities → 200, returns 3 activities
  2. Verify all belong to current user
Cleanup: Delete all activities
```

---

## 3. File Import

### AC-IMP-01: Import GPX
```
Setup:  Register user
Steps:
  1. POST /api/import/gpx with valid GPX → 200
  2. Response contains activity_id
  3. GET /api/activities/{activity_id} → 200, type=ride
  4. GET /api/activities/{activity_id}/geojson → 200, valid LineString
Cleanup: Delete activity
```

**Test GPX:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="test">
  <trk><name>Test Ride</name><trkseg>
    <trkpt lat="52.5200" lon="13.4050"><time>2024-01-15T08:00:00Z</time></trkpt>
    <trkpt lat="52.5210" lon="13.4060"><time>2024-01-15T08:01:00Z</time></trkpt>
    <trkpt lat="52.5220" lon="13.4070"><time>2024-01-15T08:02:00Z</time></trkpt>
  </trkseg></trk>
</gpx>
```

### AC-IMP-02: Import Invalid GPX
```
Setup:  Register user
Steps:
  1. POST /api/import/gpx with malformed XML → 400
  2. Verify error message is descriptive
Cleanup: None
```

---

## 4. GeoJSON

### AC-GEO-01: Get GeoJSON for Activity
```
Setup:  Register user, import GPX (creates activity + route)
Steps:
  1. GET /api/activities/{id}/geojson → 200
  2. Response is valid GeoJSON Feature
  3. geometry.type == "LineString"
  4. coordinates is array of [lon, lat] or [lon, lat, ele]
Cleanup: Delete activity
```

### AC-GEO-02: GeoJSON for Non-existent Activity
```
Setup:  None
Steps:
  1. GET /api/activities/nonexistent/geojson → 404
Cleanup: None
```

---

## 5. Social — Follow

### AC-FOL-01: Follow/Unfollow Lifecycle
```
Setup:  Register user A and user B
Steps:
  1. POST /api/users/{B}/follow (as A) → 200
  2. GET /api/users/{A}/following → 200, contains B
  3. GET /api/users/{B}/followers → 200, contains A
  4. DELETE /api/users/{B}/follow (as A) → 200
  5. GET /api/users/{A}/following → 200, empty
Cleanup: None (users persist)
```

### AC-FOL-02: Follow Count
```
Setup:  Register users A, B, C
Steps:
  1. B follows A, C follows A
  2. GET /api/users/{A}/followers → 200, count=2
  3. GET /api/users/{B}/following → 200, count=1
Cleanup: Unfollow
```

---

## 6. Social — Likes

### AC-LIK-01: Like/Unlike Lifecycle
```
Setup:  Register user (owner), create activity, register liker
Steps:
  1. POST /api/activities/{id}/like (as liker) → 200
  2. GET /api/activities/{id} → 200, likes_count=1
  3. DELETE /api/activities/{id}/like (as liker) → 200
  4. GET /api/activities/{id} → 200, likes_count=0
Cleanup: Delete activity
```

### AC-LIK-02: Duplicate Like Prevention
```
Setup:  Register user, create activity
Steps:
  1. POST /api/activities/{id}/like → 200
  2. POST /api/activities/{id}/like (same user) → 409 or idempotent
Cleanup: Delete activity
```

---

## 7. Social — Comments

### AC-COM-01: Add/Delete Comment
```
Setup:  Register user (owner), create activity, register commenter
Steps:
  1. POST /api/activities/{id}/comments → 200, comment created
  2. GET /api/activities/{id}/comments → 200, contains comment
  3. DELETE /api/comments/{comment_id} (as commenter) → 200
  4. GET /api/activities/{id}/comments → 200, empty
Cleanup: Delete activity
```

### AC-COM-02: Cannot Delete Others' Comments
```
Setup:  Register two users, create activity, add comment as user A
Steps:
  1. DELETE /api/comments/{id} (as user B) → 403
Cleanup: Delete activity, comment
```

---

## 8. Feed

### AC-FEED-01: Personal Feed
```
Setup:  Register user A (follower) and B (followed)
        B creates public activity
        A follows B
Steps:
  1. GET /api/feed (as A) → 200
  2. Contains B's activity
  3. Sorted by newest first
Cleanup: Unfollow, delete activity
```

### AC-FEED-02: Public Feed
```
Setup:  Register user, create public activity
Steps:
  1. GET /api/feed/public (no auth) → 200
  2. Contains the activity
Cleanup: Delete activity
```

### AC-FEED-03: Private Activity Not in Public Feed
```
Setup:  Register user, create private activity
Steps:
  1. GET /api/feed/public → 200
  2. Does NOT contain the private activity
Cleanup: Delete activity
```

---

## Running Tests

```bash
# Start server
docker compose up -d

# Run all e2e tests
cargo test --test e2e -- --ignored

# Run specific test
cargo test --test e2e test_health -- --ignored

# Run with output
cargo test --test e2e -- --ignored --nocapture
```

---

*Last updated: 2026-08-10*
