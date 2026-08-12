use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub actor_id: String,
    pub r#type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub content: Option<String>,
    pub read: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct NotificationRepository {
    pool: SqlitePool,
}

impl NotificationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new notification
    pub async fn create(
        &self,
        user_id: &str,
        actor_id: &str,
        r#type: &str,
        entity_type: &str,
        entity_id: &str,
        content: Option<&str>,
    ) -> Result<Notification, anyhow::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO notifications (id, user_id, actor_id, type, entity_type, entity_id, content, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(user_id)
        .bind(actor_id)
        .bind(r#type)
        .bind(entity_type)
        .bind(entity_id)
        .bind(content)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let notification = sqlx::query_as::<_, Notification>("SELECT * FROM notifications WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(notification)
    }

    /// List notifications for a user (newest first)
    pub async fn list(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, anyhow::Error> {
        let notifications = sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(notifications)
    }

    /// Mark a single notification as read
    pub async fn mark_read(
        &self,
        notification_id: &str,
        user_id: &str,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query(
            "UPDATE notifications SET read = 1 WHERE id = ? AND user_id = ?",
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Mark all notifications as read for a user
    pub async fn mark_all_read(
        &self,
        user_id: &str,
    ) -> Result<i64, anyhow::Error> {
        let result = sqlx::query(
            "UPDATE notifications SET read = 1 WHERE user_id = ? AND read = 0",
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    /// Count unread notifications
    pub async fn unread_count(
        &self,
        user_id: &str,
    ) -> Result<i64, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM notifications WHERE user_id = ? AND read = 0",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Delete a notification
    pub async fn delete(
        &self,
        notification_id: &str,
        user_id: &str,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query(
            "DELETE FROM notifications WHERE id = ? AND user_id = ?",
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
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

        sqlx::query("CREATE TABLE users (id TEXT PRIMARY KEY, email TEXT UNIQUE, username TEXT UNIQUE, password_hash TEXT, display_name TEXT, avatar_url TEXT, created_at TEXT, updated_at TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE notifications (id TEXT PRIMARY KEY, user_id TEXT NOT NULL, actor_id TEXT NOT NULL, type TEXT NOT NULL, entity_type TEXT NOT NULL, entity_id TEXT NOT NULL, content TEXT, read INTEGER NOT NULL DEFAULT 0, created_at TEXT NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        // Insert test users
        sqlx::query("INSERT INTO users (id, email, username, password_hash) VALUES ('user1', 'u1@test.com', 'user1', 'hash')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO users (id, email, username, password_hash) VALUES ('user2', 'u2@test.com', 'user2', 'hash')")
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_notification() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        let n = repo
            .create("user1", "user2", "follow", "user", "user2", None)
            .await
            .unwrap();

        assert_eq!(n.user_id, "user1");
        assert_eq!(n.actor_id, "user2");
        assert_eq!(n.r#type, "follow");
        assert!(!n.read);
    }

    #[tokio::test]
    async fn test_list_notifications() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        repo.create("user1", "user2", "follow", "user", "user2", None).await.unwrap();
        repo.create("user1", "user2", "like", "activity", "act1", Some("Nice ride!")).await.unwrap();

        let notifications = repo.list("user1", 10, 0).await.unwrap();
        assert_eq!(notifications.len(), 2);
        // Newest first
        assert_eq!(notifications[0].r#type, "like");
        assert_eq!(notifications[1].r#type, "follow");
    }

    #[tokio::test]
    async fn test_mark_read() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        let n = repo.create("user1", "user2", "follow", "user", "user2", None).await.unwrap();
        assert!(!n.read);

        let marked = repo.mark_read(&n.id, "user1").await.unwrap();
        assert!(marked);

        let notifications = repo.list("user1", 10, 0).await.unwrap();
        assert!(notifications[0].read);
    }

    #[tokio::test]
    async fn test_mark_all_read() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        repo.create("user1", "user2", "follow", "user", "user2", None).await.unwrap();
        repo.create("user1", "user2", "like", "activity", "act1", None).await.unwrap();

        let count = repo.mark_all_read("user1").await.unwrap();
        assert_eq!(count, 2);

        let unread = repo.unread_count("user1").await.unwrap();
        assert_eq!(unread, 0);
    }

    #[tokio::test]
    async fn test_unread_count() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        repo.create("user1", "user2", "follow", "user", "user2", None).await.unwrap();
        repo.create("user1", "user2", "like", "activity", "act1", None).await.unwrap();

        let unread = repo.unread_count("user1").await.unwrap();
        assert_eq!(unread, 2);

        let notifications = repo.list("user1", 10, 0).await.unwrap();
        repo.mark_read(&notifications[0].id, "user1").await.unwrap();

        let unread = repo.unread_count("user1").await.unwrap();
        assert_eq!(unread, 1);
    }

    #[tokio::test]
    async fn test_delete_notification() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        let n = repo.create("user1", "user2", "follow", "user", "user2", None).await.unwrap();

        let deleted = repo.delete(&n.id, "user1").await.unwrap();
        assert!(deleted);

        let notifications = repo.list("user1", 10, 0).await.unwrap();
        assert_eq!(notifications.len(), 0);
    }

    #[tokio::test]
    async fn test_cannot_mark_others_read() {
        let pool = setup_db().await;
        let repo = NotificationRepository::new(pool);

        let n = repo.create("user1", "user2", "follow", "user", "user2", None).await.unwrap();

        // user2 trying to mark user1's notification
        let marked = repo.mark_read(&n.id, "user2").await.unwrap();
        assert!(!marked);
    }
}
