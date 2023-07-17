use std::fs;

use super::*;

pub fn decompress(path: &str) {
    let data = fs::read(path).expect("failed to read from input file");

    // Parse the file to get the map, the data, and the data length.
    let (map, data, data_len) = parse_file(data);
    let mut contents = decode_data(map, data);

    // Get rid of redundant bits that were written to the file because of padding.
    while contents.len() > data_len {
        contents.pop();
    }

    // Write the decoded file to disk.
    let decoded_path = path.replace(".hzip", "");
    fs::write(decoded_path, contents).expect("failed to write to output file.");
}

fn parse_file(data: Vec<u8>) -> (DecodingMap, Vec<u8>, usize) {
    // Get the data length from the start of the file.
    let (data_len, data) = data.split_at(8);
    let data_len = usize::from_le_bytes(
        data_len
            .try_into()
            .expect("data length slice with incorrect length"),
    );

    // Get the map size from the start of the file.
    let (map_size, data) = data.split_at(8);
    let map_size = usize::from_le_bytes(
        map_size
            .try_into()
            .expect("map size slice with incorrect length"),
    );

    // Read and parse the map from the file.
    let (map, data) = data.split_at(map_size);
    let map = parse_map(map);

    (map, data.to_vec(), data_len)
}

fn parse_map(raw_map: &[u8]) -> DecodingMap {
    // Convert the raw map to a string.
    let map_str =
        String::from_utf8(raw_map.to_vec()).expect("failed to convert map bytes to String");

    // For each entry (separated by \0) read the char and the encoding vec and add it to the map.
    map_str
        .split('\0')
        .map(|entry_str| {
            let mut chars = entry_str.chars();
            let char = chars.next().expect("an encoding map entry was empty");
            let encoding_vec = encoding_vec_from_string(chars.as_str());

            (encoding_vec, char as u8)
        })
        .collect()
}

fn encoding_vec_from_string(string: &str) -> EncodingVec {
    string.chars().map(|c| c == '1').collect()
}

fn decode_data(map: DecodingMap, data: Vec<u8>) -> Vec<u8> {
    let mut decoded = Vec::new();
    let bits = EncodingVec::from_vec(data);
    let mut current_bits = EncodingVec::new();

    // For each bit in the file, add it to a bit buffer. If it matches an
    // encoding-string, turn it into the corresponding char and clear the bit buffer.
    for bit in bits {
        current_bits.push(bit);

        if let Some(&char) = map.get(&current_bits) {
            decoded.push(char);
            current_bits.clear();
        }
    }

    decoded
}
