use bitvec::vec::BitVec;

mod cli;
mod compress;
mod decompress;

pub use cli::run_cli;
use compress::compress;
use decompress::decompress;

type EncodingVec = BitVec<u8>;

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

use measure;
