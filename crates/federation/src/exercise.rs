use serde::{Deserialize, Serialize};
use url::Url;

/// fedisport Exercise object
///
/// See: https://github.com/fedisport/vocabulary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exercise {
    #[serde(rename = "type")]
    pub kind: String, // "Exercise"
    pub id: Url,
    pub attributed_to: Url,
    pub activity_type: String,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub route_url: Option<Url>,
    #[serde(default)]
    pub stats_url: Option<Url>,
    pub published: String,
    #[serde(default)]
    pub to: Vec<serde_json::Value>,
    #[serde(default)]
    pub cc: Vec<serde_json::Value>,
}

/// Exercise stats (served at statsUrl)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseStats {
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub elevation_gain: Option<f64>,
    #[serde(default)]
    pub device: Option<String>,
    #[serde(default)]
    pub avg_pace: Option<i64>,
    #[serde(default)]
    pub avg_heart_rate: Option<i32>,
    #[serde(default)]
    pub max_heart_rate: Option<i32>,
    #[serde(default)]
    pub avg_power: Option<f64>,
    #[serde(default)]
    pub max_power: Option<f64>,
    #[serde(default)]
    pub normalized_power: Option<f64>,
    #[serde(default)]
    pub avg_cadence: Option<f64>,
}

/// WebFinger response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebfingerResponse {
    pub subject: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub links: Vec<WebfingerLink>,
}

/// WebFinger link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebfingerLink {
    pub rel: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub href: String,
}

/// Map internal activity type string to fedisport activityType value
pub fn map_activity_type(activity_type: &str) -> &str {
    match activity_type {
        "ride" => "ride",
        "run" => "run",
        "swim" => "swim",
        "walk" => "walk",
        "hike" => "hike",
        "virtual_ride" => "virtualRide",
        _ => "workout",
    }
}

/// Map fedisport activityType value back to internal activity type string
pub fn map_activity_type_reverse(activity_type: &str) -> &str {
    match activity_type {
        "ride" => "ride",
        "run" => "run",
        "swim" => "swim",
        "walk" => "walk",
        "hike" => "hike",
        "virtualRide" => "virtual_ride",
        _ => "ride",
    }
}

