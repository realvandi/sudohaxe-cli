use clap::Parser;

mod utils;

/// Sudohaxe-CLI
#[derive(Parser, Debug)]
#[command(name = "sudohaxe_cli", version = "1.0", author = "Sudohaxe", about = "Echoes the input provided by the user")]
struct Args {
    input: String,
}

fn main() {
    let args = Args::parse();

    let _hello_world = "welcome to Sudohaxe-CLI"; 
    let _hello_world_capitalized = utils::capitalize(_hello_world);

    let capitalized_input = utils::capitalize(&args.input);

    println!("Hello, {} {}", capitalized_input, _hello_world_capitalized);
}
