use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Federation fields
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub inbox_url: Option<String>,
    pub outbox_url: Option<String>,
    pub actor_url: Option<String>,
    pub is_local: Option<bool>,
    pub last_refreshed_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: CreateUser, password_hash: &str) -> anyhow::Result<User> {
        let id = uuid::Uuid::new_v4().to_string();

        let user = sqlx::query_as::<_, User>(
            r#"INSERT INTO users (id, email, username, password_hash, display_name)
               VALUES (?, ?, ?, ?, ?)
               RETURNING *"#,
        )
        .bind(&id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(password_hash)
        .bind(&user.display_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    /// Update federation fields for a user
    pub async fn update_federation(
        &self,
        id: &str,
        public_key: &str,
        private_key: &str,
        actor_url: &str,
        inbox_url: &str,
        outbox_url: &str,
    ) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"UPDATE users
               SET public_key = ?, private_key = ?, actor_url = ?, inbox_url = ?, outbox_url = ?, is_local = 1
               WHERE id = ?
               RETURNING *"#,
        )
        .bind(public_key)
        .bind(private_key)
        .bind(actor_url)
        .bind(inbox_url)
        .bind(outbox_url)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_db() -> SqlitePool {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                display_name TEXT,
                avatar_url TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now')),
                public_key TEXT,
                private_key TEXT,
                inbox_url TEXT,
                outbox_url TEXT,
                actor_url TEXT,
                is_local BOOLEAN DEFAULT 1,
                last_refreshed_at TEXT
            )"#,
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = setup_db().await;
        let repo = UserRepository::new(pool);

        let user = CreateUser {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            display_name: Some("Test User".to_string()),
        };

        let created = repo.create(user.clone(), "hashed_password").await.unwrap();

        assert_eq!(created.email, user.email);
        assert_eq!(created.username, user.username);
        assert_eq!(created.password_hash, "hashed_password");
        assert_eq!(created.display_name, user.display_name);
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let pool = setup_db().await;
        let repo = UserRepository::new(pool);

        let user = CreateUser {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        repo.create(user.clone(), "hashed_password").await.unwrap();

        let found = repo.find_by_email(&user.email).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, user.email);
    }

    #[tokio::test]
    async fn test_find_by_username() {
        let pool = setup_db().await;
        let repo = UserRepository::new(pool);

        let user = CreateUser {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        repo.create(user.clone(), "hashed_password").await.unwrap();

        let found = repo.find_by_username(&user.username).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, user.username);
    }

    #[tokio::test]
    async fn test_user_response_conversion() {
        let user = User {
            id: "test-id".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: Some("Test User".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            public_key: None,
            private_key: None,
            inbox_url: None,
            outbox_url: None,
            actor_url: None,
            is_local: Some(true),
            last_refreshed_at: None,
        };

        let response = UserResponse::from(user.clone());

        assert_eq!(response.id, user.id);
        assert_eq!(response.email, user.email);
        assert_eq!(response.username, user.username);
        assert_eq!(response.display_name, user.display_name);
        assert_eq!(response.avatar_url, user.avatar_url);
    }
}
