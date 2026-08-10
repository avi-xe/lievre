//! Authentication tests (AC-AUTH-01, AC-AUTH-02, AC-AUTH-03)
//!
//! Covers:
//! - AC-AUTH-01: Register + Login + Get Current User
//! - AC-AUTH-02: Invalid credentials
//! - AC-AUTH-03: Duplicate registration

use crate::common::*;
use serde_json::Value;

/// AC-AUTH-01: Full registration, login, and profile retrieval flow
#[tokio::test]
#[ignore]
async fn test_register_login_me() {
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

/// AC-AUTH-02: Login with wrong password or non-existent email fails
#[tokio::test]
#[ignore]
async fn test_invalid_credentials() {
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

/// AC-AUTH-03: Registering with duplicate email fails
#[tokio::test]
#[ignore]
async fn test_duplicate_registration() {
    let client = Client::new();
    let suffix = format!("auth03_{}", test_id());
    let (_, _, email, _) = register_user(&client, &suffix).await;

    // Try to register again with same email
    let resp = client
        .post(format!("{}/api/auth/register", BASE_URL))
        .json(&json!({
            "email": email,
            "username": format!("other_{}", suffix),
            "password": "testpass123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400); // or 409 Conflict
}
