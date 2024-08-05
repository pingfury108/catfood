use super::food::Food;
use crate::core::cat::food::{food_new, food_one, food_save};
use anyhow::Result;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, Redirect, Response},
    routing::{get, post, Router},
};
use base64::Engine;
use minijinja::context;
use serde::Deserialize;
use std::convert::From as std_From;
use std::sync::Arc;
use ulid::Ulid;

pub fn routes(state: Arc<crate::AppState>) -> Router {
    Router::new()
        .route("/food/:gid", get(describe))
        .route("/food/img/:gid", get(img))
        .route("/food/add", get(add_page).post(add_form))
        .route("/food/edit/:gid", get(edit_page))
        .route("/food/edit", post(edit_form))
        .with_state(state)
}

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

pub async fn img(
    State(state): State<Arc<crate::AppState>>,
    Path(gid): Path<String>,
) -> Response<axum::body::Body> {
    let food = food_one(&state.pool, gid).await;
    match food {
        Ok(food) => match food.img {
            Some(img) => {
                let mut img_data: Vec<u8> = vec![];
                let data = &img[..].split(",").collect::<Vec<&str>>();
                let ftype: String = match data.first() {
                    Some(s) => {
                        let ts = s.split(";").collect::<Vec<&str>>();
                        match ts.first() {
                            Some(s) => {
                                let tts = s.split(":").collect::<Vec<&str>>();
                                match tts.last() {
                                    Some(s) => s.to_string(),
                                    None => {
                                        return Response::builder()
                                            .status(StatusCode::NOT_FOUND)
                                            .body(vec![].into())
                                            .unwrap();
                                    }
                                }
                            }
                            None => {
                                return Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(vec![].into())
                                    .unwrap();
                            }
                        }
                    }
                    None => {
                        println!("img base64 file type");
                        return Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(vec![].into())
                            .unwrap();
                    }
                };
                let img_base64_str = match data.last() {
                    Some(s) => s.to_string().into_bytes(),
                    None => {
                        println!("img base 64 ");
                        return Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(vec![].into())
                            .unwrap();
                    }
                };
                base64::prelude::BASE64_STANDARD
                    .decode_vec(img_base64_str, &mut img_data)
                    .expect("base64 decode");
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", &ftype[..])
                    .body(img_data.into())
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Vec::<u8>::new().into())
                .unwrap(),
        },
        Err(e) => {
            println!("cat web food img err: {:#?}", e);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Vec::<u8>::new().into())
                .unwrap()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddForm {
    title: String,
    describe: String,
    img: Option<String>,                         // 图片
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
            img: source.img,
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
) -> Result<Redirect, StatusCode> {
    let res = food_new(&state.pool, Food::from(input)).await;
    match res {
        Ok(res) => Ok(Redirect::to(&format!("/cat/food/{}", res.gid)[..])),
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
    img: Option<String>,                         // 图片
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
            img: source.img,
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
) -> Result<Redirect, StatusCode> {
    let food = Food::from(input);

    let res = food_save(&state.pool, food.clone()).await;
    match res {
        Ok(_res) => Ok(Redirect::to(&format!("/cat/food/{}", food.gid)[..])),
        Err(e) => {
            println!("edit cat food err: {:#?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
