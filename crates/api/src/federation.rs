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

/// Outbox endpoint
/// GET /users/:username/outbox or /users/:username/outbox?page=1
pub async fn outbox(
    Path(username): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
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

    let outbox_url = db.outbox_url(&user.username);
    let actor_url = db.actor_url(&user.username);

    // Count public activities
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM activities WHERE user_id = ? AND visibility = 'public'",
    )
    .bind(&user.id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // If page parameter is provided, return the page
    if let Some(page_str) = params.get("page") {
        let page = page_str.parse::<i64>().unwrap_or(1);
        let limit = 20;
        let offset = (page - 1) * limit;

        // Get public activities
        let activities = sqlx::query_as::<_, lievre_core::activity::Activity>(
            "SELECT * FROM activities WHERE user_id = ? AND visibility = 'public' ORDER BY started_at DESC LIMIT ? OFFSET ?",
        )
        .bind(&user.id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // Convert to ActivityStreams Create activities
        let items: Vec<serde_json::Value> = activities
            .iter()
            .map(|a| {
                let exercise_url = db.exercise_url(&a.id);
                json!({
                    "@context": "https://www.w3.org/ns/activitystreams",
                    "type": "Create",
                    "id": format!("{}/create", exercise_url),
                    "actor": actor_url,
                    "object": {
                        "@context": [
                            "https://www.w3.org/ns/activitystreams",
                            "https://fedisport.github.io/vocabulary/context.jsonld"
                        ],
                        "type": "Exercise",
                        "id": exercise_url,
                        "attributedTo": actor_url,
                        "activityType": a.activity_type,
                        "startedAt": a.started_at.to_rfc3339(),
                        "name": a.title,
                        "routeUrl": db.route_url(&a.id),
                        "statsUrl": db.stats_url(&a.id),
                        "published": a.created_at.to_rfc3339(),
                        "to": ["https://www.w3.org/ns/activitystreams#Public"],
                        "cc": [format!("{}/followers", actor_url)],
                    }
                })
            })
            .collect();

        let page_data = json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "type": "OrderedCollectionPage",
            "id": format!("{}?page={}", outbox_url, page),
            "partOf": outbox_url,
            "totalItems": count,
            "orderedItems": items,
        });

        return Ok(Json(page_data));
    }

    // Otherwise return the collection metadata
    let outbox = json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "type": "OrderedCollection",
        "id": outbox_url,
        "totalItems": count,
        "first": format!("{}?page=1", outbox_url),
    });

    Ok(Json(outbox))
}
