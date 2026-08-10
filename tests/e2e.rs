//! End-to-end tests for Lièvre API
//!
//! Each test is sandboxed: creates own data, cleans up, runs independently.
//! Requires running server: docker compose up -d
//! Run: cargo test --test e2e -- --ignored

use reqwest::Client;
use serde_json::{json, Value};

const BASE_URL: &str = "http://localhost:80";

/// Unique suffix for test isolation
fn test_id() -> String {
    format!("{}_{}", chrono::Utc::now().timestamp_millis(), rand_suffix())
}

fn rand_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    format!("{:x}", nanos)
}

/// Register a user and return (user_id, token, email, username)
async fn register_user(client: &Client, suffix: &str) -> (String, String, String, String) {
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

    assert!(resp.status().is_success(), "Registration failed: {}", resp.status());
    let body: Value = resp.json().await.unwrap();
    let token = body["token"].as_str().unwrap().to_string();
    let user_id = body["user"]["id"].as_str().unwrap().to_string();

    (user_id, token, email, username)
}

/// Delete an activity (cleanup helper)
async fn delete_activity(client: &Client, token: &str, activity_id: &str) {
    let _ = client
        .delete(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}

/// Unfollow (cleanup helper)
async fn unfollow(client: &Client, token: &str, target_id: &str) {
    let _ = client
        .delete(format!("{}/api/users/{}/follow", BASE_URL, target_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}

// ============================================================
// AUTH TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_auth_register_login_me() {
    let client = Client::new();
    let suffix = format!("auth01_{}", test_id());

    // Register
    let (user_id, token, email, username) = register_user(&client, &suffix).await;
    assert!(!user_id.is_empty());
    assert!(!token.is_empty());

    // Login with same credentials
    let resp = client
        .post(format!("{}/api/auth/login", BASE_URL))
        .json(&json!({ "email": email, "password": "testpass123" }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["token"].is_string());

    // Get current user
    let resp = client
        .get(format!("{}/api/users/me", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["id"].as_str().unwrap(), user_id);
    assert_eq!(body["username"].as_str().unwrap(), username);
}

#[tokio::test]
#[ignore]
async fn test_auth_invalid_credentials() {
    let client = Client::new();
    let suffix = format!("auth02_{}", test_id());
    let (_, _, email, _) = register_user(&client, &suffix).await;

    // Wrong password
    let resp = client
        .post(format!("{}/api/auth/login", BASE_URL))
        .json(&json!({ "email": email, "password": "wrongpass" }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);

    // Non-existent email
    let resp = client
        .post(format!("{}/api/auth/login", BASE_URL))
        .json(&json!({ "email": "noone@test.local", "password": "testpass123" }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}

// ============================================================
// ACTIVITY TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_activity_crud_lifecycle() {
    let client = Client::new();
    let suffix = format!("act01_{}", test_id());
    let (_, token, _, _) = register_user(&client, &suffix).await;

    // Create
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "activity_type": "ride",
            "title": "Test Ride",
            "started_at": "2024-01-15T08:00:00Z",
            "duration_seconds": 3600,
            "distance_meters": 50000.0
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["activity_type"].as_str().unwrap(), "ride");

    // Read
    let resp = client
        .get(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["title"].as_str().unwrap(), "Test Ride");

    // List
    let resp = client
        .get(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body.as_array().unwrap().iter().any(|a| a["id"].as_str() == Some(&activity_id)));

    // Delete
    let resp = client
        .delete(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify deleted
    let resp = client
        .get(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ============================================================
// IMPORT TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_import_gpx_creates_activity() {
    let client = Client::new();
    let suffix = format!("imp01_{}", test_id());
    let (_, token, _, _) = register_user(&client, &suffix).await;

    let gpx = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="test">
  <trk><name>Import Test</name><trkseg>
    <trkpt lat="52.5200" lon="13.4050"><time>2024-01-15T08:00:00Z</time></trkpt>
    <trkpt lat="52.5210" lon="13.4060"><time>2024-01-15T08:01:00Z</time></trkpt>
    <trkpt lat="52.5220" lon="13.4070"><time>2024-01-15T08:02:00Z</time></trkpt>
  </trkseg></trk>
</gpx>"#;

    let form = reqwest::multipart::Form::new().text("file", gpx.to_string());

    let resp = client
        .post(format!("{}/api/import/gpx", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success(), "Import failed: {}", resp.status());
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["activity_id"].as_str().unwrap().to_string();

    // Verify activity exists
    let resp = client
        .get(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify GeoJSON
    let resp = client
        .get(format!("{}/api/activities/{}/geojson", BASE_URL, activity_id))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let geo: Value = resp.json().await.unwrap();
    assert_eq!(geo["type"].as_str().unwrap(), "Feature");
    assert_eq!(geo["geometry"]["type"].as_str().unwrap(), "LineString");

    // Cleanup
    delete_activity(&client, &token, &activity_id).await;
}

#[tokio::test]
#[ignore]
async fn test_import_invalid_gpx() {
    let client = Client::new();
    let suffix = format!("imp02_{}", test_id());
    let (_, token, _, _) = register_user(&client, &suffix).await;

    let bad_gpx = "this is not xml";

    let form = reqwest::multipart::Form::new().text("file", bad_gpx.to_string());

    let resp = client
        .post(format!("{}/api/import/gpx", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ============================================================
// GEOJSON TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_geojson_nonexistent_activity() {
    let client = Client::new();
    let resp = client
        .get(format!("{}/api/activities/nonexistent/geojson", BASE_URL))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ============================================================
// SOCIAL — FOLLOW TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_follow_unfollow_lifecycle() {
    let client = Client::new();
    let ts = test_id();
    let (id_a, token_a, _, _) = register_user(&client, &format!("fola_{}", ts)).await;
    let (id_b, _token_b, _, _) = register_user(&client, &format!("folb_{}", ts)).await;

    // Follow
    let resp = client
        .post(format!("{}/api/users/{}/follow", BASE_URL, id_b))
        .header("Authorization", format!("Bearer {}", token_a))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify following list
    let resp = client
        .get(format!("{}/api/users/{}/following", BASE_URL, id_a))
        .header("Authorization", format!("Bearer {}", token_a))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 1);

    // Verify followers list
    let resp = client
        .get(format!("{}/api/users/{}/followers", BASE_URL, id_b))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 1);

    // Unfollow
    let resp = client
        .delete(format!("{}/api/users/{}/follow", BASE_URL, id_b))
        .header("Authorization", format!("Bearer {}", token_a))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify empty
    let resp = client
        .get(format!("{}/api/users/{}/following", BASE_URL, id_a))
        .header("Authorization", format!("Bearer {}", token_a))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 0);
}

// ============================================================
// SOCIAL — LIKE TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_like_unlike_lifecycle() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_owner, _, _) = register_user(&client, &format!("lkow_{}", ts)).await;
    let (_, token_liker, _, _) = register_user(&client, &format!("lkli_{}", ts)).await;

    // Create activity
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token_owner))
        .json(&json!({
            "activity_type": "ride",
            "title": "Likeable",
            "started_at": "2024-01-15T08:00:00Z",
            "visibility": "public"
        }))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    // Like
    let resp = client
        .post(format!("{}/api/activities/{}/like", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token_liker))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Unlike
    let resp = client
        .delete(format!("{}/api/activities/{}/like", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token_liker))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Cleanup
    delete_activity(&client, &token_owner, &activity_id).await;
}

// ============================================================
// SOCIAL — COMMENT TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_add_delete_comment() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_owner, _, _) = register_user(&client, &format!("cmow_{}", ts)).await;
    let (_, token_commenter, _, _) = register_user(&client, &format!("cmco_{}", ts)).await;

    // Create activity
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token_owner))
        .json(&json!({
            "activity_type": "ride",
            "title": "Commentable",
            "started_at": "2024-01-15T08:00:00Z",
            "visibility": "public"
        }))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    // Add comment
    let resp = client
        .post(format!("{}/api/activities/{}/comments", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token_commenter))
        .json(&json!({ "content": "Nice ride!" }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let comment_id = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["content"].as_str().unwrap(), "Nice ride!");

    // Get comments
    let resp = client
        .get(format!("{}/api/activities/{}/comments", BASE_URL, activity_id))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 1);

    // Delete comment
    let resp = client
        .delete(format!("{}/api/comments/{}", BASE_URL, comment_id))
        .header("Authorization", format!("Bearer {}", token_commenter))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Cleanup
    delete_activity(&client, &token_owner, &activity_id).await;
}

// ============================================================
// FEED TESTS
// ============================================================

#[tokio::test]
#[ignore]
async fn test_personal_feed() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_followed, _, _) = register_user(&client, &format!("fdfo_{}", ts)).await;
    let (_, token_follower, _, _) = register_user(&client, &format!("fdfe_{}", ts)).await;

    // Get followed user's ID
    let resp = client
        .get(format!("{}/api/users/me", BASE_URL))
        .header("Authorization", format!("Bearer {}", token_followed))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let followed_id = body["id"].as_str().unwrap().to_string();

    // Create activity as followed user
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token_followed))
        .json(&json!({
            "activity_type": "ride",
            "title": "Feed Ride",
            "started_at": "2024-01-15T08:00:00Z",
            "visibility": "public"
        }))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    // Follow
    client
        .post(format!("{}/api/users/{}/follow", BASE_URL, followed_id))
        .header("Authorization", format!("Bearer {}", token_follower))
        .send()
        .await
        .unwrap();

    // Get feed
    let resp = client
        .get(format!("{}/api/feed", BASE_URL))
        .header("Authorization", format!("Bearer {}", token_follower))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let activities = body.as_array().unwrap();
    assert!(
        activities.iter().any(|a| a["id"].as_str() == Some(&activity_id)),
        "Feed should contain the activity"
    );

    // Cleanup
    unfollow(&client, &token_follower, &followed_id).await;
    delete_activity(&client, &token_followed, &activity_id).await;
}

