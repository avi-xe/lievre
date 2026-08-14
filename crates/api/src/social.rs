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

/// GET /api/users/:id/follow-status — check if current user follows target
pub async fn follow_status(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(target_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    let following = state
        .social
        .is_following(&user.id, &target_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!({ "is_following": following })))
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

    // Check if this is a remote activity
    let is_remote = state
        .social
        .is_remote_activity(&activity_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if is_remote {
        // For remote activities, we need to send a Like activity to the remote inbox
        // First, get the remote exercise URL
        let exercise_url = state
            .social
            .get_remote_exercise_url(&activity_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    "Remote activity not found".to_string(),
                )
            })?;

        // Create the local like record (for tracking)
        let like = state
            .social
            .like(&activity_id, &user.id)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        // Build the Like activity for federation
        let base_url = state.fed_db.base_url();
        let like_id = format!("{}/likes/{}", base_url, like.id);
        let actor_url = state.fed_db.actor_url(&user.username).to_string();

        let like_activity = json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "type": "Like",
            "id": like_id,
            "actor": actor_url,
            "object": exercise_url,
        });

        // TODO: Determine the remote inbox URL from the exercise URL
        // For now, we'll log the activity that would be sent
        tracing::info!(
            "Would send Like activity to remote inbox for exercise {}: {}",
            exercise_url,
            like_activity
        );

        // In a real implementation, we would:
        // 1. Fetch the remote actor's inbox URL
        // 2. Sign the activity with HTTP Signatures
        // 3. Send it to the remote inbox
        // For now, we just log it
    } else {
        // For local activities, just create the like record
        state
            .social
            .like(&activity_id, &user.id)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    }

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

/// GET /api/activities/:id/likes — list who liked an activity
pub async fn get_likes(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(activity_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let likes = state
        .social
        .get_likes(&activity_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let count = state
        .social
        .get_like_count(&activity_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Check if current user has liked (optional — unauthenticated returns false)
    let liked = match auth_user(&state, &headers).await {
        Ok(user) => state
            .social
            .has_liked(&activity_id, &user.id)
            .await
            .unwrap_or(false),
        Err(_) => false,
    };

    Ok(Json(
        json!({ "likes": likes, "count": count, "liked": liked }),
    ))
}
