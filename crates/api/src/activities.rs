use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use lievre_core::{ActivityType, CreateActivity, Visibility};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct CreateActivityBody {
    pub activity_type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub started_at: String,
    pub duration_seconds: Option<i64>,
    pub distance_meters: Option<f64>,
    pub elevation_gain_meters: Option<f64>,
    pub visibility: Option<String>,
}

/// POST /api/activities — create a new activity (auth required)
pub async fn create_activity(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Json(body): Json<CreateActivityBody>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, String)> {
    let token = crate::auth::extract_token(&headers)?;
    let user = state
        .auth
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let activity_type = match body.activity_type.as_str() {
        "ride" => ActivityType::Ride,
        "run" => ActivityType::Run,
        "swim" => ActivityType::Swim,
        other => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid activity_type: {other}"),
            ))
        }
    };

    let started_at: DateTime<Utc> = body
        .started_at
        .parse()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid started_at: {e}")))?;

    let visibility = match body.visibility.as_deref() {
        Some("public") => Some(Visibility::Public),
        Some("private") => Some(Visibility::Private),
        Some("followers") => Some(Visibility::Followers),
        Some(other) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid visibility: {other}"),
            ))
        }
        None => None,
    };

    let create = CreateActivity {
        activity_type,
        title: body.title,
        description: body.description,
        started_at,
        duration_seconds: body.duration_seconds,
        distance_meters: body.distance_meters,
        elevation_gain_meters: body.elevation_gain_meters,
        visibility,
    };

    let activity = state
        .activity_repo
        .create(&user.id, create)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(json!(activity))))
}

/// GET /api/activities — list current user's activities (auth required)
pub async fn list_activities(
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
        .activity_repo
        .find_by_user_id(&user.id, 50, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!(activities)))
}

/// GET /api/activities/:id — get a single activity (auth required)
pub async fn get_activity(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = crate::auth::extract_token(&headers)?;
    let _user = state
        .auth
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let activity = state
        .activity_repo
        .find_by_id(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match activity {
        Some(a) => Ok(Json(json!(a))),
        None => Err((StatusCode::NOT_FOUND, "Activity not found".to_string())),
    }
}

/// DELETE /api/activities/:id — delete an activity (auth required, must be owner)
pub async fn delete_activity(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = crate::auth::extract_token(&headers)?;
    let user = state
        .auth
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let activity = state
        .activity_repo
        .find_by_id(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match activity {
        Some(a) => {
            if a.user_id != user.id {
                return Err((StatusCode::FORBIDDEN, "Not your activity".to_string()));
            }
        }
        None => {
            return Err((StatusCode::NOT_FOUND, "Activity not found".to_string()));
        }
    }

    state
        .activity_repo
        .delete(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "deleted": true })))
}
