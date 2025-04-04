mod handlers;
mod models;
mod utils;

use dotenvy::dotenv;
use handlers::auth_handler::auth_router;
use handlers::users_handler::users_router;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_sessions::cookie::time::Duration;
use tower_sessions::session_store::ExpiredDeletion;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Error connecting to database");

    //// session
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60 * 60 * 24)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60 * 60 * 24))); // day

    let app_state = AppState { pool };

    let cors = CorsLayer::very_permissive();

    let app = users_router()
        .merge(auth_router())
        .layer(session_layer)
        .layer(cors)
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    deletion_task.await??;

    Ok(())
}
