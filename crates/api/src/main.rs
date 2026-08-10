mod geojson;
mod import;

use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use lievre_core::{ActivityRepository, RouteRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "lievre=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/lievre.db".to_string());
    let pool = lievre_shared::db::create_pool(&database_url).await?;
    lievre_shared::db::run_migrations(&pool).await?;

    // Initialize repositories
    let activity_repo = ActivityRepository::new(pool.clone());
    let route_repo = RouteRepository::new(pool.clone());

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/import/gpx", post(import::import_gpx))
        .route("/api/activities/:id/geojson", get(geojson::get_activity_geojson))
        .layer(TraceLayer::new_for_http())
        .with_state((activity_repo, route_repo));

    // Start server
    let addr = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port)).await?;

    tracing::info!("Starting Lièvre on {}:{}", addr, port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "OK"
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

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body, "OK");
    }
}
