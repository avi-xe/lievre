use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use lievre_federation::webfinger::WebfingerQuery;
use lievre_federation::{Person, WebfingerResponse};
use serde_json::json;

use crate::AppState;

/// WebFinger handler
/// GET /.well-known/webfinger?resource=acct:user@domain
pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    State(state): State<AppState>,
) -> Result<Json<WebfingerResponse>, (StatusCode, String)> {
    lievre_federation::webfinger::webfinger_handler(Query(query), State(state.fed_db)).await
}

/// Actor endpoint
/// GET /users/:username
pub async fn actor(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Person>, (StatusCode, String)> {
    let db = &state.fed_db;

    // Check if user exists
    let user = sqlx::query_as::<_, lievre_core::user::User>(
        "SELECT * FROM users WHERE username = ? AND is_local = 1",
    )
    .bind(&username)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
        }
    };

    let actor_url = db.actor_url(&user.username);
    let inbox_url = db.inbox_url(&user.username);
    let outbox_url = db.outbox_url(&user.username);

    let person = Person {
        kind: Default::default(),
        id: actor_url.clone(),
        preferred_username: user.username.clone(),
        name: user.display_name.clone(),
        inbox: inbox_url,
        outbox: outbox_url,
        following: None,
        followers: None,
        public_key: activitypub_federation::protocol::public_key::PublicKey {
            id: format!("{}#main-key", actor_url),
            owner: actor_url,
            public_key_pem: user.public_key.unwrap_or_default(),
        },
        icon: user.avatar_url.map(|url| lievre_federation::PersonIcon {
            kind: "Image".to_string(),
            media_type: "image/png".to_string(),
            url: url.parse().expect("Invalid avatar URL"),
        }),
    };

    Ok(Json(person))
}

/// Exercise stats endpoint
/// GET /api/exercises/:id/stats
pub async fn exercise_stats(
    Path(exercise_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let db = &state.fed_db;

    // Look up the exercise
    let exercise = sqlx::query_as::<_, lievre_core::activity::Activity>(
        "SELECT a.* FROM exercises e JOIN activities a ON e.activity_id = a.id WHERE e.id = ?",
    )
    .bind(&exercise_id)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match exercise {
        Some(activity) => {
            let stats = json!({
                "distance": activity.distance_meters,
                "duration": activity.duration_seconds,
                "elevationGain": activity.elevation_gain_meters,
            });
            Ok(Json(stats))
        }
        None => Err((StatusCode::NOT_FOUND, "Exercise not found".to_string())),
    }
}
