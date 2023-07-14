#![allow(unused)]

mod huffman;

fn main() {
    measure!({ huffman::compress("input/input4.txt") });
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
