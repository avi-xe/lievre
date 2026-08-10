-- Create jobs table for background processing
CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    job_type TEXT NOT NULL,
    payload TEXT NOT NULL,          -- JSON
    status TEXT NOT NULL DEFAULT 'pending',  -- pending, processing, completed, failed
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    priority INTEGER NOT NULL DEFAULT 0,  -- higher = more urgent
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    started_at TEXT,
    completed_at TEXT,
    next_retry_at TEXT,             -- for exponential backoff
    error TEXT                      -- last error message
);

-- Index for efficient job polling
CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status, priority DESC, created_at);
CREATE INDEX IF NOT EXISTS idx_jobs_retry ON jobs(next_retry_at) WHERE status = 'pending';
