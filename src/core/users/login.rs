use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, Router},
};

use minijinja::context;
use std::sync::Arc;

pub fn routes(state: Arc<crate::AppState>) -> Router {
    Router::new()
        .route("/login", get(login_page).post(login))
        .with_state(state)
}

pub async fn login_page(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("login").unwrap();

    let rendered = template.render(context! {}).unwrap();

    Ok(Html(rendered))
}

pub async fn login(State(_state): State<Arc<crate::AppState>>) -> Result<Html<String>, StatusCode> {
    Ok(Html("ok".into()))
}
