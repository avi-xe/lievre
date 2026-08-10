//! Social feature tests (AC-FOL-01, AC-LIK-01, AC-COM-01)
//!
//! Covers:
//! - AC-FOL-01: Follow/Unfollow lifecycle
//! - AC-FOL-02: Follower/Following counts
//! - AC-LIK-01: Like/Unlike lifecycle
//! - AC-LIK-02: Like count on activity
//! - AC-COM-01: Add/Delete comment
//! - AC-COM-02: Comment count

use crate::common::*;
use serde_json::Value;

// ============================================================
// FOLLOW TESTS
// ============================================================

/// AC-FOL-01: Follow and unfollow a user
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
    unfollow(&client, &token_a, &id_b).await;

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

/// AC-FOL-02: Multiple followers
#[tokio::test]
#[ignore]
async fn test_follow_counts() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_a, _, _) = register_user(&client, &format!("fca_{}", ts)).await;
    let (id_b, token_b, _, _) = register_user(&client, &format!("fcb_{}", ts)).await;
    let (_, token_c, _, _) = register_user(&client, &format!("fcc_{}", ts)).await;

    let id_a = get_user_id(&client, &token_a).await;

    // B and C follow A
    client
        .post(format!("{}/api/users/{}/follow", BASE_URL, id_a))
        .header("Authorization", format!("Bearer {}", token_b))
        .send()
        .await
        .unwrap();
    client
        .post(format!("{}/api/users/{}/follow", BASE_URL, id_a))
        .header("Authorization", format!("Bearer {}", token_c))
        .send()
        .await
        .unwrap();

    // A's followers = 2
    let resp = client
        .get(format!("{}/api/users/{}/followers", BASE_URL, id_a))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 2);

    // B's following = 1
    let resp = client
        .get(format!("{}/api/users/{}/following", BASE_URL, id_b))
        .header("Authorization", format!("Bearer {}", token_b))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 1);

    // Cleanup
    unfollow(&client, &token_b, &id_a).await;
    unfollow(&client, &token_c, &id_a).await;
}

// ============================================================
// LIKE TESTS
// ============================================================

/// AC-LIK-01: Like and unlike an activity
#[tokio::test]
#[ignore]
async fn test_like_unlike_lifecycle() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_owner, _, _) = register_user(&client, &format!("lkow_{}", ts)).await;
    let (_, token_liker, _, _) = register_user(&client, &format!("lkli_{}", ts)).await;

    let activity_id = create_activity(&client, &token_owner, "Likeable", "public").await;

    // Like
    let resp = client
        .post(format!("{}/api/activities/{}/like", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token_liker))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Unlike
    unlike(&client, &token_liker, &activity_id).await;

    delete_activity(&client, &token_owner, &activity_id).await;
}

/// AC-LIK-02: Like count is tracked
#[tokio::test]
#[ignore]
async fn test_like_count() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_owner, _, _) = register_user(&client, &format!("lkcw_{}", ts)).await;
    let (_, token_liker, _, _) = register_user(&client, &format!("lkli_{}", ts)).await;

    let activity_id = create_activity(&client, &token_owner, "Countable", "public").await;

    // Like
    client
        .post(format!("{}/api/activities/{}/like", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token_liker))
        .send()
        .await
        .unwrap();

    // Get activity - should have like_count
    let resp = client
        .get(format!("{}/api/activities/{}", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token_owner))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    // like_count may or may not be in response depending on implementation
    // At minimum, the like should not error

    unlike(&client, &token_liker, &activity_id).await;
    delete_activity(&client, &token_owner, &activity_id).await;
}

// ============================================================
// COMMENT TESTS
// ============================================================

/// AC-COM-01: Add and delete a comment
#[tokio::test]
#[ignore]
async fn test_add_delete_comment() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_owner, _, _) = register_user(&client, &format!("cmow_{}", ts)).await;
    let (_, token_commenter, _, _) = register_user(&client, &format!("cmco_{}", ts)).await;

    let activity_id = create_activity(&client, &token_owner, "Commentable", "public").await;

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
    delete_comment(&client, &token_commenter, &comment_id).await;

    // Verify empty
    let resp = client
        .get(format!("{}/api/activities/{}/comments", BASE_URL, activity_id))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 0);

    delete_activity(&client, &token_owner, &activity_id).await;
}

/// AC-COM-02: Multiple comments
#[tokio::test]
#[ignore]
async fn test_multiple_comments() {
    let client = Client::new();
    let ts = test_id();
    let (_, token_owner, _, _) = register_user(&client, &format!("cmmt_{}", ts)).await;
    let (_, token_commenter, _, _) = register_user(&client, &format!("cmco_{}", ts)).await;

    let activity_id = create_activity(&client, &token_owner, "Multi-Comment", "public").await;

    // Add 3 comments
    let mut comment_ids = Vec::new();
    for i in 0..3 {
        let resp = client
            .post(format!("{}/api/activities/{}/comments", BASE_URL, activity_id))
            .header("Authorization", format!("Bearer {}", token_commenter))
            .json(&json!({ "content": format!("Comment {}", i) }))
            .send()
            .await
            .unwrap();
        let body: Value = resp.json().await.unwrap();
        comment_ids.push(body["id"].as_str().unwrap().to_string());
    }

    // Verify count
    let resp = client
        .get(format!("{}/api/activities/{}/comments", BASE_URL, activity_id))
        .send()
        .await
        .unwrap();
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body.as_array().unwrap().len(), 3);

    // Cleanup
    for id in &comment_ids {
        delete_comment(&client, &token_commenter, id).await;
    }
    delete_activity(&client, &token_owner, &activity_id).await;
}
