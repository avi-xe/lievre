use argon2::{password_hash::SaltString, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::user::{LoginUser, User, UserRepository};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    /// Get a reference to the user repository
    pub fn user_repo(&self) -> &UserRepository {
        &self.user_repo
    }

    pub async fn register(&self, user: LoginUser, password: &str) -> anyhow::Result<User> {
        // Check if user exists
        if self.user_repo.find_by_email(&user.email).await?.is_some() {
            anyhow::bail!("User with this email already exists");
        }

        // Hash password
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = argon2::Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?
            .to_string();

        // Create user
        let username = user
            .username
            .unwrap_or_else(|| user.email.split('@').next().unwrap_or("user").to_string());
        let create_user = crate::user::CreateUser {
            email: user.email,
            username,
            password: password.to_string(),
            display_name: None,
        };

        let user = self.user_repo.create(create_user, &password_hash).await?;
        Ok(user)
    }

    pub async fn login(&self, credentials: LoginUser, password: &str) -> anyhow::Result<String> {
        // Find user by email
        let user = self
            .user_repo
            .find_by_email(&credentials.email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        // Verify password
        let parsed_hash = argon2::PasswordHash::new(&user.password_hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow::anyhow!("Invalid credentials"))?;

        // Generate JWT
        let claims = Claims {
            sub: user.id,
            exp: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub async fn verify_token(&self, token: &str) -> anyhow::Result<User> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        let user = self
            .user_repo
            .find_by_id(&token_data.claims.sub)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_service() -> AuthService {
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

        let user_repo = UserRepository::new(pool);
        AuthService::new(user_repo, "test-secret".to_string())
    }

    #[tokio::test]
    async fn test_register_user() {
        let service = setup_service().await;

        let user = LoginUser {
            email: "test@example.com".to_string(),
            username: Some("testuser".to_string()),
            password: "password123".to_string(),
        };

        let created = service.register(user, "password123").await.unwrap();
        assert_eq!(created.email, "test@example.com");
        assert_eq!(created.username, "testuser");
    }

    #[tokio::test]
    async fn test_login_user() {
        let service = setup_service().await;

        let user = LoginUser {
            email: "test@example.com".to_string(),
            username: Some("testuser".to_string()),
            password: "password123".to_string(),
        };

        service.register(user.clone(), "password123").await.unwrap();

        let token = service.login(user, "password123").await.unwrap();
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_verify_token() {
        let service = setup_service().await;

        let user = LoginUser {
            email: "test@example.com".to_string(),
            username: Some("testuser".to_string()),
            password: "password123".to_string(),
        };

        let created = service.register(user.clone(), "password123").await.unwrap();
        let token = service.login(user, "password123").await.unwrap();

        let verified = service.verify_token(&token).await.unwrap();
        assert_eq!(verified.id, created.id);
    }
}
