use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use log;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    log::error!("{:#?}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

#[derive(Debug)]
pub enum ClientError {
    LoginFail,
    NoAuth,
    UserNameExist,
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ClientError::LoginFail => (
                StatusCode::UNAUTHORIZED,
                Html("<div>登录失败</div>".to_string()),
            )
                .into_response(),
            ClientError::NoAuth => (
                StatusCode::UNAUTHORIZED,
                Html("<div>未登录</div>".to_string()),
            )
                .into_response(),
            ClientError::UserNameExist => (
                StatusCode::BAD_REQUEST,
                Html("<div>用户名已存在</div>".to_string()),
            )
                .into_response(),
        }
    }
}
