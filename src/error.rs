use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

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
    InvalidEmail,
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ClientError::LoginFail => (
                StatusCode::UNAUTHORIZED,
                //StatusCode::OK,
                Html(include_str!("../templates/login_fail").to_string()),
            )
                .into_response(),
            ClientError::NoAuth => (
                StatusCode::UNAUTHORIZED,
                Html("<div>未登录</div>".to_string()),
            )
                .into_response(),
            ClientError::UserNameExist => (
                StatusCode::BAD_REQUEST,
                Html(include_str!("../templates/user_exist").to_string()),
            )
                .into_response(),
            ClientError::InvalidEmail => (
                StatusCode::BAD_REQUEST,
                Html(include_str!("../templates/invalid_email").to_string()),
            )
                .into_response(),
        }
    }
}
