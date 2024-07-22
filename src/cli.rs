use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cmd {
    #[arg(short, long, default_value_t =  String::from("0.0.0.0:3000"))]
    pub addr: String,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    H {},
}
