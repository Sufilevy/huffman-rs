use std::{collections::HashMap, error::Error};

use bitvec::vec::BitVec;
use trees::Tree;

mod compress;
mod decompress;

pub use compress::compress;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type EmptyResult = Result<()>;

type CharMap = HashMap<u8, i32>;
type CharTree = Tree<u8>;
type EncodingVec = BitVec<u8>;
type EncodingMap = HashMap<u8, EncodingVec>;

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
