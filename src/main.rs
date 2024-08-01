mod cli;
mod core;
mod schema;

use axum::{routing::get, routing::post, Router};
use clap::Parser;
use cli::Cmd;
use core::{cat, home};
use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use minijinja::Environment;
use std::env;
use std::sync::Arc;

struct AppState {
    env: Environment<'static>,
    pool: Pool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cmd = Cmd::parse();
    env_logger::init();
    println!("Hello, world!");
    match cmd.command {
        Some(cli::Commands::H {}) => {
            println!("run h cmd!");
        }
        None => {
            println!("run main cmd!, {}", cmd.addr);
            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            // set up connection pool
            let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
            let pool = Pool::builder(manager).build().unwrap();

            // init template engine and add templates
            let mut jinja_env = Environment::new();
            {
                let tmps = vec![
                    ("layout", include_str!("../templates/layout.jinja")),
                    ("home", include_str!("../templates/home.jinja")),
                    ("about", include_str!("../templates/about.jinja")),
                    (
                        "cat_food_describe",
                        include_str!("../templates/cat_food_describe.jinja"),
                    ),
                    (
                        "cat_food_add",
                        include_str!("../templates/cat_food_add.jinja"),
                    ),
                    (
                        "cat_food_edit",
                        include_str!("../templates/cat_food_edit.jinja"),
                    ),
                ];
                for (name, file) in tmps {
                    jinja_env.add_template(name, file).unwrap();
                }
            }

            let app_state = Arc::new(AppState {
                env: jinja_env,
                pool,
            });
            // build our application with a single route
            let app = Router::new()
                .route("/", get(home::home))
                .route("/about", get(home::about))
                .route("/assets/:f", get(core::assets::assets_handler))
                .route(
                    "/api/cat/food",
                    get(cat::api::food_list_handler).post(cat::api::food_create_handler),
                )
                .route("/cat/food/:gid", get(cat::web::describe))
                .route("/cat/food/img/:gid", get(cat::web::img))
                .route(
                    "/cat/food/add",
                    get(cat::web::add_page).post(cat::web::add_form),
                )
                .route("/cat/food/edit/:gid", get(cat::web::edit_page))
                .route("/cat/food/edit", post(cat::web::edit_form))
                .with_state(app_state);

            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind(cmd.addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
