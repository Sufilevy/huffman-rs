use priority_queue::PriorityQueue;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    sync::{Arc, Mutex},
};
use trees::{Node, Tree};

use crate::measure;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type EmptyResult = Result<()>;

type CharMap = HashMap<u8, i32>;
type CharTree = Tree<u8>;
type EncodingMap = HashMap<u8, String>;

pub fn compress(path: &str) -> EmptyResult {
    let data = fs::read(path)?;

    let char_map = measure!({ create_char_map(data) });
    if char_map.is_empty() {
        return Ok(());
    }

    let tree = create_char_tree(char_map);
    let encoding_map = create_encoding_map(tree);

    Ok(())
}

fn create_char_map(data: Vec<u8>) -> CharMap {
    let mut map = CharMap::new();

    for char in data {
        let count = map.entry(char).or_insert(0);
        *count += 1;
    }

    map
}

fn create_char_tree(count_map: CharMap) -> CharTree {
    let mut queue = PriorityQueue::new();
    for (char, count) in count_map {
        queue.push(Tree::new(char), count);
    }

    while queue.len() > 1 {
        let first = queue
            .pop()
            .expect("The priority queue was empty on first pop");
        let second = queue
            .pop()
            .expect("The priority queue was empty on second pop");

        let mut new_node = Tree::new(0);
        new_node.push_back(first.0);
        new_node.push_back(second.0);
        queue.push(new_node, first.1 + second.1);
    }

    queue
        .pop()
        .expect("The priority queue was empty on last pop")
        .0
}

fn create_encoding_map(tree: CharTree) -> EncodingMap {
    let encoding_map = EncodingMap::new();

    encoding_map
}
