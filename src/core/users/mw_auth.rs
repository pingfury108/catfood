use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{extract::Request, response::Redirect};
use log::debug;
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request, next: Next) -> Response {
    let token = cookies.get("token").map(|c| c.value().to_string());
    match token {
        Some(t) => {
            let uid = super::login::verify_token(t);
            match uid {
                Ok(uid) => {
                    debug!("session uid: {uid}")
                }
                Err(e) => {
                    log::error!("session token invalid: {e}");
                    return Redirect::to("/login").into_response();
                }
            }
        }
        None => {
            debug!("no token");
            return Redirect::to("/login").into_response();
        }
    }
    next.run(req).await
}