#[tokio::test]
#[ignore]
async fn test_public_feed() {
    let client = Client::new();
    let ts = test_id();
    let (_, token, _, _) = register_user(&client, &format!("pbfd_{}", ts)).await;

    // Create public activity
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "activity_type": "ride",
            "title": "Public Ride",
            "started_at": "2024-01-15T08:00:00Z",
            "visibility": "public"
        }))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    // Get public feed (no auth)
    let resp = client
        .get(format!("{}/api/feed/public", BASE_URL))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(
        body.as_array().unwrap().iter().any(|a| a["id"].as_str() == Some(&activity_id)),
        "Public feed should contain public activity"
    );

    // Cleanup
    delete_activity(&client, &token, &activity_id).await;
}

#[tokio::test]
#[ignore]
async fn test_private_not_in_public_feed() {
    let client = Client::new();
    let ts = test_id();
    let (_, token, _, _) = register_user(&client, &format!("pvfd_{}", ts)).await;

    // Create private activity
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "activity_type": "ride",
            "title": "Private Ride",
            "started_at": "2024-01-15T08:00:00Z",
            "visibility": "private"
        }))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    // Get public feed
    let resp = client
        .get(format!("{}/api/feed/public", BASE_URL))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert!(
        !body.as_array().unwrap().iter().any(|a| a["id"].as_str() == Some(&activity_id)),
        "Private activity should NOT be in public feed"
    );

    // Cleanup
    delete_activity(&client, &token, &activity_id).await;
}
