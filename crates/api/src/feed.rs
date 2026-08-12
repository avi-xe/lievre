use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value;

/// GET /api/feed — personal feed (auth required)
pub async fn personal_feed(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = crate::auth::extract_token(&headers)?;
    let user = state
        .auth
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    let activities = state
        .social
        .get_feed_with_details(&user.id, 50, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!(activities)))
}

/// GET /api/feed/public — public feed (no auth required)
pub async fn public_feed(
    State(state): State<crate::AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let activities = state
        .social
        .get_public_feed_with_details(50, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!(activities)))
}
