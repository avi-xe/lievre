use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub id: String,
    pub follower_id: String,
    pub following_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Like {
    pub id: String,
    pub activity_id: String,
    pub user_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub activity_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct SocialRepository {
    pool: SqlitePool,
}

impl SocialRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // Follow operations

    pub async fn follow(&self, follower_id: &str, following_id: &str) -> Result<Follow, anyhow::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO follows (id, follower_id, following_id, status, created_at)
             VALUES (?, ?, ?, 'accepted', ?)"
        )
        .bind(&id)
        .bind(follower_id)
        .bind(following_id)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let follow = sqlx::query_as::<_, Follow>("SELECT * FROM follows WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(follow)
    }

    pub async fn unfollow(&self, follower_id: &str, following_id: &str) -> Result<bool, anyhow::Error> {
        let result = sqlx::query(
            "DELETE FROM follows WHERE follower_id = ? AND following_id = ?"
        )
        .bind(follower_id)
        .bind(following_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn is_following(&self, follower_id: &str, following_id: &str) -> Result<bool, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM follows WHERE follower_id = ? AND following_id = ? AND status = 'accepted'"
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0 > 0)
    }

    pub async fn get_followers(&self, user_id: &str) -> Result<Vec<Follow>, anyhow::Error> {
        let follows = sqlx::query_as::<_, Follow>(
            "SELECT * FROM follows WHERE following_id = ? AND status = 'accepted' ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(follows)
    }

    pub async fn get_following(&self, user_id: &str) -> Result<Vec<Follow>, anyhow::Error> {
        let follows = sqlx::query_as::<_, Follow>(
            "SELECT * FROM follows WHERE follower_id = ? AND status = 'accepted' ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(follows)
    }

    pub async fn get_follower_count(&self, user_id: &str) -> Result<i64, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM follows WHERE following_id = ? AND status = 'accepted'"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    pub async fn get_following_count(&self, user_id: &str) -> Result<i64, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM follows WHERE follower_id = ? AND status = 'accepted'"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    // Like operations

    pub async fn like(&self, activity_id: &str, user_id: &str) -> Result<Like, anyhow::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO likes (id, activity_id, user_id, created_at)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(activity_id)
        .bind(user_id)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let like = sqlx::query_as::<_, Like>("SELECT * FROM likes WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(like)
    }

    pub async fn unlike(&self, activity_id: &str, user_id: &str) -> Result<bool, anyhow::Error> {
        let result = sqlx::query(
            "DELETE FROM likes WHERE activity_id = ? AND user_id = ?"
        )
        .bind(activity_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn has_liked(&self, activity_id: &str, user_id: &str) -> Result<bool, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM likes WHERE activity_id = ? AND user_id = ?"
        )
        .bind(activity_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0 > 0)
    }

    pub async fn get_like_count(&self, activity_id: &str) -> Result<i64, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM likes WHERE activity_id = ?"
        )
        .bind(activity_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    pub async fn get_likes(&self, activity_id: &str) -> Result<Vec<Like>, anyhow::Error> {
        let likes = sqlx::query_as::<_, Like>(
            "SELECT * FROM likes WHERE activity_id = ? ORDER BY created_at DESC"
        )
        .bind(activity_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(likes)
    }

    // Comment operations

    pub async fn add_comment(&self, activity_id: &str, user_id: &str, content: &str) -> Result<Comment, anyhow::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO comments (id, activity_id, user_id, content, created_at)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(activity_id)
        .bind(user_id)
        .bind(content)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let comment = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(comment)
    }

    pub async fn delete_comment(&self, comment_id: &str, user_id: &str) -> Result<bool, anyhow::Error> {
        let result = sqlx::query(
            "DELETE FROM comments WHERE id = ? AND user_id = ?"
        )
        .bind(comment_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_comments(&self, activity_id: &str) -> Result<Vec<Comment>, anyhow::Error> {
        let comments = sqlx::query_as::<_, Comment>(
            "SELECT * FROM comments WHERE activity_id = ? ORDER BY created_at ASC"
        )
        .bind(activity_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }

    pub async fn get_comment_count(&self, activity_id: &str) -> Result<i64, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM comments WHERE activity_id = ?"
        )
        .bind(activity_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    // Feed operations

    pub async fn get_feed(&self, user_id: &str, limit: i64, offset: i64) -> Result<Vec<crate::activity::Activity>, anyhow::Error> {
        let activities = sqlx::query_as::<_, crate::activity::Activity>(
            "SELECT a.* FROM activities a
             INNER JOIN follows f ON a.user_id = f.following_id
             WHERE f.follower_id = ? AND f.status = 'accepted'
             AND a.visibility IN ('public', 'followers')
             ORDER BY a.started_at DESC
             LIMIT ? OFFSET ?"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(activities)
    }

    pub async fn get_public_feed(&self, limit: i64, offset: i64) -> Result<Vec<crate::activity::Activity>, anyhow::Error> {
        let activities = sqlx::query_as::<_, crate::activity::Activity>(
            "SELECT * FROM activities
             WHERE visibility = 'public'
             ORDER BY started_at DESC
             LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(activities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                display_name TEXT,
                avatar_url TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS activities (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id),
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
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS follows (
                id TEXT PRIMARY KEY,
                follower_id TEXT NOT NULL REFERENCES users(id),
                following_id TEXT NOT NULL REFERENCES users(id),
                status TEXT NOT NULL DEFAULT 'accepted',
                created_at TEXT DEFAULT (datetime('now')),
                UNIQUE(follower_id, following_id)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS likes (
                id TEXT PRIMARY KEY,
                activity_id TEXT NOT NULL REFERENCES activities(id),
                user_id TEXT NOT NULL REFERENCES users(id),
                created_at TEXT DEFAULT (datetime('now')),
                UNIQUE(activity_id, user_id)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                activity_id TEXT NOT NULL REFERENCES activities(id),
                user_id TEXT NOT NULL REFERENCES users(id),
                content TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now'))
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create test users
        sqlx::query("INSERT INTO users (id, email, username, password_hash) VALUES ('user1', 'a@a.com', 'user1', 'hash')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO users (id, email, username, password_hash) VALUES ('user2', 'b@b.com', 'user2', 'hash')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO users (id, email, username, password_hash) VALUES ('user3', 'c@c.com', 'user3', 'hash')")
            .execute(&pool).await.unwrap();

        // Create test activity
        sqlx::query("INSERT INTO activities (id, user_id, activity_type, started_at) VALUES ('act1', 'user2', 'ride', '2024-01-15T08:00:00Z')")
            .execute(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_follow() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        let follow = repo.follow("user1", "user2").await.unwrap();
        assert_eq!(follow.follower_id, "user1");
        assert_eq!(follow.following_id, "user2");
        assert!(repo.is_following("user1", "user2").await.unwrap());
    }

    #[tokio::test]
    async fn test_unfollow() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        repo.follow("user1", "user2").await.unwrap();
        let result = repo.unfollow("user1", "user2").await.unwrap();
        assert!(result);
        assert!(!repo.is_following("user1", "user2").await.unwrap());
    }

    #[tokio::test]
    async fn test_followers_count() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        repo.follow("user1", "user2").await.unwrap();
        repo.follow("user3", "user2").await.unwrap();

        assert_eq!(repo.get_follower_count("user2").await.unwrap(), 2);
        assert_eq!(repo.get_following_count("user1").await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_like() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        let like = repo.like("act1", "user1").await.unwrap();
        assert_eq!(like.activity_id, "act1");
        assert!(repo.has_liked("act1", "user1").await.unwrap());
        assert_eq!(repo.get_like_count("act1").await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_unlike() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        repo.like("act1", "user1").await.unwrap();
        let result = repo.unlike("act1", "user1").await.unwrap();
        assert!(result);
        assert!(!repo.has_liked("act1", "user1").await.unwrap());
    }

    #[tokio::test]
    async fn test_comment() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        let comment = repo.add_comment("act1", "user1", "Great ride!").await.unwrap();
        assert_eq!(comment.content, "Great ride!");
        assert_eq!(repo.get_comment_count("act1").await.unwrap(), 1);

        let comments = repo.get_comments("act1").await.unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_comment() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        let comment = repo.add_comment("act1", "user1", "Nice!").await.unwrap();
        let result = repo.delete_comment(&comment.id, "user1").await.unwrap();
        assert!(result);
        assert_eq!(repo.get_comment_count("act1").await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_feed() {
        let pool = setup_db().await;
        let repo = SocialRepository::new(pool);

        repo.follow("user1", "user2").await.unwrap();

        let feed = repo.get_feed("user1", 10, 0).await.unwrap();
        assert_eq!(feed.len(), 1);
        assert_eq!(feed[0].id, "act1");
    }
}
