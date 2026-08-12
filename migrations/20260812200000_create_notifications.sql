-- Notifications for social interactions (follow, like, comment)
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    actor_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type TEXT NOT NULL,              -- 'follow', 'like', 'comment'
    entity_type TEXT NOT NULL,       -- 'activity', 'user'
    entity_id TEXT NOT NULL,         -- activity_id or user_id
    content TEXT,                    -- optional preview text (e.g. comment body)
    read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications(user_id, read, created_at DESC);
