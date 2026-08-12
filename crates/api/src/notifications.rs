use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    http::header::HeaderMap,
    Json,
};
use serde_json::{json, Value};

/// Extract user from Bearer token
async fn auth_user(
    state: &crate::AppState,
    headers: &HeaderMap,
) -> Result<lievre_core::user::User, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

    let token = auth_header.strip_prefix("Bearer ").ok_or((
        StatusCode::UNAUTHORIZED,
        "Invalid Authorization header".to_string(),
    ))?;

    state
        .auth
        .verify_token(token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))
}

/// GET /api/notifications — list notifications for current user
pub async fn list_notifications(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;

    let notifications = state
        .notification_repo
        .list(&user.id, 50, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let unread = state
        .notification_repo
        .unread_count(&user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "notifications": notifications,
        "unread_count": unread,
    })))
}

/// PUT /api/notifications/:id/read — mark one as read
pub async fn mark_read(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Path(notification_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;

    let marked = state
        .notification_repo
        .mark_read(&notification_id, &user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if marked {
        Ok(Json(json!({"ok": true})))
    } else {
        Err((StatusCode::NOT_FOUND, "Notification not found".to_string()))
    }
}

/// PUT /api/notifications/read-all — mark all as read
pub async fn mark_all_read(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;

    let count = state
        .notification_repo
        .mark_all_read(&user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({"ok": true, "marked": count})))
}

/// DELETE /api/notifications/:id — delete a notification
pub async fn delete_notification(
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Path(notification_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;

    let deleted = state
        .notification_repo
        .delete(&notification_id, &user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(Json(json!({"ok": true})))
    } else {
        Err((StatusCode::NOT_FOUND, "Notification not found".to_string()))
    }
}
