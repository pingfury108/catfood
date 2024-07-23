use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

pub async fn home(State(state): State<Arc<crate::AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            welcome_text => "欢迎来到宠物界的豆瓣!",
        })
        .unwrap();

    Ok(Html(rendered))
}
