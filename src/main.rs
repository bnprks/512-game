use std::env;
use std::fs::File;

use puzzle512::{optimal_strategy, save_strategy};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --release [strategy path]");
        return;
    }
    let strategy = optimal_strategy();
    save_strategy(
        &strategy,
        File::create(&args[1]).expect("Could not create file"),
    );
}
