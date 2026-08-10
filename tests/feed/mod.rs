//! Feed tests (AC-FEED-01, AC-FEED-02, AC-FEED-03)
//!
//! Covers:
//! - AC-FEED-01: Personal feed (followed users)
//! - AC-FEED-02: Public feed (no auth)
//! - AC-FEED-03: Private activity not in public feed

use crate::common::*;
use serde_json::Value;

/// AC-FEED-01: Personal feed shows followed users' activities
#[tokio::test]
#[ignore]
async fn test_personal_feed() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_followed, _, _) = register_user(&client, &format!("fdfo_{}", ts)).await;
    let (_, token_follower, _, _) = register_user(&client, &format!("fdfe_{}", ts)).await;

    let followed_id = get_user_id(&client, &token_followed).await;

    // Create activity as followed user
    let activity_id = create_activity(&client, &token_followed, "Feed Ride", "public").await;

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

/// AC-FEED-02: Public feed shows public activities without auth
#[tokio::test]
#[ignore]
async fn test_public_feed() {
    let client = Client::new();
    let ts = test_id();
    let (_, token, _, _) = register_user(&client, &format!("pbfd_{}", ts)).await;

    let activity_id = create_activity(&client, &token, "Public Ride", "public").await;

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

    delete_activity(&client, &token, &activity_id).await;
}

/// AC-FEED-03: Private activity NOT in public feed
#[tokio::test]
#[ignore]
async fn test_private_not_in_public_feed() {
    let client = Client::new();
    let ts = test_id();
    let (_, token, _, _) = register_user(&client, &format!("pvfd_{}", ts)).await;

    let activity_id = create_activity(&client, &token, "Private Ride", "private").await;

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

    delete_activity(&client, &token, &activity_id).await;
}

/// Feed is sorted by newest first
#[tokio::test]
#[ignore]
async fn test_feed_sorted_by_time() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_followed, _, _) = register_user(&client, &format!("fdso_{}", ts)).await;
    let (_, token_follower, _, _) = register_user(&client, &format!("fdsf_{}", ts)).await;

    let followed_id = get_user_id(&client, &token_followed).await;

    // Create 3 activities with different times
    let mut ids = Vec::new();
    for i in 0..3 {
        let resp = client
            .post(format!("{}/api/activities", BASE_URL))
            .header("Authorization", format!("Bearer {}", token_followed))
            .json(&json!({
                "activity_type": "ride",
                "title": format!("Ride {}", i),
                "started_at": format!("2024-01-15T0{}:00:00Z", 8 + i),
                "visibility": "public"
            }))
            .send()
            .await
            .unwrap();
        let body: Value = resp.json().await.unwrap();
        ids.push(body["id"].as_str().unwrap().to_string());
    }

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
    let body: Value = resp.json().await.unwrap();
    let activities = body.as_array().unwrap();

    // Should be sorted newest first ( Ride 2, Ride 1, Ride 0)
    if activities.len() >= 2 {
        let first_time = activities[0]["started_at"].as_str().unwrap();
        let second_time = activities[1]["started_at"].as_str().unwrap();
        assert!(first_time >= second_time, "Feed should be sorted newest first");
    }

    // Cleanup
    unfollow(&client, &token_follower, &followed_id).await;
    for id in &ids {
        delete_activity(&client, &token_followed, id).await;
    }
}
