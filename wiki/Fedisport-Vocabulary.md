# Fedisport Vocabulary

Lièvre adopts the [fedisport vocabulary](https://github.com/fedisport/vocabulary) — an ActivityPub extension for representing fitness and sports activities in the fediverse.

## What is Fedisport?

Fedisport is a set of ActivityPub extensions that let fitness platforms share structured workout data across the fediverse. Instead of federating activities as generic `Note` objects (like Mastodon does), fedisport provides a dedicated `Exercise` type with sports-specific properties.

## Why Fedisport?

### For Users

- **Structured data** — Activity type, route, stats are preserved
- **Cross-platform** — Follow athletes from Mastodon, Lemmy, or any AP server
- **Privacy** — Route and stats URLs are access-controlled

### For Developers

- **Standardized** — Common vocabulary for fitness data
- **Extensible** — New activity types and metrics can be added
- **Backward-compatible** — Non-fitness clients see a readable post

## Core Type: Exercise

The `Exercise` object extends `as:Object`:

```json
{
  "type": "Exercise",
  "activityType": "ride",
  "startedAt": "2026-08-14T08:15:00Z",
  "name": "Morning Ride",
  "routeUrl": "https://lievre.example.com/api/exercises/abc123/route",
  "statsUrl": "https://lievre.example.com/api/exercises/abc123/stats"
}
```

## Properties

### Required

| Property | Type | Description |
|----------|------|-------------|
| `type` | String | Always `"Exercise"` |
| `attributedTo` | URI | The athlete's actor URL |
| `activityType` | String | Sport type (see below) |

### Optional

| Property | Type | Description |
|----------|------|-------------|
| `startedAt` | DateTime | When the activity started |
| `name` | String | Activity title |
| `content` | String | Description (HTML) |
| `routeUrl` | URI | Link to GeoJSON route |
| `statsUrl` | URI | Link to fitness metrics |
| `published` | DateTime | When the activity was published |

## Activity Types

| Value | Description | Emoji |
|-------|-------------|-------|
| `ride` | Road cycling | 🚴 |
| `gravel-ride` | Gravel cycling | 🚴 |
| `mountain-bike-ride` | Mountain biking | 🚵 |
| `e-bike-ride` | Electric bike | 🚴‍♂️ |
| `virtual-ride` | Indoor cycling | 🚴 |
| `run` | Road running | 🏃 |
| `trail-run` | Trail running | 🏃 |
| `virtual-run` | Treadmill | 🏃 |
| `swim` | Swimming | 🏊 |
| `walk` | Walking | 🚶 |
| `hike` | Hiking | 🥾 |

Unknown values are treated as opaque strings — the set of types may grow.

## Route (GeoJSON)

The `routeUrl` points to a GeoJSON document:

```json
{
  "type": "FeatureCollection",
  "features": [{
    "type": "Feature",
    "geometry": {
      "type": "LineString",
      "coordinates": [
        [13.404954, 52.520008, 34.0],
        [13.405101, 52.520212, 35.2]
      ]
    }
  }]
}
```

Coordinates are `[longitude, latitude]` or `[longitude, latitude, elevation]`.

## Stats (JSON)

The `statsUrl` points to a JSON document:

```json
{
  "distance": 42000.0,
  "duration": 5400,
  "elevationGain": 350.0,
  "avgSpeed": 7.78,
  "maxSpeed": 12.5,
  "avgHeartRate": 152,
  "maxHeartRate": 174,
  "avgPower": 230,
  "maxPower": 310,
  "normalizedPower": 245,
  "avgCadence": 85
}
```

All fields are optional — only available data is included.

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

## Version

Lièvre implements **fedisport v0.2** (pre-alpha). The vocabulary is stable enough to implement against, but may evolve before 1.0.

## Links

- [Fedisport Vocabulary](https://github.com/fedisport/vocabulary)
- [ActivityStreams 2.0](https://www.w3.org/TR/activitystreams-core/)
- [ActivityPub](https://www.w3.org/TR/activitypub/)

---

**See also:** [Exercise Objects](Exercise-Objects.md) | [How Federation Works](How-Federation-Works.md)
