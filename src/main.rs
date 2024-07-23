mod cli;
mod core;

use axum::{routing::get, Router};
use clap::Parser;
use cli::Cmd;
use core::cat;
use dotenvy::dotenv;
use std::env;

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
            let manager =
                deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
            let pool = deadpool_diesel::postgres::Pool::builder(manager)
                .build()
                .unwrap();

            // build our application with a single route
            let app = Router::new()
                .route("/", get(|| async { "Hello, World!" }))
                .route(
                    "/api/cat/food",
                    get(cat::api::food_list).post(cat::api::food_create),
                )
                .with_state(pool);

            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind(cmd.addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
