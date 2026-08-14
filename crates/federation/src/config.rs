use activitypub_federation::config::FederationConfig;
use sqlx::SqlitePool;
use url::Url;

/// Federation database connection
#[derive(Clone)]
pub struct FederationDb {
    pub pool: SqlitePool,
    pub domain: String,
    pub scheme: String,
}

impl FederationDb {
    pub fn new(pool: SqlitePool, domain: String, scheme: String) -> Self {
        Self {
            pool,
            domain,
            scheme,
        }
    }

    /// Get the base URL for this instance (e.g., https://example.com)
    pub fn base_url(&self) -> String {
        format!("{}://{}", self.scheme, self.domain)
    }

    /// Get actor URL for a user
    pub fn actor_url(&self, username: &str) -> Url {
        Url::parse(&format!("{}/users/{}", self.base_url(), username)).expect("Invalid actor URL")
    }

    /// Get inbox URL for a user
    pub fn inbox_url(&self, username: &str) -> Url {
        Url::parse(&format!("{}/users/{}/inbox", self.base_url(), username))
            .expect("Invalid inbox URL")
    }

    /// Get outbox URL for a user
    pub fn outbox_url(&self, username: &str) -> Url {
        Url::parse(&format!("{}/users/{}/outbox", self.base_url(), username))
            .expect("Invalid outbox URL")
    }

    /// Get exercise URL
    pub fn exercise_url(&self, exercise_id: &str) -> Url {
        Url::parse(&format!("{}/exercises/{}", self.base_url(), exercise_id))
            .expect("Invalid exercise URL")
    }

    /// Get route URL for an exercise
    pub fn route_url(&self, exercise_id: &str) -> Url {
        Url::parse(&format!(
            "{}/api/exercises/{}/route",
            self.base_url(),
            exercise_id
        ))
        .expect("Invalid route URL")
    }

    /// Get stats URL for an exercise
    pub fn stats_url(&self, exercise_id: &str) -> Url {
        Url::parse(&format!(
            "{}/api/exercises/{}/stats",
            self.base_url(),
            exercise_id
        ))
        .expect("Invalid stats URL")
    }

    /// Get like URL
    pub fn like_url(&self, like_id: &str) -> Url {
        Url::parse(&format!("{}/likes/{}", self.base_url(), like_id)).expect("Invalid like URL")
    }

    /// Get WebFinger URL
    pub fn webfinger_url(&self) -> Url {
        Url::parse(&format!("{}/.well-known/webfinger", self.base_url()))
            .expect("Invalid webfinger URL")
    }
}

/// Create federation config for an Axum server
pub async fn create_federation_config(
    db: FederationDb,
) -> anyhow::Result<FederationConfig<FederationDb>> {
    let config = FederationConfig::builder()
        .domain(db.domain.clone())
        .app_data(db)
        .build()
        .await?;
    Ok(config)
}
