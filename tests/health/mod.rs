//! Health endpoint tests (AC-HEALTH-01)

use crate::common::{Client, BASE_URL};

#[tokio::test]
#[ignore]
async fn test_health_returns_ok() {
    let client = Client::new();
    let resp = client
        .get(format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("request failed");

    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "OK");
}
