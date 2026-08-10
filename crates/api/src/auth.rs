use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
}

/// Extract token from Authorization header
pub fn extract_token(headers: &axum::http::header::HeaderMap) -> Result<String, (StatusCode, String)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?;

    auth_header
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization format".to_string()))
}

pub async fn register(
    State(state): State<crate::AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let auth_service = &state.auth;
    let user = lievre_core::user::LoginUser {
        email: req.email,
        username: Some(req.username),
        password: req.password.clone(),
    };

    let user = auth_service
        .register(user, &req.password)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let login_user = lievre_core::user::LoginUser {
        email: user.email.clone(),
        username: Some(user.username.clone()),
        password: req.password.clone(),
    };

    let token = auth_service
        .login(login_user, &req.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
        },
    }))
}

pub async fn login(
    State(state): State<crate::AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let auth_service = &state.auth;
    let user = lievre_core::user::LoginUser {
        email: req.email,
        username: None,
        password: req.password.clone(),
    };

    let token = auth_service
        .login(user, &req.password)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let user = auth_service
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
        },
    }))
}

pub async fn get_current_user(
    State(state): State<crate::AppState>,
    headers: axum::http::header::HeaderMap,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let auth_service = &state.auth;
    let token = extract_token(&headers)?;
    let user = auth_service
        .verify_token(&token)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
    }))
}
