mod activities;
mod auth;
mod federation;
mod feed;
mod geojson;
mod import;
mod notifications;
mod social;
mod worker;
pub mod ws;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use lievre_core::{
    ActivityRepository, AuthService, JobRepository, NotificationRepository, RouteRepository,
    SocialRepository, UserRepository,
};
use lievre_federation::config::FederationDb;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::ws::ConnectionManager;

/// Combined application state
#[derive(Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub activity_repo: ActivityRepository,
    pub route_repo: RouteRepository,
    pub social: SocialRepository,
    pub notification_repo: NotificationRepository,
    pub job_repo: JobRepository,
    pub fed_db: FederationDb,
    pub ws_manager: ConnectionManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lievre=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/lievre.db".to_string());
    let pool = lievre_shared::db::create_pool(&database_url).await?;
    lievre_shared::db::run_migrations(&pool).await?;

    // Initialize repositories
    let activity_repo = ActivityRepository::new(pool.clone());
    let route_repo = RouteRepository::new(pool.clone());
    let user_repo = UserRepository::new(pool.clone());

    // Initialize auth service
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-secret-change-in-production".to_string());
    let auth_service = AuthService::new(user_repo, jwt_secret);

    // Social repository
    let social = SocialRepository::new(pool.clone());

    // Notification repository
    let notification_repo = NotificationRepository::new(pool.clone());

    // Job repository
    let job_repo = JobRepository::new(pool.clone());

    // Federation database
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let scheme = std::env::var("SCHEME").unwrap_or_else(|_| "http".to_string());
    let fed_db = FederationDb::new(pool.clone(), domain, scheme);

    // Combined state
    let state = AppState {
        auth: auth_service,
        activity_repo,
        route_repo,
        social,
        notification_repo,
        job_repo,
        fed_db,
        ws_manager: ConnectionManager::new(),
    };

    // Build router
    let app = Router::new()
        // Health
        .route("/health", get(health))
        // Auth
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/users/me", get(auth::get_current_user))
        .route("/api/users/:id", get(auth::get_user))
        .route("/api/users", get(auth::list_users))
        // Activities CRUD
        .route("/api/activities", post(activities::create_activity))
        .route("/api/activities", get(activities::list_activities))
        .route(
            "/api/users/:id/activities",
            get(activities::list_user_activities),
        )
        .route("/api/activities/:id", get(activities::get_activity))
        .route("/api/activities/:id", delete(activities::delete_activity))
        .route("/api/activities/:id", put(activities::update_activity))
        .route(
            "/api/activities/:id/geojson",
            get(geojson::get_activity_geojson),
        )
        // Social
        .route("/api/users/:id/follow", post(social::follow_user))
        .route("/api/users/:id/follow", delete(social::unfollow_user))
        .route("/api/users/:id/follow-status", get(social::follow_status))
        .route("/api/users/:id/followers", get(social::get_followers))
        .route("/api/users/:id/following", get(social::get_following))
        .route("/api/activities/:id/like", post(social::like_activity))
        .route("/api/activities/:id/like", delete(social::unlike_activity))
        .route("/api/activities/:id/likes", get(social::get_likes))
        .route("/api/activities/:id/comments", get(social::get_comments))
        .route("/api/activities/:id/comments", post(social::add_comment))
        .route("/api/comments/:id", delete(social::delete_comment))
        // Feed
        .route("/api/feed", get(feed::personal_feed))
        .route("/api/feed/public", get(feed::public_feed))
        // Notifications
        .route("/api/notifications", get(notifications::list_notifications))
        .route("/api/notifications/:id/read", put(notifications::mark_read))
        .route(
            "/api/notifications/read-all",
            put(notifications::mark_all_read),
        )
        .route(
            "/api/notifications/:id",
            delete(notifications::delete_notification),
        )
        // Federation
        .route("/.well-known/webfinger", get(federation::webfinger))
        .route("/users/:username", get(federation::actor))
        .route("/users/:username/inbox", post(federation::inbox))
        .route("/users/:username/outbox", get(federation::outbox))
        .route("/api/exercises/:id/route", get(federation::exercise_route))
        .route("/api/exercises/:id/stats", get(federation::exercise_stats))
        .route(
            "/ns/fedisport",
            get(lievre_federation::context::fedisport_context),
        )
        // Import
        .route("/api/import/gpx", post(import::import_gpx))
        .route("/api/import/tcx", post(import::import_tcx))
        .route("/api/import/strava", post(import::import_strava))
        // WebSocket
        .route("/ws", get(ws_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port)).await?;

    tracing::info!("Starting Lièvre on {}:{}", addr, port);

    // Spawn background worker
    let worker_pool = pool.clone();
    tokio::spawn(async move {
        worker::run_worker(worker_pool, 5000).await;
    });

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "OK"
}

/// WebSocket upgrade handler
///
/// GET /ws?token=<jwt>
/// Upgrades to WebSocket and registers the connection for real-time notifications.
async fn ws_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    // Extract token from query params
    let token = match params.get("token") {
        Some(t) => t.clone(),
        None => {
            return axum::response::Response::builder()
                .status(401)
                .body("Missing token".into())
                .unwrap();
        }
    };

    // Verify token and get user_id
    let user_id = match state.auth.verify_token(&token).await {
        Ok(user) => user.id,
        Err(_) => {
            return axum::response::Response::builder()
                .status(401)
                .body("Invalid token".into())
                .unwrap();
        }
    };

    let manager = state.ws_manager.clone();

    ws.on_upgrade(move |socket| ws::handle_socket(socket, user_id, manager))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = Router::new().route("/health", get(health));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body, "OK");
    }
}
