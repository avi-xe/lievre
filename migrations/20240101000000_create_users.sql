-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Create index on email for fast lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Create index on username for fast lookups
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
