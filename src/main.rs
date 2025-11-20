use axum::{
    routing::{get, post}, Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handlers;
mod middleware;

use config::Config;

#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_dev_server=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();
    info!("Starting server with config: {:?}", config);

    let state = AppState {
        config: Arc::new(config.clone()),
    };

    // Build router
    let app = create_router(state.clone());

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("🚀 Server listening on http://{}", addr);
    info!("📁 Serving static files from: {}", config.static_dir);
    info!("🔧 Environment: {}", config.environment);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");

    info!("Server shutdown complete");
}

fn create_router(state: AppState) -> Router {
    // API routes
    let api_routes = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/echo", post(handlers::echo))
        .route("/items", get(handlers::list_items))
        .route("/items", post(handlers::create_item))
        .with_state(state.clone());

    // Static file serving
    let static_service = ServeDir::new(&state.config.static_dir)
        .append_index_html_on_directories(true);

    // Main router with middleware
    Router::new()
        .nest("/api", api_routes)
        .fallback_service(static_service)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::request_id::RequestIdLayer::new())
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            warn!("Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            warn!("Received SIGTERM, shutting down gracefully...");
        },
    }
}
