#![allow(unused)]

use std::env::{self, args};

mod huffman;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = format!("input/input{}.txt", args[1].parse::<u32>().unwrap());
    measure!({ huffman::compress(&input) });
}

#[macro_export]
macro_rules! measure {
    ($fun:block) => {{
        use std::time::Instant;
        let start = Instant::now();
        let result = $fun;
        let elapsed = start.elapsed();
        println!(
            "{} execution time: {}ms",
            stringify!($fun),
            elapsed.as_millis()
        );
        result
    }};
}
