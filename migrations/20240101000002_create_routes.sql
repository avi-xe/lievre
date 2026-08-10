-- Create routes table
CREATE TABLE IF NOT EXISTS routes (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    coordinates TEXT NOT NULL,  -- JSON array of [lon, lat] or [lon, lat, ele]
    elevation_data TEXT,        -- JSON array of elevation values
    created_at TEXT DEFAULT (datetime('now'))
);

-- Create index on activity_id for fast lookups
CREATE INDEX IF NOT EXISTS idx_routes_activity_id ON routes(activity_id);
