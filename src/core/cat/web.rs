use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

use super::food::Food;

pub async fn describe(
    State(state): State<Arc<crate::AppState>>,
    Path(gid): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("cat_food_describe").unwrap();

    let rendered = template
        .render(context! {
            food => Food{
                gid,
                title: "cccc".to_string(),
                describe: "bbb".to_string(),
                ..Food::default()
            },
        })
        .unwrap();

    Ok(Html(rendered))
}

pub async fn add(State(state): State<Arc<crate::AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("cat_food_add").unwrap();

    let rendered = template
        .render(context! {
            food => Food{
                gid: "to".to_string(),
                title:"cccc".to_string(),
                describe: "bbb".to_string(),
                ..Food::default()
            },
        })
        .unwrap();

    Ok(Html(rendered))
}
