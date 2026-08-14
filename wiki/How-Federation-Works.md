# How Federation Works

Lièvre implements the [ActivityPub](https://www.w3.org/TR/activitypub/) protocol for server-to-server federation, following the [fedisport vocabulary](https://github.com/fedisport/vocabulary) for fitness activities.

## Federation Flow

### Outbound: Sharing an Activity

```
Lièvre Server                          Remote Server (Mastodon)
     │                                        │
     │  1. User creates activity              │
     │     ↓                                  │
     │  2. Worker generates Exercise object   │
     │     ↓                                  │
     │  3. Wraps in Create activity           │
     │     ↓                                  │
     │  4. Signs with HTTP Signature          │
     │     ↓                                  │
     │  ─────── POST to inbox ──────────────▶ │
     │                                        │
     │                              5. Verify signature
     │                              6. Store activity
     │                              7. Show in timeline
```

### Inbound: Receiving a Follow

```
Remote Server (Mastodon)                   Lièvre Server
     │                                        │
     │  1. User clicks Follow                 │
     │     ↓                                  │
     │  ─────── Follow activity ────────────▶ │
     │                                        │
     │                    2. Verify signature  │
     │                    3. Store follow      │
     │                    4. Send Accept       │
     │     ◀────────────────────────────────  │
     │                                        │
     │  5. Mastodon confirms follow           │
```

## Protocol Stack

| Layer | Protocol | Purpose |
|-------|----------|---------|
| **Discovery** | WebFinger | Find users by `@username@domain` |
| **Identity** | ActivityPub Actors | Person profiles with public keys |
| **Content** | ActivityStreams 2.0 | Create, Like, Follow, Undo |
| **Fitness** | Fedisport Vocabulary | Exercise objects with sports data |
| **Security** | HTTP Signatures | Authenticate server-to-server requests |
| **Transport** | HTTPS | All communication over TLS |

## Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/.well-known/webfinger` | GET | User discovery |
| `/users/{username}` | GET | Actor profile (JSON-LD) |
| `/users/{username}/inbox` | POST | Receive activities |
| `/users/{username}/outbox` | GET | Activity feed |
| `/users/{username}/followers` | GET | Follower collection |
| `/users/{username}/following` | GET | Following collection |
| `/ns/fedisport` | GET | JSON-LD context |
| `/api/exercises/{id}/route` | GET | GeoJSON route |
| `/api/exercises/{id}/stats` | GET | Fitness metrics |

## Activity Types

Lièvre federates these ActivityStreams activities:

| Activity | Trigger | Direction |
|----------|---------|-----------|
| **Create** | User creates an activity | Outbound |
| **Follow** | User follows another athlete | Both |
| **Accept** | Accept a follow request | Outbound |
| **Like** | User likes an activity | Both |
| **Undo** | User unfollows or unlikes | Both |

## Exercise Object Format

The core federated object is the **Exercise** (fedisport vocabulary):

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
  "name": "Morning Ride",
  "routeUrl": "https://lievre.example.com/api/exercises/abc123/route",
  "statsUrl": "https://lievre.example.com/api/exercises/abc123/stats",
  "published": "2026-08-14T09:30:00Z",
  "to": ["https://www.w3.org/ns/activitystreams#Public"],
  "cc": ["https://lievre.example.com/users/alice/followers"]
}
```

## Privacy Model

| Visibility | Federated To | Who Can See |
|------------|-------------|-------------|
| **Public** | All followers + public outbox | Everyone |
| **Followers** | Only followers | Followers only |
| **Private** | Not federated | Owner only |

## HTTP Signatures

All server-to-server requests are signed using HTTP Signatures:

```
Date: Thu, 14 Aug 2026 12:00:00 GMT
Host: remote.example.com
Digest: SHA-256=...
Signature: keyId="https://lievre.example.com/users/alice#main-key",algorithm="rsa-sha256",headers="date host digest",signature="..."
```

The remote server verifies the signature against the public key published in the actor's `publicKey` field.

---

**See also:** [Exercise Objects](Exercise-Objects.md) | [Fedisport Vocabulary](Fedisport-Vocabulary.md)
