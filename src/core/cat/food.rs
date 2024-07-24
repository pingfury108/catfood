use crate::schema;
use anyhow::Result;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::{Insertable, QueryDsl, Queryable, Selectable};
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::cat_food)]
pub struct Food {
    pub gid: String,
    pub title: String,
    pub describe: String,
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
