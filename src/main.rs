use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(
    name = "sudohaxe_cli",
    version = "1.0",
    author = "Sudohaxe",
    about = "Rust sandbox for Sudohaxe"
)]

struct Args {
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello!");
}
