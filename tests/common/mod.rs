//! Shared test utilities for e2e tests
//!
//! Provides sandboxed test helpers that create unique data per test.

pub use reqwest::Client;
pub use serde_json::{json, Value};

pub const BASE_URL: &str = "http://localhost:80";

/// Unique suffix for test isolation (timestamp + random)
pub fn test_id() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    format!("{:x}", nanos)
}

/// Register a user and return (user_id, token, email, username)
pub async fn register_user(client: &Client, suffix: &str) -> (String, String, String, String) {
    let email = format!("{}@test.local", suffix);
    let username = suffix.to_string();
    let password = "testpass123";

    let resp = client
        .post(format!("{}/api/auth/register", BASE_URL))
        .json(&json!({
            "email": email,
            "username": username,
            "password": password
        }))
        .send()
        .await
        .expect("register request failed");

    assert!(
        resp.status().is_success(),
        "Registration failed: {}",
        resp.status()
    );
    let body: Value = resp.json().await.unwrap();
    let token = body["token"].as_str().unwrap().to_string();
    let user_id = body["user"]["id"].as_str().unwrap().to_string();

    (user_id, token, email, username)
}

/// Login and return token
#[allow(dead_code)]
pub async fn login_user(client: &Client, email: &str, password: &str) -> String {
    let resp = client
        .post(format!("{}/api/auth/login", BASE_URL))
        .json(&json!({ "email": email, "password": password }))
        .send()
        .await
        .expect("login request failed");

    assert!(
        resp.status().is_success(),
        "Login failed: {}",
        resp.status()
    );
    let body: Value = resp.json().await.unwrap();
    body["token"].as_str().unwrap().to_string()
}

/// Create an activity and return activity_id
pub async fn create_activity(
    client: &Client,
    token: &str,
    title: &str,
    visibility: &str,
) -> String {
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "activity_type": "ride",
            "title": title,
            "started_at": "2024-01-15T08:00:00Z",
            "duration_seconds": 3600,
            "distance_meters": 50000.0,
            "visibility": visibility
        }))
        .send()
        .await
        .expect("create activity failed");

    assert!(
        resp.status().is_success(),
        "Create activity failed: {}",
        resp.status()
    );
    let body: Value = resp.json().await.unwrap();
    body["id"].as_str().unwrap().to_string()
}

/// Delete an activity (cleanup helper)
pub async fn delete_activity(client: &Client, token: &str, activity_id: &str) {
    let _ = client
        .delete(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}

/// Unfollow a user (cleanup helper)
pub async fn unfollow(client: &Client, token: &str, target_id: &str) {
    let _ = client
        .delete(format!("{}/api/users/{}/follow", BASE_URL, target_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}

/// Remove a like (cleanup helper)
pub async fn unlike(client: &Client, token: &str, activity_id: &str) {
    let _ = client
        .delete(format!("{}/api/activities/{}/like", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}

/// Delete a comment (cleanup helper)
pub async fn delete_comment(client: &Client, token: &str, comment_id: &str) {
    let _ = client
        .delete(format!("{}/api/comments/{}", BASE_URL, comment_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}

/// Get user ID from token
pub async fn get_user_id(client: &Client, token: &str) -> String {
    let resp = client
        .get(format!("{}/api/users/me", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    body["id"].as_str().unwrap().to_string()
}

/// Upload GPX and return activity_id
pub async fn import_gpx(client: &Client, token: &str, gpx_content: &str) -> String {
    let form = reqwest::multipart::Form::new().text("file", gpx_content.to_string());

    let resp = client
        .post(format!("{}/api/import/gpx", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .expect("import GPX failed");

    assert!(
        resp.status().is_success(),
        "Import GPX failed: {}",
        resp.status()
    );
    let body: Value = resp.json().await.unwrap();
    body["activity_id"].as_str().unwrap().to_string()
}

/// Standard test GPX content
pub fn test_gpx() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="test">
  <trk><name>Test Ride</name><trkseg>
    <trkpt lat="52.5200" lon="13.4050"><time>2024-01-15T08:00:00Z</time></trkpt>
    <trkpt lat="52.5210" lon="13.4060"><time>2024-01-15T08:01:00Z</time></trkpt>
    <trkpt lat="52.5220" lon="13.4070"><time>2024-01-15T08:02:00Z</time></trkpt>
  </trkseg></trk>
</gpx>"#
}
