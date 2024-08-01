use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn assets_handler(
    State(_state): State<Arc<crate::AppState>>,
    Path(f): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let mut assets: HashMap<String, &str> = HashMap::new();
    assets.insert(
        "tailwindcss.v3.4.5.js".to_string(),
        include_str!("../../assets/tailwindcss.v3.4.5.js"),
    );
    assets.insert(
        "htmx.2.0.1.min.js".to_string(),
        include_str!("../../assets/htmx.2.0.1.min.js"),
    );
    assets.insert(
        "browser-image-compression.js".to_string(),
        include_str!("../../assets/browser-image-compression.js"),
    );

    let content = assets.get(&f[..]);
    match content {
        Some(content) => Ok(Html(content.to_string())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
