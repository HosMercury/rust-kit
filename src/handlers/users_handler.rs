use crate::{AppState, models::user::User, utils::validation::general_error};
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use tower_sessions::Session;

pub fn users_router() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/me", get(me))
            .route("/logout", post(logout)),
    )
}

async fn me(user: User) -> impl IntoResponse {
    (StatusCode::OK, Json(user))
}

pub async fn logout(session: Session) -> impl IntoResponse {
    if let Err(_) = session.flush().await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(general_error("Logout failed")),
        )
            .into_response();
    }

    (StatusCode::NO_CONTENT).into_response()
}
