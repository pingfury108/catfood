use super::user::{user_new, User};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Html,
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
    display_name: String,
    email: Option<String>,
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
            display_name: value.display_name,
            email: value.email,
        }
    }
}

pub async fn register(
    State(state): State<Arc<crate::AppState>>,
    Form(input): Form<RegisterForm>,
) -> Result<Html<String>, StatusCode> {
    user_new(&state.pool, User::from(input))
        .await
        .expect("????");
    Ok(Html("ok".into()))
}
