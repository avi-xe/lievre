-- Activity statistics computed from route data
CREATE TABLE IF NOT EXISTS activity_stats (
    activity_id TEXT PRIMARY KEY REFERENCES activities(id) ON DELETE CASCADE,
    total_distance_meters REAL,
    total_duration_seconds INTEGER,
    total_elevation_gain_meters REAL,
    total_elevation_loss_meters REAL,
    avg_speed_ms REAL,
    max_speed_ms REAL,
    avg_pace_min_km REAL,           -- for running (seconds per km)
    total_calories INTEGER,
    avg_heart_rate INTEGER,
    max_heart_rate INTEGER,
    avg_power_watts REAL,
    max_power_watts REAL,
    avg_cadence INTEGER,
    computed_at TEXT NOT NULL DEFAULT (datetime('now'))
);
