use std::env;

mod engine;
mod vendors;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} text_file", args[0]);
        return;
    }

    let input_path = ::std::env::args().nth(1).unwrap();

    engine::program::start(&input_path).unwrap();
}

