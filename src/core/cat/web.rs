use super::food::Food;
use crate::core::cat::food::{food_new, food_one, food_save};
use anyhow::Result;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use serde::Deserialize;
use std::convert::From as std_From;
use std::sync::Arc;
use ulid::Ulid;

pub async fn describe(
    State(state): State<Arc<crate::AppState>>,
    Path(gid): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("cat_food_describe").unwrap();

    let food = food_one(&state.pool, gid).await;
    match food {
        Ok(food) => {
            let rendered = template
                .render(context! {
                    food => food,
                })
                .unwrap();

            Ok(Html(rendered))
        }
        Err(e) => {
            println!("cat web food describe err: {:#?}", e);
            let rendered = template
                .render(context! {
                    food => Food::default(),
                })
                .unwrap();

            Ok(Html(rendered))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddForm {
    title: String,
    describe: String,
    product_name: Option<String>,                // 品名
    brand_owner: Option<String>,                 // 品牌商
    brand_owner_addr: Option<String>,            // 品牌商地址
    manufacturer: Option<String>,                // 生产商
    manufacturer_addr: Option<String>,           // 生产商地址
    manufacturer_license_number: Option<String>, // 生产许可证号
    product_standard: Option<String>,            //产品执行标准
    net_content: Option<String>,                 // 净含量
    reference_price: Option<String>,             // 参考价格
    ingredients: Option<String>,                 // 原料组成
    production_method: Option<String>,           // 制作方式
    additives: Option<String>,                   // 添加剂组成
    guaranteed_analysis: Option<String>,
}

impl std_From<AddForm> for Food {
    fn from(source: AddForm) -> Self {
        Self {
            gid: {
                let id = Ulid::new();
                id.to_string()
            },
            title: source.title,
            describe: source.describe,
            product_name: source.product_name,
            brand_owner: source.brand_owner,
            brand_owner_addr: source.brand_owner_addr,
            manufacturer: source.manufacturer,
            manufacturer_addr: source.manufacturer_addr,
            manufacturer_license_number: source.manufacturer_license_number,
            product_standard: source.product_standard,
            net_content: source.net_content,
            reference_price: source.reference_price,
            ingredients: source.ingredients,
            production_method: source.production_method,
            additives: source.additives,
            guaranteed_analysis: source.guaranteed_analysis,
        }
    }
}

pub async fn add_page(
    State(state): State<Arc<crate::AppState>>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("cat_food_add").unwrap();

    let rendered = template.render(context! {}).unwrap();

    Ok(Html(rendered))
}

pub async fn add_form(
    State(state): State<Arc<crate::AppState>>,
    Form(input): Form<AddForm>,
) -> Result<Html<String>, StatusCode> {
    println!("{:#?}", input);
    let res = food_new(&state.pool, Food::from(input)).await;
    match res {
        Ok(res) => Ok(Html(serde_json::json!(res).to_string())),
        Err(e) => {
            println!("add cat food err: {:#?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateForm {
    gid: String,
    title: String,
    describe: String,
    product_name: Option<String>,                // 品名
    brand_owner: Option<String>,                 // 品牌商
    brand_owner_addr: Option<String>,            // 品牌商地址
    manufacturer: Option<String>,                // 生产商
    manufacturer_addr: Option<String>,           // 生产商地址
    manufacturer_license_number: Option<String>, // 生产许可证号
    product_standard: Option<String>,            //产品执行标准
    net_content: Option<String>,                 // 净含量
    reference_price: Option<String>,             // 参考价格
    ingredients: Option<String>,                 // 原料组成
    production_method: Option<String>,           // 制作方式
    additives: Option<String>,                   // 添加剂组成
    guaranteed_analysis: Option<String>,
}

impl std_From<UpdateForm> for Food {
    fn from(source: UpdateForm) -> Self {
        Self {
            gid: source.gid,
            title: source.title,
            describe: source.describe,
            product_name: source.product_name,
            brand_owner: source.brand_owner,
            brand_owner_addr: source.brand_owner_addr,
            manufacturer: source.manufacturer,
            manufacturer_addr: source.manufacturer_addr,
            manufacturer_license_number: source.manufacturer_license_number,
            product_standard: source.product_standard,
            net_content: source.net_content,
            reference_price: source.reference_price,
            ingredients: source.ingredients,
            production_method: source.production_method,
            additives: source.additives,
            guaranteed_analysis: source.guaranteed_analysis,
        }
    }
}

pub async fn edit_page(
    State(state): State<Arc<crate::AppState>>,
    Path(gid): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("cat_food_edit").unwrap();

    let food = food_one(&state.pool, gid).await;
    match food {
        Ok(food) => {
            let rendered = template
                .render(context! {
                    food => food,
                })
                .unwrap();

            Ok(Html(rendered))
        }
        Err(e) => {
            println!("cat web food edit err: {:#?}", e);
            let rendered = template
                .render(context! {
                    food => Food::default(),
                })
                .unwrap();

            Ok(Html(rendered))
        }
    }
}

pub async fn edit_form(
    State(state): State<Arc<crate::AppState>>,
    Form(input): Form<UpdateForm>,
) -> Result<Html<String>, StatusCode> {
    println!("{:#?}", input);
    let food = Food::from(input);

    let res = food_save(&state.pool, food).await;
    match res {
        Ok(res) => Ok(Html(serde_json::json!(res).to_string())),
        Err(e) => {
            println!("edit cat food err: {:#?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
