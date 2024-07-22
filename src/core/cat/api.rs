use super::food::Food;
use super::schema::cat_food::dsl::*;
use axum::response::Html;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub async fn food_list() -> Html<&'static str> {
    let conn = SqliteConnection::establish("catfood.db");
    match conn {
        Ok(mut con) => {
            let food = cat_food.load::<Food>(&mut con);
            println!("{:#?}", food);
        }
        Err(e) => {
            println!("{e:#?}");
        }
    }
    Html("<h1>hello api cat food get</h1>")
}
