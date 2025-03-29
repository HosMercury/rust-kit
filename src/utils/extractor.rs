use crate::models::user::User;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use tower_sessions::Session;

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<Session>()
            .expect("from req parts getting user err from session")
            .get::<User>("auth_user")
            .await
            .expect("Getting user from request part");

        match user {
            Some(user) => Ok(user),
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
