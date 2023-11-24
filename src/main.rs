use std::env;

mod huffman;

fn main() {
    let args: Vec<String> = env::args().collect();
    huffman::run_cli(&args);
}
