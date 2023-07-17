use bitvec::vec::BitVec;
use priority_queue::PriorityQueue;
use rayon::prelude::*;
use std::{collections::HashMap, fs};
use trees::Tree;

use super::*;

pub fn compress(path: &str) -> bool {
    let data = fs::read(path).expect("failed to read from input file");

    // Creating the char map, containing the number of occurrences of each char in the file.
    let char_map = create_char_map(&data);
    if char_map.is_empty() {
        println!("The input file is empty.");
        return false;
    }

    // Creating the char tree, describing how each char should be encoded
    // by choosing the shortest encoded string for the most used chars.
    let char_tree = create_char_tree(char_map);

    // Creating the encoding map, containing the encoded strings of each char in the file.
    let encoding_map = create_encoding_map(char_tree);

    // Encoding the file and writing the results to disk.
    let contents = encode_data(&data, &encoding_map);
    write_to_file(path, encoding_map, &contents, data.len());

    true
}

fn create_char_map(data: &[u8]) -> CharMap {
    data.par_chunks(1_000_000)
        .flat_map(|chunk| {
            // Count the number of occurrences of each char in the chunk.
            let mut map = CharMap::new();
            for &char in chunk {
                let count = map.entry(char).or_insert(-1);
                *count -= 1;
            }
            map
        })
        .collect()
}

fn create_char_tree(count_map: CharMap) -> CharTree {
    // Create the initial tree, with all of the chars in a tree
    // with the priority as the number of occurrences.
    let mut queue = PriorityQueue::new();
    for (char, count) in count_map {
        queue.push(Tree::new(char), count);
    }

    // While there are nodes in the queue, get the nodes with the most
    // priority and make them the children of a new node. Then add that node
    // to the queue with the priority of both child nodes combined.
    while queue.len() > 1 {
        let first = queue
            .pop()
            .expect("the priority queue was empty on first pop");
        let second = queue
            .pop()
            .expect("the priority queue was empty on second pop");

        let mut new_node = Tree::new(0);
        new_node.push_back(first.0);
        new_node.push_back(second.0);
        queue.push(new_node, first.1 + second.1);
    }

    // Pop the last element in the queue, which should be the root of the tree.
    queue
        .pop()
        .expect("the priority queue was empty on last pop")
        .0
}

fn create_encoding_map(tree: CharTree) -> EncodingMap {
    // Create the encoding map recursively.
    rec_create_encoding_map(tree, BitVec::new())
}

fn rec_create_encoding_map(mut tree: CharTree, mut encoding: EncodingVec) -> EncodingMap {
    // If this is not a leaf node, pop it's left and right children and continue creating the map from them.
    if let Some(left) = tree.pop_front() {
        encoding.push(false);
        let mut map = rec_create_encoding_map(left, encoding.clone());
        encoding.pop();

        if let Some(right) = tree.pop_front() {
            encoding.push(true);
            let new_map = rec_create_encoding_map(right, encoding.clone());
            map.extend(new_map);
            encoding.pop();
        }

        map
    } else {
        // If this is a leaf node, create a new map with this char's encoding.
        let mut map = HashMap::new();
        map.insert(*tree.data(), encoding);
        map
    }
}

fn encode_data(data: &[u8], map: &EncodingMap) -> EncodingVec {
    let mut contents = EncodingVec::new();

    for char in data {
        match map.get(char) {
            Some(encoded) => contents.extend(encoded),
            None => panic!("missing encoding for char"),
        }
    }

    contents
}

fn write_to_file(path: &str, map: EncodingMap, contents: &EncodingVec, contents_len: usize) {
    let map_string = encoding_map_to_string(map);
    let map_string_len = map_string.len().to_le_bytes();

    // Write the encoded data to the file, together with the data length,
    // the encoding map length, and the encoding map itself.
    fs::write(
        path.to_owned() + ".hzip",
        [
            &contents_len.to_le_bytes(),
            &map_string_len,
            map_string.as_bytes(),
            contents.as_raw_slice(),
        ]
        .concat(),
    )
    .expect("failed to write to output file");
}

fn encoding_map_to_string(map: EncodingMap) -> String {
    let mut string = String::new();

    // Convert the encoding map to a string, using \0 as the separator.
    for (char, encoding) in map {
        let encoding_str: String = encoding
            .iter()
            .map(|b| if *b { "1" } else { "0" })
            .collect();
        string += &format!("{}{}\0", char as char, encoding_str);
    }
    string.pop();

    string
}
