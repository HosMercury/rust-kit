use crate::{
    AppState,
    models::user::User,
    utils::validation::{general_error, validation_errors},
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use validator::Validate;

pub fn users_router() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/login", post(post_login))
            .route("/register", post(post_register))
            .route("/logout", post(logout)),
    )
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

pub async fn post_login(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Json(data): Json<LoginData>,
) -> impl IntoResponse {
    if let Err(errors) = data.validate() {
        return (StatusCode::BAD_REQUEST, Json(validation_errors(errors))).into_response();
    }

    match User::login(&pool, data).await {
        Ok(user) => {
            if let Err(_) = session.insert("auth_user", &user).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(general_error("Session error")),
                )
                    .into_response();
            }

            (StatusCode::OK, Json(user)).into_response()
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(general_error("Invalid Credentials")),
        )
            .into_response(),
    }
}

#[derive(Debug, Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    #[validate(length(min = 2, message = "First name must be at least 2 characters long"))]
    pub first_name: String,

    #[validate(length(min = 2, message = "Last name must be at least 2 characters long"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

pub async fn post_register(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Json(data): Json<RegisterData>,
) -> impl IntoResponse {
    if let Err(errors) = data.validate() {
        return (StatusCode::BAD_REQUEST, Json(validation_errors(errors))).into_response();
    }

    if User::email_exists(&pool, &data.email)
        .await
        .unwrap_or(false)
    {
        return (
            StatusCode::CONFLICT,
            Json(general_error("Email already exists")),
        )
            .into_response();
    }

    match User::register(&pool, data).await {
        Ok(user) => {
            if let Err(_) = session.insert("auth_user", &user).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(general_error("System error")),
                )
                    .into_response();
            }

            (StatusCode::CREATED, Json(user)).into_response()
        }

        Err(e) => {
            println!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(general_error("User registration failed")),
            )
                .into_response()
        }
    }
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
