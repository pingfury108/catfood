use anyhow::Result;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, Router},
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use minijinja::context;
use serde::Deserialize;
use sha2::Sha256;
use std::sync::Arc;
use std::{collections::BTreeMap, env};
use tower_cookies::{Cookie, Cookies};

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
    email: String,
    pwd: String,
}

pub async fn login(
    State(state): State<Arc<crate::AppState>>,
    cookies: Cookies,
    Json(input): Json<LoginForm>,
) -> impl IntoResponse {
    let u = super::user::user_one_by_email(&state.pool, input.email)
        .await
        .expect("");
    let r = bcrypt::verify(input.pwd, &u.pwd[..]);
    match r {
        Ok(r) => {
            log::debug!("{:#?}", r);
            if r {
                let token_str = sign_token(u.uid);
                match token_str {
                    Ok(t) => {
                        let mut cookie = Cookie::new("token", t);
                        cookie.set_http_only(true);
                        cookie.set_path("/");
                        cookies.add(cookie);
                        return Html("".to_string()).into_response();
                    }
                    Err(e) => {
                        log::error!("login {e}");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Html("<div>jwt err</div>"),
                        )
                            .into_response();
                    }
                }
            } else {
                crate::error::ClientError::LoginFail.into_response()
            }
        }
        Err(e) => {
            log::debug!("login err: {:#?}", e);
            crate::error::ClientError::LoginFail.into_response()
        }
    }
}

pub fn sign_token(uid: String) -> Result<String> {
    let secret = env::var("CATFOOD_JWT_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", uid);
    let token_str = claims.sign_with_key(&key)?;
    Ok(token_str)
}

pub fn verify_token(token_str: String) -> Result<String> {
    let secret = env::var("CATFOOD_JWT_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key)?;
    Ok(claims["sub"].to_string())
}
