use std::fs;

use ahash::AHashMap;
use anyhow::{anyhow, Context};

use super::EncodingVec;

type DecodingMap = AHashMap<EncodingVec, u8>;

/// Returns `true` if decompression was successful.
pub fn decompress(path: &str) -> anyhow::Result<()> {
    let data = fs::read(path).context("failed to read from input file")?;

    // Parse the file to get the map, the data, and the data length.
    let (map, data, data_len) = parse_file(data)?;
    let mut contents = decode_data(map, data)?;

    // Get rid of redundant bits that were written to the file because of padding.
    while contents.len() > data_len {
        contents.pop();
    }

    // Write the decoded file to disk.
    let decoded_path = path.replace(".hzip", "");
    fs::write(decoded_path, contents).context("failed to write to output file")?;

    Ok(())
}

fn parse_file(data: Vec<u8>) -> anyhow::Result<(DecodingMap, Vec<u8>, usize)> {
    // Get the data length from the start of the file.
    let (data_len, data) = data.split_at(8);
    let data_len = usize::from_le_bytes(
        data_len
            .try_into()
            .context("the data size segment of the file had incorrect length")?,
    );

    // Get the map size from the start of the file.
    let (map_size, data) = data.split_at(8);
    let map_size = usize::from_le_bytes(
        map_size
            .try_into()
            .context("the map size segment of the file had incorrect length")?,
    );

    // Read and parse the map from the file.
    let (map, data) = data.split_at(map_size);
    let map = parse_map(map)?;

    Ok((map, data.to_vec(), data_len))
}

fn parse_map(raw_map: &[u8]) -> anyhow::Result<DecodingMap> {
    // Convert the raw map to a string.
    let map_str =
        String::from_utf8(raw_map.to_vec()).context("failed to convert map bytes to String")?;

    // For each entry (separated by \0) read the char and the encoding vec and add it to the map.
    map_str
        .split('\0')
        .map(|entry_str| {
            let mut chars = entry_str.chars();
            match chars.next() {
                Some(char) => {
                    let encoding_vec = encoding_vec_from_string(chars.as_str());
                    Ok((encoding_vec, char as u8))
                }
                None => Err(anyhow!("an encoding map entry was empty")),
            }
        })
        .collect()
}

fn encoding_vec_from_string(string: &str) -> EncodingVec {
    string.chars().map(|c| c == '1').collect()
}

fn decode_data(map: DecodingMap, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let mut decoded = Vec::new();
    let bits = EncodingVec::from_vec(data);
    let mut current_bits = EncodingVec::new();

    // Get the char with the shortest encode.
    let (shortest_encode, &shortest_char) = map
        .iter()
        .min_by_key(|(k, _)| k.len())
        .context("map encoding is empty")?;
    let shortest_encode_len = shortest_encode.len();

    // If the shortest encode is only 1 bit, we can save some time by checking
    // if the current bit is 1 when we start reading a new encode. Because of the
    // way the binary-tree construction works in the compression, we know that if
    // the shortest encode is 1 bit it has to be the value 1.
    if shortest_encode_len == 1 {
        // For each bit in the file, add it to a bit buffer. If it matches an
        // encode, turn it into the corresponding char and clear the bit buffer.
        for bit in bits {
            if current_bits.is_empty() && bit {
                decoded.push(shortest_char);
                continue;
            }

            current_bits.push(bit);

            if let Some(char) = map.get(&current_bits) {
                decoded.push(*char);
                current_bits.clear();
            }
        }
    } else {
        // For each bit in the file, add it to a bit buffer. If it matches an
        // encode, turn it into the corresponding char and clear the bit buffer.
        for bit in bits {
            current_bits.push(bit);

            // Check if the current bits match an encode only if the current bits buffer is long enough.
            if current_bits.len() >= shortest_encode_len {
                if let Some(char) = map.get(&current_bits) {
                    decoded.push(*char);
                    current_bits.clear();
                }
            }
        }
    }

    Ok(decoded)
}
