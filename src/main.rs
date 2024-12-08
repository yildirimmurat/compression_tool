use std::{env, fs::File, io::{self, Read, Write}};
use compression_tool::compression::CompressionTool;
use compression_tool::decompression::DecompressionTool;

fn main() -> io::Result<()> {
    // Get the arguments (input path, compressed output path, and decompressed output path)
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run <input_file> <compressed_file> <decompressed_file>");
        return Ok(());
    }

    let input_file = &args[1];        // Path to the input file
    let compressed_file = &args[2];    // Path to save the compressed file
    let decompressed_file = &args[3];  // Path to save the decompressed file
    
    let mut input_content = String::new();
    let mut input_file = File::open(input_file)?;
    input_file.read_to_string(&mut input_content)?;

    // Step 2: Compress the content and save to the compressed file
    let mut compression_tool = CompressionTool::new(input_content);
    compression_tool.compress(compressed_file).expect("Compression failed");

    // Step 3: Decompress the compressed file and save to the decompressed file
    let decompression_tool = DecompressionTool::new(compressed_file);
    let decompressed_data = decompression_tool.decompress().expect("Decompression failed");

    // Step 4: Save the decompressed data to the output file
    let mut decompressed_file = File::create(decompressed_file)?;
    decompressed_file.write_all(decompressed_data.as_bytes())?;

    println!("Compression and decompression completed successfully!");

    Ok(())
}
