use std::fs;

use super::*;

pub fn compress(path: &str) -> EmptyResult {
    let data = fs::read(path)?;

    Ok(())
}
