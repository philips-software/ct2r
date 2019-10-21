use std::env;

mod engine;
mod vendors;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} tool json_file", args[0]);
        println!("");
        println!("  tool:");
        println!("    - xray");
        println!("    - gradle");
        return;
    }

    let tool = ::std::env::args().nth(1).unwrap();
    let input_path = ::std::env::args().nth(2).unwrap();

    engine::program::start(&tool, &input_path).unwrap();
}

