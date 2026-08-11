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
