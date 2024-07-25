use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use serde::Deserialize;
use std::convert::From as std_From;
use std::sync::Arc;
use ulid::Ulid;

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
    State(_state): State<Arc<crate::AppState>>,
    Form(input): Form<AddForm>,
) -> Result<Html<String>, StatusCode> {
    println!("{:#?}", input);
    Ok(Html("ok".to_string()))
}
