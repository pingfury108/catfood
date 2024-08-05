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
        .route("/", get(home))
        .route("/about", get(about))
        .route("/assets/:f", get(super::assets::static_handler))
        .with_state(state)
}

pub async fn home(State(state): State<Arc<crate::AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let dd = match crate::cat::food::food_list(&state.pool).await {
        Ok(dd) => dd,
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    };

    let rendered = template
        .render(context! {
            title => "Home",
            welcome_text => "欢迎来到宠物界的豆瓣!",
            foods => dd,
        })
        .unwrap();

    Ok(Html(rendered))
}

pub async fn about(State(state): State<Arc<crate::AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("about").unwrap();

    let rendered = template
        .render(context! {
            title => "About",
            about_text => "这里是 CotFood 宠物界的豆瓣",
        })
        .unwrap();

    Ok(Html(rendered))
}
