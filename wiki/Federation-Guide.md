# Federation Guide

Lièvre is part of the fediverse. You can follow Lièvre athletes from Mastodon, Lemmy, PeerTube, or any ActivityPub-compatible server — and they can follow you back.

## Following a Lièvre User from Another Platform

### From Mastodon

1. Open your Mastodon instance
2. In the search bar, type the full address: `@username@lievre.yourdomain.com`
3. Click the profile that appears
4. Click **Follow**

That's it! The athlete's activities (rides, runs, swims) will appear in your Mastodon timeline as posts.

### From Lemmy

1. Open your Lemmy instance
2. Search for: `@username@lievre.yourdomain.com`
3. Click the user profile
4. Click **Follow**

### From PeerTube / Pixelfed / Other

The process is the same — search for the full address `@username@lievre.yourdomain.com` and follow.

### From the Lièvre Web UI

1. Go to **Users** in the nav bar
2. Find the athlete you want to follow
3. Click their profile
4. Click **Follow**

## What Gets Federated

When a Lièvre athlete creates an activity, it's federated as an **Exercise object** (fedisport vocabulary):

| Field | What Remote Users See |
|-------|----------------------|
| **Activity Type** | ride, run, swim, walk, hike, etc. |
| **Title** | "Morning Ride through the Alps" |
| **Start Time** | When the activity started |
| **Route** | Map of the route (via link) |
| **Stats** | Distance, duration, elevation (via link) |
| **Description** | Any notes the athlete added |

### Example: What a Mastodon User Sees

When Alice (on Lièvre) creates a ride, Bob (on Mastodon) sees:

```
🚴 Morning Ride through the Alps

Activity: ride
Started: 2026-08-14 08:15 UTC

📍 View route: https://lievre.example.com/api/exercises/abc123/route
📊 Stats: https://lievre.example.com/api/exercises/abc123/stats

# cycling #ride #activitypub
```

## What Doesn't Get Federated (Yet)

- ❌ Private activities (respect privacy)
- ❌ Comments on activities (future)
- ❌ Gear/equipment data
- ❌ Detailed power/HR zones

## Likes Across Instances

When you like a Lièvre activity from Mastodon:

1. Mastodon sends a `Like` activity to Lièvre
2. Lièvre records the like and notifies the athlete
3. The like count updates on the activity

When a Lièvre user likes your Mastodon post:

1. Lièvre sends a `Like` activity to your Mastodon
2. You see the like notification on Mastodon

## Discovering Lièvre Users

### On Mastodon

Search for hashtags like `#cycling`, `#running`, `#activitypub` — federated Lièvre activities may appear.

### On Fedi.Directory

If the Lièvre instance opts in, users appear in [Fedi.Directory](https://fedi.directory) for discovery.

### Direct Address

If someone tells you their Lièvre handle, search for `@username@lievre.yourdomain.com` on your instance.

## Troubleshooting Federation

### "Can't find user" on Mastodon

- Wait 5 minutes — federation can be slow on first follow
- Verify the user exists: `curl -s https://lievre.yourdomain.com/users/username`
- Check WebFinger: `curl -s "https://lievre.yourdomain.com/.well-known/webfinger?resource=acct:username@lievre.yourdomain.com"`

### Activities don't appear on Mastodon

- Check that the activity is `public` or `followers` visibility
- Verify the outbox: `curl -s https://lievre.yourdomain.com/users/username/outbox`
- Check Lièvre logs: `docker compose logs app | grep federation`

### Like doesn't register

- Likes are idempotent — liking twice doesn't create duplicates
- Check the activity exists on Lièvre first

## Technical Details

For developers and instance admins:

- [How Federation Works](How-Federation-Works.md) — Protocol details
- [Exercise Objects](Exercise-Objects.md) — Wire format
- [Fedisport Vocabulary](Fedisport-Vocabulary.md) — The standard

---

**Next:** [How Federation Works](How-Federation-Works.md) — Deep dive into the protocol
