# Lièvre — Ubiquitous Language

This glossary defines the domain vocabulary used throughout Lièvre. All code, docs, and conversations should use these terms consistently.

---

## Core Domain

### Activity
A recorded physical exercise (ride, run, swim, etc.). The central entity in Lièvre. Each activity has a type, time range, metrics, and optionally a route.

### Exercise
The fedisport vocabulary term for Activity. Used in ActivityPub federation. When discussing federation, prefer "Exercise"; internally, use "Activity."

### Activity Type
The sport or movement: `ride`, `run`, `swim`, `walk`, `hike`, `virtual-ride`, etc. Defined by the fedisport vocabulary.

### Route
The geographical path of an activity, stored as a GeoJSON `LineString`. Contains `[longitude, latitude]` or `[longitude, latitude, elevation]` coordinates.

### Stats
Fitness metrics for an activity: distance, duration, elevation, power, heart rate, pace, cadence. Served from `statsUrl` in federation.

---

## Users & Identity

### User
A local account on a Lièvre instance. Has email, username, password. May own activities.

### Actor
An ActivityPub entity (local or remote). Local actors are linked to Users. Remote actors are cached representations of users on other instances.

### Instance
A running Lièvre server (e.g., `lievre.social`). May be single-user or multi-user.

### Handle
A user's address in the fediverse: `@username@instance.social`. Used for discovery and federation.

---

## Federation

### Federation
The exchange of activities between Lièvre instances and other ActivityPub servers (Mastodon, PeerTube, etc.).

### WebFinger
Discovery protocol for finding actors. Endpoint: `/.well-known/webfinger`.

### Inbox
Endpoint where a server receives activities from other servers.

### Outbox
Endpoint where a server publishes activities for followers to see.

### Follow
A relationship where one actor subscribes to another's activities. Can be local or federated.

### Create Activity
An ActivityPub activity that delivers new content (e.g., a new Exercise).

### Like Activity
An ActivityPub activity representing kudos/appreciation.

### Announce Activity
An ActivityPub activity representing a boost/repost.

---

## Cycling Domain

### Peloton
The main group of riders in a race. Not to be confused with the fitness company.

### Lièvre
The pacemaker — a rider who sets a fast pace but isn't expected to win. Our project's namesake.

### FTP (Functional Threshold Power)
The maximum average power a cyclist can sustain for ~1 hour. Used for training zones.

### Power Zone
Training zones based on FTP: Z1 (recovery) to Z5 (anaerobic).

### TSS (Training Stress Score)
A metric quantifying the workload of a training session.

### Segment
A specific section of road (e.g., a climb, sprint). Has leaderboards.

### KOM / QOM
King/Queen of the Mountain — fastest rider on a segment.

### Gruppetto
The main pack of riders, especially those just trying to finish.

---

## Data Formats

### GPX (GPS Exchange Format)
XML-based format for GPS data. Contains tracks, waypoints, and routes.

### FIT (Flexible and Interoperable Data Transfer)
Binary format used by Garmin and other devices. Richer than GPX (includes power, HR, cadence).

### TCX (Training Center XML)
XML format from Garmin. Contains workout data.

### GeoJSON
JSON format for geographic data. Used for route visualization.

### Strava Export
ZIP archive containing activities exported from Strava.

---

## Technical Terms

### PWA (Progressive Web App)
Web application that can be installed on a device, works offline, and sends push notifications.

### Service Worker
JavaScript file that enables offline support, caching, and push notifications.

### PostGIS
PostgreSQL extension for geospatial queries (storing routes, finding nearby activities).

### Privacy Zone
A configurable radius around start/end points that is blurred in shared routes.

### Gear
Equipment tracked by the user: bikes, shoes, wetsuits. Activities can be linked to gear.

### Segment
A named section of road with a leaderboard. (See cycling domain.)

---

## ActivityPub Object Types

### Person
Standard ActivityPub actor type for users.

### Exercise
Fedisport extension for fitness activities. Our primary content type.

### Note
Standard ActivityPub content type. Used for comments on activities.

### Create / Update / Delete
ActivityPub activities for content lifecycle.

### Follow / Accept / Reject
ActivityPub activities for relationship management.

### Like / Announce
ActivityPub activities for social interactions.

---

## Conventions

| Context | Term | Meaning |
|---------|------|---------|
| UI | "Activity" | A workout shown in the feed |
| Federation | "Exercise" | The ActivityPub object type |
| Code | `activity` | The Rust struct / DB table |
| Code | `exercise` | The AP object type |
| Database | `activities` | Table storing local activities |
| API | `/api/activities` | REST endpoint for CRUD |
| Federation | `/exercises/{id}` | AP endpoint for remote access |

---

*Last updated: 2026-08-10*
