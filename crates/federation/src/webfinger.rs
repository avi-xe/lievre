use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::config::FederationDb;
use crate::exercise::{WebfingerLink, WebfingerResponse};

#[derive(Deserialize)]
pub struct WebfingerQuery {
    pub resource: String,
}

/// Handle WebFinger requests
///
/// GET /.well-known/webfinger?resource=acct:user@domain
pub async fn webfinger_handler(
    Query(query): Query<WebfingerQuery>,
    axum::extract::State(db): axum::extract::State<FederationDb>,
) -> Result<Json<WebfingerResponse>, (axum::http::StatusCode, String)> {
    // Parse the resource URI: acct:user@domain
    let resource = &query.resource;
    if !resource.starts_with("acct:") {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "Invalid resource format".to_string(),
        ));
    }

    let acct = resource.strip_prefix("acct:").unwrap();
    let parts: Vec<&str> = acct.split('@').collect();
    if parts.len() != 2 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "Invalid acct format".to_string(),
        ));
    }

    let username = parts[0];
    let domain = parts[1];

    // Check if domain matches our instance
    if domain != db.domain {
        return Err((
            axum::http::StatusCode::NOT_FOUND,
            "User not found".to_string(),
        ));
    }

    // Check if user exists
    let user = sqlx::query_as::<_, lievre_core::user::User>(
        "SELECT * FROM users WHERE username = ? AND is_local = 1",
    )
    .bind(username)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err((
                axum::http::StatusCode::NOT_FOUND,
                "User not found".to_string(),
            ));
        }
    };

    let actor_url = db.actor_url(&user.username);

    let response = WebfingerResponse {
        subject: resource.clone(),
        aliases: vec![actor_url.to_string()],
        links: vec![
            WebfingerLink {
                rel: "self".to_string(),
                kind: Some("application/activity+json".to_string()),
                href: actor_url.to_string(),
            },
            WebfingerLink {
                rel: "http://webfinger.net/rel/profile-page".to_string(),
                kind: Some("text/html".to_string()),
                href: format!("{}/users/{}", db.base_url(), user.username),
            },
        ],
    };

    Ok(Json(response))
}
