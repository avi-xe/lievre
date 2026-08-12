use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

/// Extract Bearer token from Authorization header
async fn auth_user(
    state: &crate::AppState,
    headers: &axum::http::header::HeaderMap,
) -> Result<lievre_core::user::User, (StatusCode, String)> {
    let token = crate::auth::extract_token(headers)?;
    state
        .auth
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))
}

// ============================================================
// FOLLOW
// ============================================================

/// POST /api/users/:id/follow
pub async fn follow_user(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(target_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    state
        .social
        .follow(&user.id, &target_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Notify the target user (ignore self-follows)
    if user.id != target_id {
        let _ = state
            .notification_repo
            .create(&target_id, &user.id, "follow", "user", &user.id, None)
            .await;
    }

    Ok(Json(json!({ "ok": true })))
}

/// DELETE /api/users/:id/follow
pub async fn unfollow_user(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(target_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    state
        .social
        .unfollow(&user.id, &target_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!({ "ok": true })))
}

// ============================================================
// LIKE
// ============================================================

/// POST /api/activities/:id/like
pub async fn like_activity(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(activity_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    state
        .social
        .like(&activity_id, &user.id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Notify activity owner (ignore self-likes)
    if let Ok(Some(activity)) = state.activity_repo.find_by_id(&activity_id).await {
        if activity.user_id != user.id {
            let _ = state
                .notification_repo
                .create(
                    &activity.user_id,
                    &user.id,
                    "like",
                    "activity",
                    &activity_id,
                    None,
                )
                .await;
        }
    }

    Ok(Json(json!({ "ok": true })))
}

/// DELETE /api/activities/:id/like
pub async fn unlike_activity(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(activity_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    state
        .social
        .unlike(&activity_id, &user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!({ "ok": true })))
}

// ============================================================
// FOLLOWERS/FOLLOWING LISTS
// ============================================================

/// GET /api/users/:id/followers
pub async fn get_followers(
    State(state): State<crate::AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let followers = state
        .social
        .get_followers(&user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!(followers)))
}

/// GET /api/users/:id/following
pub async fn get_following(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _ = auth_user(&state, &headers).await?;
    let following = state
        .social
        .get_following(&user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!(following)))
}

/// GET /api/activities/:id/comments
pub async fn get_comments(
    State(state): State<crate::AppState>,
    Path(activity_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let comments = state
        .social
        .get_comments(&activity_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!(comments)))
}

// ============================================================
// COMMENTS
// ============================================================

#[derive(Deserialize)]
pub struct CommentBody {
    pub content: String,
}

/// POST /api/activities/:id/comments
pub async fn add_comment(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(activity_id): Path<String>,
    Json(body): Json<CommentBody>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    let comment = state
        .social
        .add_comment(&activity_id, &user.id, &body.content)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Notify activity owner (ignore self-comments)
    if let Ok(Some(activity)) = state.activity_repo.find_by_id(&activity_id).await {
        if activity.user_id != user.id {
            let preview = if body.content.len() > 100 {
                format!("{}…", &body.content[..100])
            } else {
                body.content.clone()
            };
            let _ = state
                .notification_repo
                .create(
                    &activity.user_id,
                    &user.id,
                    "comment",
                    "activity",
                    &activity_id,
                    Some(&preview),
                )
                .await;
        }
    }

    Ok(Json(json!(comment)))
}

/// DELETE /api/comments/:id
pub async fn delete_comment(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(comment_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    state
        .social
        .delete_comment(&comment_id, &user.id)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, e.to_string()))?;
    Ok(Json(json!({ "ok": true })))
}
