-- Create activities table
CREATE TABLE IF NOT EXISTS activities (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity_type TEXT NOT NULL,
    title TEXT,
    description TEXT,
    started_at TEXT NOT NULL,
    duration_seconds INTEGER,
    distance_meters REAL,
    elevation_gain_meters REAL,
    visibility TEXT DEFAULT 'followers',
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Create index on user_id for fast lookups
CREATE INDEX IF NOT EXISTS idx_activities_user_id ON activities(user_id);

-- Create index on started_at for sorting
CREATE INDEX IF NOT EXISTS idx_activities_started_at ON activities(started_at DESC);

-- Create index on visibility for filtering
CREATE INDEX IF NOT EXISTS idx_activities_visibility ON activities(visibility);
