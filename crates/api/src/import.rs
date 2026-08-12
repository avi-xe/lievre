use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use lievre_core::{GpxParser, StravaParser, TcxParser};

#[derive(Debug, serde::Serialize)]
pub struct ImportResponse {
    pub activity_id: String,
    pub message: String,
}

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

/// POST /api/import/gpx — import a GPX file
pub async fn import_gpx(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    let parser = GpxParser::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("unknown").to_string();

        if name == "file" {
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            let content = String::from_utf8(data.to_vec())
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let track = parser
                .parse(&content)
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            tracing::info!(
                "GPX parsed: name={:?}, coords={}, elevation={}, distance={:?}, elevation_gain={:?}",
                track.name,
                track.coordinates.len(),
                track.elevation_data.len(),
                track.distance_meters,
                track.elevation_gain_meters
            );

            let create_activity = parser.to_create_activity(&track);
            let activity = state
                .activity_repo
                .create(&user.id, create_activity)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if !track.coordinates.is_empty() {
                let create_route = parser.to_create_route(&activity.id, &track);
                if let Err(e) = state.route_repo.create(create_route).await {
                    tracing::error!("Failed to create route for activity {}: {}", activity.id, e);
                }
            }

            return Ok(Json(ImportResponse {
                activity_id: activity.id,
                message: format!("Successfully imported from {}", name),
            }));
        }
    }

    Err((StatusCode::BAD_REQUEST, "No file provided".to_string()))
}

/// POST /api/import/tcx — import a TCX file
pub async fn import_tcx(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    let parser = TcxParser::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("unknown").to_string();

        if name == "file" {
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            let content = String::from_utf8(data.to_vec())
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let tcx_activity = parser
                .parse(&content)
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let create_activity = parser.to_create_activity(&tcx_activity);
            let activity = state
                .activity_repo
                .create(&user.id, create_activity)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let create_route = parser.to_create_route(&activity.id, &tcx_activity);
            let _ = state.route_repo.create(create_route).await;

            return Ok(Json(ImportResponse {
                activity_id: activity.id,
                message: format!("Successfully imported from {}", name),
            }));
        }
    }

    Err((StatusCode::BAD_REQUEST, "No file provided".to_string()))
}

/// POST /api/import/strava — import Strava CSV export
pub async fn import_strava(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, (StatusCode, String)> {
    let user = auth_user(&state, &headers).await?;
    let parser = StravaParser::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("unknown").to_string();

        if name == "file" {
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            let content = String::from_utf8(data.to_vec())
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let activities = parser
                .parse_activities_csv(&content)
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            if let Some(strava_activity) = activities.into_iter().next() {
                let create_activity = parser.to_create_activity(&strava_activity);
                let activity = state
                    .activity_repo
                    .create(&user.id, create_activity)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                if !strava_activity.coordinates.is_empty() {
                    let create_route = parser.to_create_route(&activity.id, &strava_activity);
                    let _ = state.route_repo.create(create_route).await;
                }

                return Ok(Json(ImportResponse {
                    activity_id: activity.id,
                    message: format!("Successfully imported from {}", name),
                }));
            }
        }
    }

    Err((StatusCode::BAD_REQUEST, "No activities found".to_string()))
}
