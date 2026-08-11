-- Add federation fields to users table
ALTER TABLE users ADD COLUMN public_key TEXT;
ALTER TABLE users ADD COLUMN private_key TEXT;
ALTER TABLE users ADD COLUMN inbox_url TEXT;
ALTER TABLE users ADD COLUMN outbox_url TEXT;
ALTER TABLE users ADD COLUMN actor_url TEXT;
ALTER TABLE users ADD COLUMN is_local BOOLEAN DEFAULT 1;
ALTER TABLE users ADD COLUMN last_refreshed_at TEXT;

-- Create index on actor_url for federation lookups
CREATE INDEX IF NOT EXISTS idx_users_actor_url ON users(actor_url);

-- Track follow relationships across instances
CREATE TABLE IF NOT EXISTS actor_follows (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    follower_actor_url TEXT NOT NULL,
    following_actor_url TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(follower_actor_url, following_actor_url)
);

-- Create indexes for follow lookups
CREATE INDEX IF NOT EXISTS idx_actor_follows_follower ON actor_follows(follower_actor_url);
CREATE INDEX IF NOT EXISTS idx_actor_follows_following ON actor_follows(following_actor_url);

-- Federated exercise objects
CREATE TABLE IF NOT EXISTS exercises (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    activity_id TEXT REFERENCES activities(id) ON DELETE SET NULL,
    actor_url TEXT NOT NULL,
    exercise_url TEXT NOT NULL UNIQUE,
    activity_type TEXT NOT NULL,
    started_at TEXT,
    name TEXT,
    content TEXT,
    route_url TEXT,
    stats_url TEXT,
    published_at TEXT NOT NULL,
    is_local BOOLEAN DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now'))
);

-- Index for looking up exercises by actor
CREATE INDEX IF NOT EXISTS idx_exercises_actor ON exercises(actor_url);
-- Index for looking up exercises by user
CREATE INDEX IF NOT EXISTS idx_exercises_user ON exercises(user_id);
