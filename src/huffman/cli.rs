use super::*;

pub fn run_cli(args: &[String]) {
    if args.len() != 3 {
        println!("Not enough arguments.");
        print_usage_message();
        return;
    }

    match args[1].as_str() {
        // Compress the file.
        "c" => {
            println!("Compressing '{}'...", args[2]);
            match measure!(compress(&args[2])) {
                Ok(_) => println!("Compressed file saved to '{}.hzip'.", args[2]),
                Err(error) => println!("Failed to compress file: {error}."),
            }
        }
        // Decompress the file.
        "d" => {
            if !args[2].ends_with(".hzip") {
                println!("Not an hzipped file.");
                print_usage_message();
                return;
            }

            println!("Decompressing '{}'...", args[2]);
            match measure!(decompress(&args[2])) {
                Ok(_) => println!(
                    "Decompressed file saved to '{}'.",
                    args[2].replace(".hzip", "")
                ),
                Err(error) => println!("Failed to decompress file: {error}."),
            }
        }
        _ => {
            println!("Unknown option.");
            print_usage_message();
        }
    }
}

fn print_usage_message() {
    println!("Usage for compression: huffman.exe c <path>");
    println!("Usage for decompression (<path> must be to a .hzip file): huffman.exe d <path>");
}
