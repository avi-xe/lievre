use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct UpdateActivity {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub visibility: Option<Visibility>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_type_display() {
        assert_eq!(ActivityType::Ride.to_string(), "ride");
        assert_eq!(ActivityType::Run.to_string(), "run");
        assert_eq!(ActivityType::Swim.to_string(), "swim");
        assert_eq!(ActivityType::Walk.to_string(), "walk");
        assert_eq!(ActivityType::Hike.to_string(), "hike");
        assert_eq!(ActivityType::VirtualRide.to_string(), "virtual_ride");
    }

    #[test]
    fn test_activity_type_serialization() {
        let ride = ActivityType::Ride;
        let serialized = serde_json::to_string(&ride).unwrap();
        assert_eq!(serialized, "\"ride\"");

        let deserialized: ActivityType = serde_json::from_str("\"ride\"").unwrap();
        assert_eq!(deserialized, ActivityType::Ride);
    }

    #[test]
    fn test_visibility_serialization() {
        let public = Visibility::Public;
        let serialized = serde_json::to_string(&public).unwrap();
        assert_eq!(serialized, "\"public\"");

        let deserialized: Visibility = serde_json::from_str("\"public\"").unwrap();
        assert_eq!(deserialized, Visibility::Public);
    }

    #[test]
    fn test_create_activity_defaults() {
        let create = CreateActivity {
            activity_type: ActivityType::Ride,
            title: Some("Morning Ride".to_string()),
            description: None,
            started_at: Utc::now(),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            visibility: None,
        };

        assert_eq!(create.activity_type, ActivityType::Ride);
        assert_eq!(create.title, Some("Morning Ride".to_string()));
        assert!(create.description.is_none());
        assert_eq!(create.duration_seconds, Some(3600));
        assert_eq!(create.distance_meters, Some(50000.0));
        assert_eq!(create.elevation_gain_meters, Some(500.0));
        assert!(create.visibility.is_none());
    }
}
