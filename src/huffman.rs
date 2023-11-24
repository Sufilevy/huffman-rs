use std::collections::HashMap;

use ahash::AHashMap;
use bitvec::vec::BitVec;
use trees::Tree;

mod compress;
mod decompress;

pub use compress::compress;
pub use decompress::decompress;

type CharMap = HashMap<u8, i64>;
type CharTree = Tree<u8>;
type EncodingVec = BitVec<u8>;
type EncodingMap = HashMap<u8, EncodingVec>;

type DecodingMap = AHashMap<EncodingVec, u8>;

macro_rules! measure {
    ($($tokens:tt)*) => {{
        use std::time::Instant;
        let start = Instant::now();
        let result = { $($tokens)* };
        let elapsed = start.elapsed();
        println!("Operation took {}ms.", elapsed.as_millis());
        result
    }};
}

pub(crate) use measure;
