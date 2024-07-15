use clap::Parser;

/// Echo CLI
#[derive(Parser, Debug)]
#[command(name = "sudohaxe_cli", version = "1.0", author = "Sudohaxe", about = "Echoes the input provided by the user")]
struct Args {
    input: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}", args.input);
}
