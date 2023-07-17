use std::env;

use crate::huffman::measure;

mod huffman;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Not enough arguments.");
        print_usage_message();
        return;
    }

    match args[1].as_str() {
        "c" => {
            // Compress the file.
            measure! {
                println!("Compressing '{}'...", args[2]);
                huffman::compress(&args[2]);
                println!("Compressed file saved to '{}.hzip'.", args[2]);
            };
        }
        "d" => {
            // Decompress the file.
            if args[2].ends_with(".hzip") {
                measure! {
                    println!("Decompressing '{}'...", args[2]);
                    huffman::decompress(&args[2]);
                    println!(
                        "Decompressed file saved to '{}'.",
                        args[2].replace(".hzip", "")
                    );
                }
            } else {
                println!("Not an hzipped file.");
                print_usage_message();
            }
        }
        _ => {
            println!("Unknown usage.");
            print_usage_message();
        }
    }
}

fn print_usage_message() {
    println!("Usage for compression: huffman-rs.exe c <path>");
    println!("Usage for decompression (path must be a .hzip file): huffman-rs.exe d <path>");
}
