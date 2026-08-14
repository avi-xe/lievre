# Exercise Objects

The **Exercise** object is the core federated content type in Lièvre. It extends ActivityStreams 2.0 with fitness-specific properties using the [fedisport vocabulary](https://github.com/fedisport/vocabulary).

## Why Exercise Objects?

Traditional ActivityPub platforms (Mastodon, Lemmy) use `Note` objects for posts. But fitness activities need structured data:

- **Activity type** — ride, run, swim, etc.
- **Route** — GPS coordinates for map display
- **Metrics** — distance, duration, elevation, power, heart rate

The fedisport `Exercise` type provides this while remaining backward-compatible with non-fitness clients.

## What Remote Users See

### On Mastodon

A Lièvre ride appears as a post with:
- Activity type emoji (🚴 ride, 🏃 run, 🏊 swim)
- Title and description
- Links to route map and stats
- hashtags like `#cycling #ride`

### On Lièvre

A full Exercise object with:
- Interactive Leaflet map
- Elevation profile chart
- Detailed metrics (distance, duration, elevation, speed)
- Like and comment sections

## Wire Format

### Exercise Object

```json
{
  "@context": [
    "https://www.w3.org/ns/activitystreams",
    "https://fedisport.github.io/vocabulary/context.jsonld"
  ],
  "type": "Exercise",
  "id": "https://lievre.example.com/exercises/abc123",
  "attributedTo": "https://lievre.example.com/users/alice",
  "activityType": "ride",
  "startedAt": "2026-08-14T08:15:00Z",
  "name": "Morning Ride through the Alps",
  "content": "Perfect weather, legs felt strong!",
  "routeUrl": "https://lievre.example.com/api/exercises/abc123/route",
  "statsUrl": "https://lievre.example.com/api/exercises/abc123/stats",
  "published": "2026-08-14T09:30:00Z",
  "to": ["https://www.w3.org/ns/activitystreams#Public"],
  "cc": ["https://lievre.example.com/users/alice/followers"]
}
```

### Activity Type Mapping

| Lièvre Type | Fedisport Value | Emoji |
|-------------|----------------|-------|
| ride | `ride` | 🚴 |
| run | `run` | 🏃 |
| swim | `swim` | 🏊 |
| walk | `walk` | 🚶 |
| hike | `hike` | 🥾 |
| virtual_ride | `virtualRide` | 🚴‍♂️ |

### Route (GeoJSON)

Fetched from `routeUrl`:

```json
{
  "type": "FeatureCollection",
  "features": [{
    "type": "Feature",
    "geometry": {
      "type": "LineString",
      "coordinates": [
        [13.404954, 52.520008, 34.0],
        [13.405101, 52.520212, 35.2],
        ...
      ]
    }
  }]
}
```

### Stats (JSON)

Fetched from `statsUrl`:

```json
{
  "distance": 42000.0,
  "duration": 5400,
  "elevationGain": 350.0,
  "avgSpeed": 7.78,
  "maxSpeed": 12.5
}
```

All fields are optional — only available data is included.

## Privacy Controls

The `routeUrl` and `statsUrl` endpoints respect the activity's visibility:

| Visibility | routeUrl | statsUrl |
|------------|----------|----------|
| **Public** | ✅ Accessible | ✅ Accessible |
| **Followers** | ✅ For followers | ✅ For followers |
| **Private** | ❌ 403 | ❌ 403 |

Remote servers can link to these URLs, but access is controlled by the origin server.

## JSON-LD Context

Lièvre serves the fedisport context at `/ns/fedisport`:

```json
{
  "@context": {
    "fedisport": "https://fedisport.github.io/vocabulary/ns#",
    "Exercise": "fedisport:Exercise",
    "activityType": "fedisport:activityType",
    "startedAt": "fedisport:startedAt",
    "routeUrl": "fedisport:routeUrl",
    "statsUrl": "fedisport:statsUrl"
  }
}
```

## Receiving Remote Exercises

When a remote fedisport-aware server sends an Exercise object, Lièvre:

1. Verifies the HTTP signature
2. Extracts the Exercise properties
3. Maps `activityType` back to internal types
4. Creates a local activity record
5. Optionally fetches route and stats from the URLs

This means activities from other fedisport instances appear in Lièvre with full detail.

---

**See also:** [How Federation Works](How-Federation-Works.md) | [Fedisport Vocabulary](Fedisport-Vocabulary.md)
