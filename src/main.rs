mod cli;
mod core;
use axum::{routing::get, Router};
use clap::Parser;
use cli::Cmd;
#[tokio::main]
async fn main() {
    let cmd = Cmd::parse();
    env_logger::init();
    println!("Hello, world!");
    match cmd.command {
        Some(cli::Commands::H {}) => {
            println!("run h cmd!");
        }
        None => {
            println!("run main cmd!, {}", cmd.addr);
            // build our application with a single route
            let app = Router::new().route("/", get(|| async { "Hello, World!" }));

            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind(cmd.addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
