mod cli;
mod core;
mod error;
mod schema;

use axum::{middleware, Router};
use clap::Parser;
use cli::Cmd;
use core::cat;
use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use minijinja::Environment;
use std::env;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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
            // 运行迁移
            let conn = pool.get().await.expect("Failed to get DB connection");
            conn.interact(|conn| {
                conn.run_pending_migrations(MIGRATIONS)
                    .expect("Failed to run migrations");
            })
            .await
            .expect("Failed to run migrations");

            // init template engine and add templates
            let mut jinja_env = Environment::new();
            minijinja_embed::load_templates!(&mut jinja_env);

            let app_state = Arc::new(AppState {
                env: jinja_env,
                pool,
            });
            // build our application with a single route

            let app = Router::new()
                .merge(core::home::routes(app_state.clone()))
                .merge(core::users::register::routes(app_state.clone()))
                .merge(core::users::login::routes(app_state.clone()))
                .nest("/cat", core::cat::web::routes(app_state.clone()))
                .layer(CookieManagerLayer::new());

            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind(cmd.addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
