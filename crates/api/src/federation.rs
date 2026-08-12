use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use lievre_federation::exercise::{exercise_to_jsonld, map_activity_type_reverse};
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

    // Try exercises table first, fall back to direct activity lookup
    let activity = sqlx::query_as::<_, lievre_core::activity::Activity>(
        "SELECT a.* FROM exercises e JOIN activities a ON e.activity_id = a.id WHERE e.id = ?",
    )
    .bind(&exercise_id)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let activity = match activity {
        Some(a) => Some(a),
        None => sqlx::query_as::<_, lievre_core::activity::Activity>(
            "SELECT * FROM activities WHERE id = ?",
        )
        .bind(&exercise_id)
        .fetch_optional(&db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    };

    match activity {
        Some(activity) => {
            // Check visibility: public exercises return stats, others require auth
            if activity.visibility != lievre_core::Visibility::Public {
                // For non-public, we still return stats if the user is authenticated
                // In a real implementation, we'd verify the requester is the owner or a follower
                // For now, we allow access to stats for all visibility levels
            }

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

/// Exercise route endpoint (GeoJSON)
/// GET /api/exercises/:id/route
pub async fn exercise_route(
    Path(exercise_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let db = &state.fed_db;

    // Try exercises table first, fall back to direct activity lookup
    let route = sqlx::query_as::<_, lievre_core::route::Route>(
        "SELECT r.* FROM routes r JOIN exercises e ON r.activity_id = e.activity_id WHERE e.id = ?",
    )
    .bind(&exercise_id)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let route = match route {
        Some(r) => Some(r),
        None => sqlx::query_as::<_, lievre_core::route::Route>(
            "SELECT * FROM routes WHERE activity_id = ?",
        )
        .bind(&exercise_id)
        .fetch_optional(&db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    };

    match route {
        Some(route) => {
            // Check activity visibility for route access
            let activity = sqlx::query_as::<_, lievre_core::activity::Activity>(
                "SELECT * FROM activities WHERE id = ?",
            )
            .bind(&exercise_id)
            .fetch_optional(&db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(ref activity) = activity {
                // Private routes require owner access
                if activity.visibility == lievre_core::Visibility::Private {
                    // In production, verify the requester is the owner
                    // For now, we allow access but log the attempt
                    tracing::warn!(
                        "Private route access attempt for exercise {}",
                        exercise_id
                    );
                }
            }

            let geojson = state
                .route_repo
                .to_geojson(&route)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(Json(geojson))
        }
        None => Err((StatusCode::NOT_FOUND, "Route not found".to_string())),
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

        let base_url = db.base_url();

        // Pre-fetch route existence for all activities to avoid async in sync closure
        let mut activity_ids: Vec<String> = Vec::new();
        for a in &activities {
            activity_ids.push(a.id.clone());
        }

        let mut has_routes: std::collections::HashMap<String, bool> = std::collections::HashMap::new();
        for id in &activity_ids {
            let count = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM routes WHERE activity_id = ?",
            )
            .bind(id)
            .fetch_one(&db.pool)
            .await
            .unwrap_or(0);
            has_routes.insert(id.clone(), count > 0);
        }

        // Convert to ActivityStreams Create activities wrapping Exercise objects
        let items: Vec<serde_json::Value> = activities
            .iter()
            .map(|a| {
                let has_route = has_routes.get(&a.id).copied().unwrap_or(false);
                let exercise_obj = exercise_to_jsonld(a, has_route, &base_url, &user.username);
                let exercise_url = db.exercise_url(&a.id);

                json!({
                    "@context": "https://www.w3.org/ns/activitystreams",
                    "type": "Create",
                    "id": format!("{}/create", exercise_url),
                    "actor": actor_url,
                    "object": exercise_obj
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

/// Inbox endpoint
/// POST /users/:username/inbox
pub async fn inbox(
    Path(username): Path<String>,
    State(state): State<AppState>,
    _headers: HeaderMap,
    Json(activity): Json<serde_json::Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.fed_db;

    // Check if user exists
    let user = sqlx::query_as::<_, lievre_core::user::User>(
        "SELECT * FROM users WHERE username = ? AND is_local = 1",
    )
    .bind(&username)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if user.is_none() {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    // Get the activity type
    let activity_type = activity.get("type").and_then(|t| t.as_str()).unwrap_or("");

    tracing::info!("Received {} activity for user {}", activity_type, username);

    // Handle different activity types
    match activity_type {
        "Follow" => {
            // Store the follow request
            let follower_url = activity.get("actor").and_then(|a| a.as_str()).unwrap_or("");
            let follow_id = activity.get("id").and_then(|i| i.as_str()).unwrap_or("");

            if !follower_url.is_empty() {
                sqlx::query(
                    "INSERT OR IGNORE INTO actor_follows (id, follower_actor_url, following_actor_url, status)
                     VALUES (?, ?, ?, 'pending')",
                )
                .bind(follow_id)
                .bind(follower_url)
                .bind(db.actor_url(&username).to_string())
                .execute(&db.pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                tracing::info!("Stored follow request from {}", follower_url);
            }

            // In a real implementation, we would send an Accept activity back
            // For now, just accept automatically
            Ok(StatusCode::ACCEPTED)
        }
        "Accept" => {
            // Handle accept of our follow request
            let object = activity.get("object").cloned().unwrap_or(json!({}));
            if let Some(actor) = object.get("actor").and_then(|a| a.as_str()) {
                tracing::info!("Follow accepted by {}", actor);
            }
            Ok(StatusCode::ACCEPTED)
        }
        "Undo" => {
            // Handle undo (e.g., unfollow)
            let object = activity.get("object").cloned().unwrap_or(json!({}));
            if object.get("type").and_then(|t| t.as_str()) == Some("Follow") {
                let follower_url = object.get("actor").and_then(|a| a.as_str()).unwrap_or("");

                if !follower_url.is_empty() {
                    sqlx::query(
                        "DELETE FROM actor_follows WHERE follower_actor_url = ? AND following_actor_url = ?",
                    )
                    .bind(follower_url)
                    .bind(db.actor_url(&username).to_string())
                    .execute(&db.pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    tracing::info!("Removed follow from {}", follower_url);
                }
            }
            Ok(StatusCode::ACCEPTED)
        }
        "Create" => {
            // Handle create activity (e.g., new exercise)
            let object = activity.get("object").cloned().unwrap_or(json!({}));

            // Store the exercise if it's an Exercise type
            if object.get("type").and_then(|t| t.as_str()) == Some("Exercise") {
                let exercise_id = uuid::Uuid::new_v4().to_string();
                let exercise_url = object.get("id").and_then(|i| i.as_str()).unwrap_or("");
                let attributed_to = object
                    .get("attributedTo")
                    .and_then(|a| a.as_str())
                    .unwrap_or("");
                let activity_type = object
                    .get("activityType")
                    .and_then(|a| a.as_str())
                    .unwrap_or("workout");
                // Map fedisport activityType back to internal type
                let internal_activity_type = map_activity_type_reverse(activity_type);
                let started_at = object
                    .get("startedAt")
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let name = object.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let content = object.get("content").and_then(|c| c.as_str()).unwrap_or("");
                let route_url = object
                    .get("routeUrl")
                    .and_then(|r| r.as_str())
                    .unwrap_or("");
                let stats_url = object
                    .get("statsUrl")
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let published = object
                    .get("published")
                    .and_then(|p| p.as_str())
                    .unwrap_or("");

                // Find or create remote user
                let remote_user = sqlx::query_as::<_, lievre_core::user::User>(
                    "SELECT * FROM users WHERE actor_url = ?",
                )
                .bind(attributed_to)
                .fetch_optional(&db.pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                let user_id = if let Some(u) = remote_user {
                    u.id
                } else {
                    // Create a remote user
                    let new_id = uuid::Uuid::new_v4().to_string();
                    let remote_username = attributed_to.rsplit('/').next().unwrap_or("remote");
                    let remote_domain = attributed_to.split("://").nth(1).unwrap_or("unknown");

                    let email = format!("{}@{}", remote_username, remote_domain);

                    sqlx::query(
                        "INSERT INTO users (id, email, username, password_hash, actor_url, is_local)
                         VALUES (?, ?, ?, '', ?, 0)",
                    )
                    .bind(&new_id)
                    .bind(&email)
                    .bind(remote_username)
                    .bind(attributed_to)
                    .execute(&db.pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    new_id
                };

                // Store the exercise with mapped activity type
                sqlx::query(
                    "INSERT INTO exercises (id, user_id, actor_url, exercise_url, activity_type, started_at, name, content, route_url, stats_url, published_at, is_local)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
                )
                .bind(&exercise_id)
                .bind(&user_id)
                .bind(attributed_to)
                .bind(exercise_url)
                .bind(internal_activity_type)
                .bind(started_at)
                .bind(name)
                .bind(content)
                .bind(route_url)
                .bind(stats_url)
                .bind(published)
                .execute(&db.pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                tracing::info!("Stored remote exercise {}", exercise_url);
            }

            Ok(StatusCode::ACCEPTED)
        }
        "Update" | "Delete" => {
            // Handle update/delete activities
            tracing::info!(
                "Received {} activity (handling not yet implemented)",
                activity_type
            );
            Ok(StatusCode::ACCEPTED)
        }
        _ => {
            tracing::warn!("Unknown activity type: {}", activity_type);
            Ok(StatusCode::ACCEPTED)
        }
    }
}
