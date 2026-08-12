use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub id: String,
    pub activity_id: String,
    pub coordinates: String, // JSON array of [lon, lat] or [lon, lat, ele]
    pub elevation_data: Option<String>, // JSON array of elevation values
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoute {
    pub activity_id: String,
    pub coordinates: Vec<Vec<f64>>, // [[lon, lat], [lon, lat, ele], ...]
    pub elevation_data: Option<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct RouteRepository {
    pool: SqlitePool,
}

impl RouteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, route: CreateRoute) -> anyhow::Result<Route> {
        let id = uuid::Uuid::new_v4().to_string();
        let coordinates = serde_json::to_string(&route.coordinates)?;
        let elevation_data = route
            .elevation_data
            .map(|e| serde_json::to_string(&e))
            .transpose()?;

        let route = sqlx::query_as::<_, Route>(
            r#"INSERT INTO routes (id, activity_id, coordinates, elevation_data)
               VALUES (?, ?, ?, ?)
               RETURNING *"#,
        )
        .bind(&id)
        .bind(&route.activity_id)
        .bind(&coordinates)
        .bind(&elevation_data)
        .fetch_one(&self.pool)
        .await?;

        Ok(route)
    }

    pub async fn find_by_activity_id(&self, activity_id: &str) -> anyhow::Result<Option<Route>> {
        let route = sqlx::query_as::<_, Route>("SELECT * FROM routes WHERE activity_id = ?")
            .bind(activity_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(route)
    }

    pub async fn delete_by_activity_id(&self, activity_id: &str) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM routes WHERE activity_id = ?")
            .bind(activity_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn to_geojson(&self, route: &Route) -> anyhow::Result<serde_json::Value> {
        let coordinates: Vec<Vec<f64>> = serde_json::from_str(&route.coordinates)?;

        let geojson = serde_json::json!({
            "type": "FeatureCollection",
            "features": [{
                "type": "Feature",
                "geometry": {
                    "type": "LineString",
                    "coordinates": coordinates
                },
                "properties": {}
            }]
        });

        Ok(geojson)
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
            )"#,
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
            )"#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS routes (
                id TEXT PRIMARY KEY,
                activity_id TEXT NOT NULL REFERENCES activities(id),
                coordinates TEXT NOT NULL,
                elevation_data TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            )"#,
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

        // Create test activity
        sqlx::query(
            r#"INSERT INTO activities (id, user_id, activity_type, started_at) VALUES ('activity-1', 'user-1', 'ride', datetime('now'))"#
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_route() {
        let pool = setup_db().await;
        let repo = RouteRepository::new(pool);

        let route = CreateRoute {
            activity_id: "activity-1".to_string(),
            coordinates: vec![
                vec![13.404954, 52.520008],
                vec![13.405101, 52.520212],
                vec![13.405200, 52.520300],
            ],
            elevation_data: Some(vec![34.0, 35.2, 36.0]),
        };

        let created = repo.create(route).await.unwrap();

        assert_eq!(created.activity_id, "activity-1");
        assert!(!created.coordinates.is_empty());
    }

    #[tokio::test]
    async fn test_find_route_by_activity_id() {
        let pool = setup_db().await;
        let repo = RouteRepository::new(pool);

        let route = CreateRoute {
            activity_id: "activity-1".to_string(),
            coordinates: vec![vec![13.404954, 52.520008], vec![13.405101, 52.520212]],
            elevation_data: None,
        };

        let created = repo.create(route).await.unwrap();
        let found = repo.find_by_activity_id("activity-1").await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn test_to_geojson() {
        let pool = setup_db().await;
        let repo = RouteRepository::new(pool);

        let route = CreateRoute {
            activity_id: "activity-1".to_string(),
            coordinates: vec![vec![13.404954, 52.520008], vec![13.405101, 52.520212]],
            elevation_data: None,
        };

        let created = repo.create(route).await.unwrap();
        let geojson = repo.to_geojson(&created).await.unwrap();

        assert_eq!(geojson["type"], "FeatureCollection");
        assert!(geojson["features"].is_array());
        let feature = &geojson["features"][0];
        assert_eq!(feature["type"], "Feature");
        assert_eq!(feature["geometry"]["type"], "LineString");
        assert!(feature["geometry"]["coordinates"].is_array());
    }

    #[tokio::test]
    async fn test_delete_route() {
        let pool = setup_db().await;
        let repo = RouteRepository::new(pool);

        let route = CreateRoute {
            activity_id: "activity-1".to_string(),
            coordinates: vec![vec![13.404954, 52.520008]],
            elevation_data: None,
        };

        repo.create(route).await.unwrap();
        let deleted = repo.delete_by_activity_id("activity-1").await.unwrap();

        assert!(deleted);

        let found = repo.find_by_activity_id("activity-1").await.unwrap();
        assert!(found.is_none());
    }
}
