//! Federation E2E tests
//!
//! Tests ActivityPub federation endpoints

use crate::common::*;
use serde_json::Value;

/// Test WebFinger discovery
#[tokio::test]
#[ignore]
async fn test_webfinger_discovery() {
    let client = Client::new();
    let suffix = format!("fed01_{}", test_id());
    let (_, _, email, _) = register_user(&client, &suffix).await;

    let resp = client
        .get(format!(
            "{}/.well-known/webfinger?resource=acct:{}@localhost",
            BASE_URL, email
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["subject"], format!("acct:{}@localhost", email));
    assert!(body["links"].is_array());
    let links = body["links"].as_array().unwrap();
    assert!(!links.is_empty());

    // Should have a self link to the actor
    let self_link = links.iter().find(|l| l["rel"] == "self");
    assert!(self_link.is_some(), "Should have self link");
}

/// Test actor endpoint
#[tokio::test]
#[ignore]
async fn test_actor_endpoint() {
    let client = Client::new();
    let suffix = format!("fed02_{}", test_id());
    let (_, _, _, username) = register_user(&client, &suffix).await;

    let resp = client
        .get(format!("{}/users/{}", BASE_URL, username))
        .header("Accept", "application/activity+json")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["type"], "Person");
    assert_eq!(body["preferredUsername"], username);
    assert!(body["inbox"].is_string());
    assert!(body["outbox"].is_string());
    assert!(body["publicKey"]["publicKeyPem"].is_string());
}

/// Test outbox endpoint
#[tokio::test]
#[ignore]
async fn test_outbox_endpoint() {
    let client = Client::new();
    let suffix = format!("fed03_{}", test_id());
    let (_, token, _, username) = register_user(&client, &suffix).await;

    // Create a public activity
    let activity_id = create_activity(&client, &token, "Outbox Test", "public").await;

    let resp = client
        .get(format!("{}/users/{}/outbox", BASE_URL, username))
        .header("Accept", "application/activity+json")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["type"], "OrderedCollection");
    assert!(body["totalItems"].as_i64().unwrap() >= 1);
    assert!(body["first"].is_string());

    // Cleanup
    delete_activity(&client, &token, &activity_id).await;
}

/// Test outbox pagination
#[tokio::test]
#[ignore]
async fn test_outbox_pagination() {
    let client = Client::new();
    let suffix = format!("fed04_{}", test_id());
    let (_, token, _, username) = register_user(&client, &suffix).await;

    // Create multiple activities
    let mut ids = Vec::new();
    for i in 0..3 {
        let id = create_activity(&client, &token, &format!("Page Test {}", i), "public").await;
        ids.push(id);
    }

    let resp = client
        .get(format!("{}/users/{}/outbox?page=1", BASE_URL, username))
        .header("Accept", "application/activity+json")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["type"], "OrderedCollectionPage");
    assert!(body["orderedItems"].is_array());
    let items = body["orderedItems"].as_array().unwrap();
    assert!(!items.is_empty());

    // Each item should be a Create activity
    for item in items {
        assert_eq!(item["type"], "Create");
        assert!(item["object"]["type"] == "Exercise");
    }

    // Cleanup
    for id in ids {
        delete_activity(&client, &token, &id).await;
    }
}

/// Test exercise stats endpoint
#[tokio::test]
#[ignore]
async fn test_exercise_stats() {
    let client = Client::new();
    let suffix = format!("fed05_{}", test_id());
    let (_, token, _, _) = register_user(&client, &suffix).await;

    // Create an activity with stats
    let resp = client
        .post(format!("{}/api/activities", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "activity_type": "ride",
            "title": "Stats Test",
            "started_at": "2024-01-15T08:00:00Z",
            "duration_seconds": 3600,
            "distance_meters": 50000.0,
            "visibility": "public"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let body: Value = resp.json().await.unwrap();
    let activity_id = body["id"].as_str().unwrap().to_string();

    // Get stats (note: exercise_id is same as activity_id for now)
    let resp = client
        .get(format!("{}/api/exercises/{}/stats", BASE_URL, activity_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let stats: Value = resp.json().await.unwrap();
    assert_eq!(stats["distance"], 50000.0);
    assert_eq!(stats["duration"], 3600);

    // Cleanup
    delete_activity(&client, &token, &activity_id).await;
}

/// Test inbox receives follow activity
#[tokio::test]
#[ignore]
async fn test_inbox_follow() {
    let client = Client::new();
    let suffix = format!("fed06_{}", test_id());
    let (_, _, _, username) = register_user(&client, &suffix).await;

    let follow_activity = json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "type": "Follow",
        "id": "https://remote.example/user1/follow/123",
        "actor": "https://remote.example/user1",
        "object": format!("{}/users/{}", BASE_URL, username)
    });

    let resp = client
        .post(format!("{}/users/{}/inbox", BASE_URL, username))
        .header("Content-Type", "application/activity+json")
        .json(&follow_activity)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 202);
}