/// Serialize an activity + optional route into a fedisport Exercise JSON-LD object.
///
/// The wire object carries social data (who, what, when, title), while fitness
/// metrics are served from `statsUrl` and route from `routeUrl` — URLs the
/// originating server controls.
pub fn exercise_to_jsonld(
    activity: &lievre_core::Activity,
    has_route: bool,
    base_url: &str,
    username: &str,
) -> serde_json::Value {
    let exercise_url = format!("{}/exercises/{}", base_url, activity.id);
    let actor_url = format!("{}/users/{}", base_url, username);
    let stats_url = format!("{}/api/exercises/{}/stats", base_url, activity.id);

    let route_url = if has_route {
        Some(format!("{}/api/exercises/{}/route", base_url, activity.id))
    } else {
        None
    };

    let mut to = vec![serde_json::json!(
        "https://www.w3.org/ns/activitystreams#Public"
    )];
    let mut cc = vec![serde_json::json!(format!("{}/followers", actor_url))];

    // For non-public, adjust visibility
    match activity.visibility {
        lievre_core::Visibility::Public => {}
        lievre_core::Visibility::Followers => {
            // Only followers should see it, not the general public
            to = vec![];
            cc = vec![serde_json::json!(format!("{}/followers", actor_url))];
        }
        lievre_core::Visibility::Private => {
            // Only the owner
            to = vec![serde_json::json!(actor_url.clone())];
            cc = vec![];
        }
    }

    let mut obj = serde_json::json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://fedisport.github.io/vocabulary/context.jsonld"
        ],
        "type": "Exercise",
        "id": exercise_url,
        "attributedTo": actor_url,
        "activityType": map_activity_type(&activity.activity_type.to_string()),
        "startedAt": activity.started_at.to_rfc3339(),
        "name": activity.title,
        "content": activity.description,
        "statsUrl": stats_url,
        "published": activity.created_at.to_rfc3339(),
        "to": to,
        "cc": cc,
    });

    if let Some(route_url) = route_url {
        obj.as_object_mut()
            .unwrap()
            .insert("routeUrl".to_string(), serde_json::json!(route_url));
    }

    obj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_activity_type() {
        assert_eq!(map_activity_type("ride"), "ride");
        assert_eq!(map_activity_type("run"), "run");
        assert_eq!(map_activity_type("swim"), "swim");
        assert_eq!(map_activity_type("walk"), "walk");
        assert_eq!(map_activity_type("hike"), "hike");
        assert_eq!(map_activity_type("virtual_ride"), "virtualRide");
        assert_eq!(map_activity_type("unknown"), "workout");
    }

    #[test]
    fn test_map_activity_type_reverse() {
        assert_eq!(map_activity_type_reverse("ride"), "ride");
        assert_eq!(map_activity_type_reverse("run"), "run");
        assert_eq!(map_activity_type_reverse("swim"), "swim");
        assert_eq!(map_activity_type_reverse("walk"), "walk");
        assert_eq!(map_activity_type_reverse("hike"), "hike");
        assert_eq!(map_activity_type_reverse("virtualRide"), "virtual_ride");
        assert_eq!(map_activity_type_reverse("unknown"), "ride");
    }

    #[test]
    fn test_exercise_to_jsonld_public_activity_with_route() {
        let activity = lievre_core::Activity {
            id: "act-123".to_string(),
            user_id: "user-1".to_string(),
            activity_type: lievre_core::ActivityType::Ride,
            title: Some("Morning Ride".to_string()),
            description: Some("A nice ride".to_string()),
            started_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T08:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            duration_seconds: Some(3600),
            distance_meters: Some(50000.0),
            elevation_gain_meters: Some(500.0),
            visibility: lievre_core::Visibility::Public,
            created_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T09:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T09:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        };

        let json = exercise_to_jsonld(&activity, true, "https://lievre.example", "alice");

        assert_eq!(json["type"], "Exercise");
        assert_eq!(json["id"], "https://lievre.example/exercises/act-123");
        assert_eq!(json["attributedTo"], "https://lievre.example/users/alice");
        assert_eq!(json["activityType"], "ride");
        assert_eq!(json["name"], "Morning Ride");
        assert_eq!(json["content"], "A nice ride");
        assert_eq!(
            json["statsUrl"],
            "https://lievre.example/api/exercises/act-123/stats"
        );
        assert_eq!(
            json["routeUrl"],
            "https://lievre.example/api/exercises/act-123/route"
        );
        // Public: to includes AS Public
        let to = json["to"].as_array().unwrap();
        assert!(to.contains(&serde_json::json!(
            "https://www.w3.org/ns/activitystreams#Public"
        )));
        // cc includes followers collection
        let cc = json["cc"].as_array().unwrap();
        assert!(cc.contains(&serde_json::json!(
            "https://lievre.example/users/alice/followers"
        )));
    }

    #[test]
    fn test_exercise_to_jsonld_no_route() {
        let activity = lievre_core::Activity {
            id: "act-456".to_string(),
            user_id: "user-1".to_string(),
            activity_type: lievre_core::ActivityType::Run,
            title: Some("Evening Run".to_string()),
            description: None,
            started_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T18:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            duration_seconds: Some(1800),
            distance_meters: Some(10000.0),
            elevation_gain_meters: None,
            visibility: lievre_core::Visibility::Public,
            created_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T19:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T19:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        };

        let json = exercise_to_jsonld(&activity, false, "https://lievre.example", "alice");

        assert_eq!(json["type"], "Exercise");
        assert!(!json.as_object().unwrap().contains_key("routeUrl"));
        assert!(json["routeUrl"].is_null() || !json.as_object().unwrap().contains_key("routeUrl"));
    }

    #[test]
    fn test_exercise_to_jsonld_followers_visibility() {
        let activity = lievre_core::Activity {
            id: "act-789".to_string(),
            user_id: "user-1".to_string(),
            activity_type: lievre_core::ActivityType::Swim,
            title: Some("Pool Session".to_string()),
            description: None,
            started_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            duration_seconds: Some(2700),
            distance_meters: Some(3000.0),
            elevation_gain_meters: None,
            visibility: lievre_core::Visibility::Followers,
            created_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T11:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T11:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        };

        let json = exercise_to_jsonld(&activity, false, "https://lievre.example", "alice");

        // Followers: no AS Public in to
        let to = json["to"].as_array().unwrap();
        assert!(!to.contains(&serde_json::json!(
            "https://www.w3.org/ns/activitystreams#Public"
        )));
        // cc includes followers collection
        let cc = json["cc"].as_array().unwrap();
        assert!(cc.contains(&serde_json::json!(
            "https://lievre.example/users/alice/followers"
        )));
    }

    #[test]
    fn test_exercise_to_jsonld_private_visibility() {
        let activity = lievre_core::Activity {
            id: "act-private".to_string(),
            user_id: "user-1".to_string(),
            activity_type: lievre_core::ActivityType::Walk,
            title: Some("Secret Walk".to_string()),
            description: None,
            started_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            duration_seconds: None,
            distance_meters: None,
            elevation_gain_meters: None,
            visibility: lievre_core::Visibility::Private,
            created_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T13:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-15T13:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        };

        let json = exercise_to_jsonld(&activity, false, "https://lievre.example", "alice");

        // Private: to only includes the owner
        let to = json["to"].as_array().unwrap();
        assert_eq!(to.len(), 1);
        assert_eq!(to[0], "https://lievre.example/users/alice");
        // cc is empty
        let cc = json["cc"].as_array().unwrap();
        assert!(cc.is_empty());
    }
}
