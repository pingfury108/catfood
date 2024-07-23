mod cli;
mod core;
mod schema;

use axum::{routing::get, Router};
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
            jinja_env
                .add_template("layout", include_str!("../templates/layout.jinja"))
                .unwrap();
            jinja_env
                .add_template("home", include_str!("../templates/home.jinja"))
                .unwrap();

            let app_state = Arc::new(AppState {
                env: jinja_env,
                pool,
            });
            // build our application with a single route
            let app = Router::new()
                .route("/", get(home::home))
                .route(
                    "/api/cat/food",
                    get(cat::api::food_list).post(cat::api::food_create),
                )
                .with_state(app_state);

            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind(cmd.addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
