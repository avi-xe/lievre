# Following & Social

Lièvre is a social platform. Follow athletes, like their activities, and discuss together.

## Following Users

### From Lièvre

1. Go to **Users** in the nav bar
2. Find the athlete you want to follow
3. Click their profile
4. Click **Follow**

### From Mastodon/Lemmy/Other

You don't need a Lièvre account to follow athletes! See [Federation Guide](Federation-Guide.md).

### What Happens When You Follow

- Their public and followers-only activities appear in your feed
- You receive notifications when they post new activities
- They see they have a new follower

### Unfollow

1. Go to the user's profile
2. Click **Unfollow**

## Liking Activities

### Like an Activity

1. Go to an activity detail page
2. Click the **Like** button
3. The like count updates

### Unlike

1. Go to the activity
2. Click **Unlike** (same button, toggles)

### Likes Are Idempotent

Liking the same activity twice doesn't create duplicates — it's safe to click multiple times.

### Federated Likes

When you like an activity from a remote instance:
- The like is sent to the origin server
- The athlete gets notified
- The like count updates

When someone from another instance likes your activity:
- You get a notification
- The like count includes remote likes

## Commenting

### Add a Comment

1. Go to an activity detail page
2. Type your comment in the input field
3. Press Enter or click the send button

### Delete a Comment

1. Go to the activity
2. Find your comment
3. Click the delete button (only your own comments)

### Federated Comments

Comments are currently local only. Remote instances see the like count but not individual comments. This may change in future versions.

## Notifications

You receive notifications for:

| Event | Notification |
|-------|-------------|
| Someone follows you | "X followed you" |
| Someone likes your activity | "X liked your activity" |
| Someone comments | "X commented on your activity" |

### Viewing Notifications

1. Click **Notifications** in the nav bar
2. See all your notifications
3. Click **Mark all read** to clear

### Notification Details

- Notifications show the actor's name
- Clicking takes you to the relevant activity/profile
- Unread notifications are highlighted

## User Profiles

### Viewing a Profile

1. Click a username anywhere (feed, comments, users list)
2. See their profile with:
   - Username and email
   - Activity list
   - Follow/unfollow button

### Your Profile

- Shows all your activities
- Shows your follower/following counts
- Other users see your public activities

---

**See also:** [Your Feed](Your-Feed.md) | [Notifications](Notifications.md)
