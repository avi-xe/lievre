-- Follow relationships between users
CREATE TABLE IF NOT EXISTS follows (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    follower_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    following_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'accepted',  -- pending, accepted, rejected
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(follower_id, following_id)
);

CREATE INDEX IF NOT EXISTS idx_follows_follower ON follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_follows_following ON follows(following_id);

-- Likes (kudos) on activities
CREATE TABLE IF NOT EXISTS likes (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(activity_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_likes_activity ON likes(activity_id);
CREATE INDEX IF NOT EXISTS idx_likes_user ON likes(user_id);

-- Comments on activities
CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_comments_activity ON comments(activity_id);
CREATE INDEX IF NOT EXISTS idx_comments_user ON comments(user_id);
