use crate::schema;
use anyhow::Result;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::{Insertable, QueryDsl, Queryable, Selectable};
use diesel::{AsChangeset, ExpressionMethods, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Queryable, Selectable, Serialize, Deserialize, Insertable, Clone, AsChangeset,
)]
#[diesel(table_name = schema::cat_food)]
pub struct Food {
    pub gid: String,
    pub title: String,
    pub describe: String,
    pub img: Option<String>,                         // 图片
    pub product_name: Option<String>,                // 品名
    pub brand_owner: Option<String>,                 // 品牌商
    pub brand_owner_addr: Option<String>,            // 品牌商地址
    pub manufacturer: Option<String>,                // 生产商
    pub manufacturer_addr: Option<String>,           // 生产商地址
    pub manufacturer_license_number: Option<String>, // 生产许可证号
    pub product_standard: Option<String>,            //产品执行标准
    pub net_content: Option<String>,                 // 净含量
    pub reference_price: Option<String>,             // 参考价格
    pub ingredients: Option<String>,                 // 原料组成
    pub production_method: Option<String>,           // 制作方式
    pub additives: Option<String>,                   // 添加剂组成
    pub guaranteed_analysis: Option<String>,         // 产品成分分析保证值
}

pub async fn food_list(pool: &Pool) -> Result<Vec<Food>, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res = conn
        .interact(|conn| schema::cat_food::table.select(Food::as_select()).load(conn))
        .await??;
    Ok(res)
}

pub async fn food_one(pool: &Pool, id: String) -> Result<Food, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res: Vec<Food> = conn
        .interact(move |conn| {
            use schema::cat_food::dsl::*;
            cat_food.filter(gid.eq(&id[..])).load(conn)
        })
        .await??;
    if !res.is_empty() {
        return Ok(res[0].clone());
    }

    Ok(Food::default())
}

pub async fn food_new(pool: &Pool, food: Food) -> Result<Food, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res: Food = conn
        .interact(move |conn| {
            diesel::insert_into(schema::cat_food::table)
                .values(food)
                .returning(Food::as_returning())
                .get_result(conn)
        })
        .await??;
    Ok(res)
}

pub async fn food_save(pool: &Pool, food: Food) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let _ = conn
        .interact(move |conn| {
            use schema::cat_food::dsl::*;
            diesel::update(schema::cat_food::table)
                .filter(gid.eq(&food.gid.clone()[..]))
                .set(food)
                .execute(conn)
        })
        .await??;
    Ok(())
}
