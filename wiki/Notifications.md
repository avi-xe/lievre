# Notifications

Stay updated on activity from your training community.

## Types of Notifications

| Trigger | Notification | Example |
|---------|-------------|---------|
| **Follow** | Someone follows you | "alice followed you" |
| **Like** | Someone likes your activity | "bob liked your activity" |
| **Comment** | Someone comments on your activity | "charlie commented on your activity" |

## Viewing Notifications

1. Click **Notifications** in the nav bar
2. See all your notifications, newest first
3. Each notification shows:
   - Actor name (clickable to profile)
   - Action type
   - Related activity (if applicable)
   - Timestamp

## Notification Format

### Follow Notification

```
👤 alice followed you
   just now
```

### Like Notification

```
❤️ bob liked your activity
   Morning Ride
   just now
```

### Comment Notification

```
💬 charlie commented on your activity
   Nice ride!
   Morning Ride
   just now
```

## Managing Notifications

### Mark as Read

- Click the checkmark on a notification
- It's marked as read (less prominent)

### Mark All as Read

1. Click **Mark all read** at the top of the notifications page
2. All unread notifications are marked as read

### No Bulk Delete

Currently, notifications can't be deleted individually or in bulk. This may be added in a future release.

## Federated Notifications

When someone from a remote instance interacts with your content:

| Remote Action | Your Notification |
|---------------|-------------------|
| Follows you | ✅ "X followed you" |
| Likes your activity | ✅ "X liked your activity" |
| Comments | ❌ Not federated yet |

### Remote Actor Names

Remote actors are identified by their full address:

```
alice@mastodon.social followed you
```

The name is clickable and leads to their profile on the remote instance.

## Real-Time Notifications

Currently, notifications are loaded when you visit the page. Real-time push notifications (via WebSocket) are planned for a future release.

### Workaround

Refresh the notifications page to see new notifications.

## Notification Settings

Currently, all notifications are enabled. Per-type notification preferences are planned for a future release.

---

**See also:** [Following & Social](Following-Social.md) | [Your Feed](Your-Feed.md)
