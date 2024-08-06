use anyhow::Result;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, Router},
};
use minijinja::context;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    name: String,
    pwd: String,
}

pub async fn login(
    State(state): State<Arc<crate::AppState>>,
    Form(input): Form<LoginForm>,
) -> impl IntoResponse {
    let u = super::user::user_one_by_name(&state.pool, input.name)
        .await
        .expect("");
    let r = bcrypt::verify(input.pwd, &u.pwd[..]);
    match r {
        Ok(r) => {
            println!("{:#?}", r);
            if r {
                Redirect::to("/").into_response()
            } else {
                crate::error::ClientError::LoginFail.into_response()
            }
        }
        Err(e) => {
            println!("login err: {:#?}", e);
            crate::error::ClientError::LoginFail.into_response()
        }
    }
}
