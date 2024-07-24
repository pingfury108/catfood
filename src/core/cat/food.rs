use crate::schema;
use anyhow::Result;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::{Insertable, QueryDsl, Queryable, Selectable};
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::cat_food)]
pub struct Food {
    pub gid: String,
    pub title: String,
    pub describe: String,
}

pub async fn food_list(pool: &Pool) -> Result<Vec<Food>, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res = conn
        .interact(|conn| schema::cat_food::table.select(Food::as_select()).load(conn))
        .await??;
    Ok(res)
}
