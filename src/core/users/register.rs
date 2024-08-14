use super::user::{user_new, User};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, Router},
};
use minijinja::context;
use serde::Deserialize;
use std::convert::From as std_From;
use std::sync::Arc;
use ulid::Ulid;

pub fn routes(state: Arc<crate::AppState>) -> Router {
    Router::new()
        .route("/register", get(register_page).post(register))
        .with_state(state)
}

pub async fn register_page(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("register").unwrap();

    let rendered = template.render(context! {}).unwrap();

    Ok(Html(rendered))
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    name: String,
    pwd: String,
    email: String,
}

impl std_From<RegisterForm> for User {
    fn from(value: RegisterForm) -> Self {
        Self {
            uid: {
                let id = Ulid::new();
                id.to_string()
            },
            name: value.name,
            pwd: value.pwd,
            email: value.email,
        }
    }
}

pub async fn register(
    State(state): State<Arc<crate::AppState>>,
    Form(input): Form<RegisterForm>,
) -> impl IntoResponse {
    if let Ok(u) = super::user::user_one_by_email(&state.pool, input.email.clone()).await {
        log::debug!("u: {:#?}", u);
        if u.uid != *"" {
            return crate::error::ClientError::UserNameExist.into_response();
        }
    };

    if input.email.is_empty() {
        return crate::error::ClientError::InvalidEmail.into_response();
    }
    let mut u = User::from(input);
    match bcrypt::hash(u.pwd.clone(), bcrypt::DEFAULT_COST) {
        Ok(hashed) => u.pwd = hashed,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
    match user_new(&state.pool, u).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("register, save user err: {:#?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
    Html("".to_string()).into_response()
}
