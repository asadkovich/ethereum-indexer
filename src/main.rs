mod db;
mod service;
mod repository;
mod rpc;
mod entities;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    dsn: String,
}

fn main() {
    println!("Hello, world!");
}
