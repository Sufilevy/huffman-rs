use std::env;

use huffman::EmptyResult;

mod huffman;

fn main() -> EmptyResult {
    let args: Vec<String> = env::args().collect();
    let input = format!("input/input{}.txt", args[1].parse::<u32>().unwrap());
    measure!({ huffman::compress(&input) })?;
    Ok(())
}
