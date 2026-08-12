-- Add federation fields to likes table for remote likes
-- Make user_id nullable to support external likes from remote actors

-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
-- First, create a new table with the updated schema
CREATE TABLE IF NOT EXISTS likes_new (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    remote_actor_url TEXT,
    object_url TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(activity_id, user_id),
    UNIQUE(activity_id, remote_actor_url)
);

-- Copy existing data
INSERT INTO likes_new (id, activity_id, user_id, created_at)
SELECT id, activity_id, user_id, created_at FROM likes;

-- Drop old table
DROP TABLE likes;

-- Rename new table
ALTER TABLE likes_new RENAME TO likes;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS idx_likes_activity ON likes(activity_id);
CREATE INDEX IF NOT EXISTS idx_likes_user ON likes(user_id);
CREATE INDEX IF NOT EXISTS idx_likes_remote_actor ON likes(remote_actor_url);
CREATE INDEX IF NOT EXISTS idx_likes_object_url ON likes(object_url);
