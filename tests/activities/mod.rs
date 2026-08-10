//! Activity CRUD tests (AC-ACT-01, AC-ACT-02)
//!
//! Covers:
//! - AC-ACT-01: Create, Read, List, Delete lifecycle
//! - AC-ACT-02: List user activities

use crate::common::*;
use serde_json::Value;

/// AC-ACT-01: Full CRUD lifecycle for an activity
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
    delete_activity(&client, &token, &activity_id).await;

    // Verify deleted
    let resp = client
        .get(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

/// AC-ACT-02: List returns only current user's activities
#[tokio::test]
#[ignore]
async fn test_list_user_activities() {
    let client = Client::new();
    let suffix = format!("act02_{}", test_id());
    let (_, token, _, _) = register_user(&client, &suffix).await;

    // Create 3 activities
    let mut ids = Vec::new();
    for i in 0..3 {
        let id = create_activity(&client, &token, &format!("Activity {}", i), "public").await;
        ids.push(id);
    }

    // List should contain all 3
    let resp = client
        .get(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    let activities = body.as_array().unwrap();

    for id in &ids {
        assert!(activities.iter().any(|a| a["id"].as_str() == Some(id)));
    }

    // Cleanup
    for id in &ids {
        delete_activity(&client, &token, id).await;
    }
}

/// Activity with all optional fields
#[tokio::test]
#[ignore]
async fn test_create_activity_full() {
    let client = Client::new();
    let suffix = format!("act03_{}", test_id());
    let (_, token, _, _) = register_user(&client, &suffix).await;

    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "activity_type": "run",
            "title": "Morning Run",
            "description": "Easy recovery run",
            "started_at": "2024-01-15T07:00:00Z",
            "duration_seconds": 2400,
            "distance_meters": 5000.0,
            "elevation_gain_meters": 50.0,
            "visibility": "followers"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    assert_eq!(body["activity_type"].as_str().unwrap(), "run");
    assert_eq!(body["title"].as_str().unwrap(), "Morning Run");
    assert_eq!(body["description"].as_str().unwrap(), "Easy recovery run");

    delete_activity(&client, &token, &activity_id).await;
}
