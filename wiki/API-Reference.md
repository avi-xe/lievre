# API Reference

Lièvre exposes a REST API for all operations. All endpoints require authentication unless noted.

## Authentication

All requests include an `Authorization` header:

```
Authorization: Bearer <jwt_token>
```

Obtain a token via `POST /api/auth/login`.

## Endpoints

### Auth

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/auth/register` | ❌ | Create account |
| POST | `/api/auth/login` | ❌ | Get JWT token |
| GET | `/api/users/me` | ✅ | Current user info |

### Activities

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/activities` | ✅ | Create activity |
| GET | `/api/activities` | ✅ | List user's activities |
| GET | `/api/activities/:id` | ✅ | Get activity detail |
| PUT | `/api/activities/:id` | ✅ | Update activity |
| DELETE | `/api/activities/:id` | ✅ | Delete activity |
| GET | `/api/users/:id/activities` | ✅ | List user's activities |

### Social

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/users/:id/follow` | ✅ | Follow user |
| DELETE | `/api/users/:id/follow` | ✅ | Unfollow user |
| GET | `/api/users/:id/follow-status` | ✅ | Check follow status |
| GET | `/api/users/:id/followers` | ✅ | List followers |
| GET | `/api/users/:id/following` | ✅ | List following |
| POST | `/api/activities/:id/like` | ✅ | Like activity (idempotent) |
| DELETE | `/api/activities/:id/like` | ✅ | Unlike activity |
| GET | `/api/activities/:id/likes` | ✅ | List likes + liked status |
| POST | `/api/activities/:id/comments` | ✅ | Add comment |
| GET | `/api/activities/:id/comments` | ✅ | List comments |
| DELETE | `/api/comments/:id` | ✅ | Delete comment |

### Feed

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/feed` | ✅ | Personal feed |
| GET | `/api/feed/public` | ❌ | Public feed |

### Import

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/import/gpx` | ✅ | Upload GPX file |
| POST | `/api/import/tcx` | ✅ | Upload TCX file |
| POST | `/api/import/strava` | ✅ | Upload Strava ZIP |

### Federation

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/.well-known/webfinger` | ❌ | WebFinger discovery |
| GET | `/users/:username` | ❌ | Actor profile (JSON-LD) |
| POST | `/users/:username/inbox` | ❌ | S2S: Receive activities |
| GET | `/users/:username/outbox` | ❌ | Activity feed |
| GET | `/ns/fedisport` | ❌ | Fedisport JSON-LD context |
| GET | `/api/exercises/:id/route` | ❌ | GeoJSON route |
| GET | `/api/exercises/:id/stats` | ❌ | Fitness metrics |

### Notifications

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/notifications` | ✅ | List notifications |
| PUT | `/api/notifications/:id/read` | ✅ | Mark as read |
| PUT | `/api/notifications/read-all` | ✅ | Mark all as read |

### Users

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/users` | ✅ | List all users |
| GET | `/api/users/:id` | ✅ | Get user by ID |

## Request/Response Examples

### Create Activity

```bash
curl -X POST http://localhost/api/activities \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "activity_type": "ride",
    "title": "Morning Ride",
    "started_at": "2026-08-14T08:00:00Z",
    "duration_seconds": 5400,
    "distance_meters": 42000,
    "visibility": "public"
  }'
```

Response:

```json
{
  "id": "abc123",
  "user_id": "user456",
  "activity_type": "ride",
  "title": "Morning Ride",
  "started_at": "2026-08-14T08:00:00Z",
  "duration_seconds": 5400,
  "distance_meters": 42000.0,
  "visibility": "public",
  "created_at": "2026-08-14T09:30:00Z"
}
```

### Get Likes

```bash
curl http://localhost/api/activities/abc123/likes \
  -H "Authorization: Bearer $TOKEN"
```

Response:

```json
{
  "likes": [
    {
      "id": "like789",
      "activity_id": "abc123",
      "user_id": "user456",
      "created_at": "2026-08-14T10:00:00Z"
    }
  ],
  "count": 1,
  "liked": true
}
```

## Error Responses

| Status | Meaning |
|--------|---------|
| 400 | Bad request (invalid data) |
| 401 | Unauthorized (missing/invalid token) |
| 403 | Forbidden (insufficient permissions) |
| 404 | Not found |
| 500 | Internal server error |

Error format:

```json
{
  "error": "Activity not found"
}
```

---

**See also:** [Architecture Overview](Architecture-Overview.md) | [Contributing](Contributing.md)
