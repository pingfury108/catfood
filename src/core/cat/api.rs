use super::food::Food;
use crate::schema;
use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ulid::Ulid;

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    println!("{:#?}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

pub async fn food_list_handler(
    State(state): State<Arc<crate::AppState>>,
) -> anyhow::Result<Json<Vec<Food>>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;
    let res: Vec<Food> = conn
        .interact(|conn| schema::cat_food::table.select(Food::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodCreateArgs {
    pub title: String,
    pub describe: String,
}

pub async fn food_create_handler(
    State(state): State<Arc<crate::AppState>>,
    Json(args): Json<FoodCreateArgs>,
) -> anyhow::Result<Json<Food>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error)?;
    let res: Food = conn
        .interact(|conn| {
            let new_food = Food {
                gid: {
                    let ulid = Ulid::new();
                    ulid.to_string()
                },
                title: args.title,
                describe: args.describe,
            };
            diesel::insert_into(schema::cat_food::table)
                .values(&new_food)
                .returning(Food::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
