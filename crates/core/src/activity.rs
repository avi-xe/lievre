use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Ride,
    Run,
    Swim,
    Walk,
    Hike,
    VirtualRide,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityType::Ride => write!(f, "ride"),
            ActivityType::Run => write!(f, "run"),
            ActivityType::Swim => write!(f, "swim"),
            ActivityType::Walk => write!(f, "walk"),
            ActivityType::Hike => write!(f, "hike"),
            ActivityType::VirtualRide => write!(f, "virtual_ride"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Followers,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Activity {
    pub id: String,
    pub user_id: String,
    pub activity_type: ActivityType,
    pub title: Option<String>,
    pub description: Option<String>,
    pub started_at: DateTime<Utc>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub visibility: Visibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateActivity {
    pub activity_type: ActivityType,
    pub title: Option<String>,
    pub description: Option<String>,
    pub started_at: DateTime<Utc>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateActivity {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Clone)]
pub struct ActivityRepository {
    pool: SqlitePool,
}

impl ActivityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_id: &str, activity: CreateActivity) -> anyhow::Result<Activity> {
        let id = uuid::Uuid::new_v4().to_string();
        let visibility = activity.visibility.unwrap_or(Visibility::Followers);

        let activity = sqlx::query_as::<_, Activity>(
            r#"INSERT INTO activities (id, user_id, activity_type, title, description, started_at, duration_seconds, distance_meters, elevation_gain_meters, visibility)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING *"#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(&activity.activity_type)
        .bind(&activity.title)
        .bind(&activity.description)
        .bind(activity.started_at)
        .bind(activity.duration_seconds)
        .bind(activity.distance_meters)
        .bind(activity.elevation_gain_meters)
        .bind(&visibility)
        .fetch_one(&self.pool)
        .await?;

        Ok(activity)
    }

    pub async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<Activity>> {
        let activity = sqlx::query_as::<_, Activity>(
            "SELECT * FROM activities WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(activity)
    }

    pub async fn find_by_user_id(&self, user_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<Activity>> {
        let activities = sqlx::query_as::<_, Activity>(
            "SELECT * FROM activities WHERE user_id = ? ORDER BY started_at DESC LIMIT ? OFFSET ?"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(activities)
    }

    pub async fn update(&self, id: &str, activity: UpdateActivity) -> anyhow::Result<Option<Activity>> {
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let existing = existing.unwrap();
        
        let activity = sqlx::query_as::<_, Activity>(
            r#"UPDATE activities 
               SET title = ?, description = ?, duration_seconds = ?, distance_meters = ?, elevation_gain_meters = ?, visibility = ?, updated_at = datetime('now')
               WHERE id = ?
               RETURNING *"#,
        )
        .bind(activity.title.as_deref().unwrap_or_else(|| existing.title.as_deref().unwrap_or("")))
        .bind(activity.description.as_deref().unwrap_or_else(|| existing.description.as_deref().unwrap_or("")))
        .bind(activity.duration_seconds.or(existing.duration_seconds))
        .bind(activity.distance_meters.or(existing.distance_meters))
        .bind(activity.elevation_gain_meters.or(existing.elevation_gain_meters))
        .bind(activity.visibility.unwrap_or(existing.visibility))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(activity)
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM activities WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count_by_user_id(&self, user_id: &str) -> anyhow::Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM activities WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
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
                updated_at TEXT DEFAULT (datetime('now'))
            )"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS activities (
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
            )"#
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create test user
        sqlx::query(
            r#"INSERT INTO users (id, email, username, password_hash) VALUES ('user-1', 'test@example.com', 'testuser', 'hashed')"#
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_activity() {
        let pool = setup_db().await;
        let repo = ActivityRepository::new(pool);

        let activity = CreateActivity {
            activity_type: ActivityType::Ride,
            title: Some("Morning Ride".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            visibility: None,
        };

        let created = repo.create("user-1", activity).await.unwrap();

        assert_eq!(created.user_id, "user-1");
        assert_eq!(created.activity_type, ActivityType::Ride);
        assert_eq!(created.title, Some("Morning Ride".to_string()));
        assert_eq!(created.duration_seconds, Some(3600));
        assert_eq!(created.distance_meters, Some(50000.0));
    }

    #[tokio::test]
    async fn test_find_activity_by_id() {
        let pool = setup_db().await;
        let repo = ActivityRepository::new(pool);

        let activity = CreateActivity {
            activity_type: ActivityType::Run,
            title: Some("Evening Run".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: Some(1800),
            distance_meters: Some(10000.0),
            elevation_gain_meters: None,
            visibility: None,
        };

        let created = repo.create("user-1", activity).await.unwrap();
        let found = repo.find_by_id(&created.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn test_find_activities_by_user() {
        let pool = setup_db().await;
        let repo = ActivityRepository::new(pool);

        // Create two activities
        let activity1 = CreateActivity {
            activity_type: ActivityType::Ride,
            title: Some("Ride 1".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: None,
            visibility: None,
        };

        let activity2 = CreateActivity {
            activity_type: ActivityType::Run,
            title: Some("Run 1".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: Some(1800),
            distance_meters: Some(10000.0),
            elevation_gain_meters: None,
            visibility: None,
        };

        repo.create("user-1", activity1).await.unwrap();
        repo.create("user-1", activity2).await.unwrap();

        let activities = repo.find_by_user_id("user-1", 10, 0).await.unwrap();
        assert_eq!(activities.len(), 2);
    }

    #[tokio::test]
    async fn test_update_activity() {
        let pool = setup_db().await;
        let repo = ActivityRepository::new(pool);

        let activity = CreateActivity {
            activity_type: ActivityType::Ride,
            title: Some("Original Title".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: None,
            visibility: None,
        };

        let created = repo.create("user-1", activity).await.unwrap();

        let update = UpdateActivity {
            title: Some("Updated Title".to_string()),
            description: Some("Updated description".to_string()),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            visibility: Some(Visibility::Public),
        };

        let updated = repo.update(&created.id, update).await.unwrap();
        assert!(updated.is_some());

        let updated = updated.unwrap();
        assert_eq!(updated.title, Some("Updated Title".to_string()));
        assert_eq!(updated.description, Some("Updated description".to_string()));
        assert_eq!(updated.visibility, Visibility::Public);
    }

    #[tokio::test]
    async fn test_delete_activity() {
        let pool = setup_db().await;
        let repo = ActivityRepository::new(pool);

        let activity = CreateActivity {
            activity_type: ActivityType::Ride,
            title: Some("To Delete".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            visibility: None,
        };

        let created = repo.create("user-1", activity).await.unwrap();
        let deleted = repo.delete(&created.id).await.unwrap();

        assert!(deleted);

        let found = repo.find_by_id(&created.id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_count_activities() {
        let pool = setup_db().await;
        let repo = ActivityRepository::new(pool);

        let count_before = repo.count_by_user_id("user-1").await.unwrap();
        assert_eq!(count_before, 0);

        let activity = CreateActivity {
            activity_type: ActivityType::Ride,
            title: Some("Activity".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            visibility: None,
        };

        repo.create("user-1", activity).await.unwrap();

        let count_after = repo.count_by_user_id("user-1").await.unwrap();
        assert_eq!(count_after, 1);
    }
}
