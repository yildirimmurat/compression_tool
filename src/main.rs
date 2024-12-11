use std::{fs::File, io::{self, Write}};
use std::io::{BufReader, BufWriter};
use compression_tool::compression::CompressionTool;
use compression_tool::decompression::DecompressionTool;

fn main() -> io::Result<()> {
    // Get the arguments (input path, compressed output path, and decompressed output path)
    //let args: Vec<String> = env::args().collect();
    let arr: [String; 4] = [        
        String::from("-"),
        String::from("hello.txt"),
        String::from("hello.dat"),
        String::from("hello2.txt"),
    ];
    let args: Vec<String> = Vec::from(arr);
    if args.len() != 4 {
        println!("Usage: cargo run <input_file> <compressed_file> <decompressed_file>");
        return Ok(());
    }

    let input_file_path = &args[1];        // Path to the input file
    let compressed_file_path = &args[2];    // Path to save the compressed file
    let decompressed_file_path = &args[3];  // Path to save the decompressed file
    
    let input_file = File::open(input_file_path)?;
    let mut reader = BufReader::new(input_file);

    // Step 2: Compress the content and save to the compressed file
    let mut compression_tool = CompressionTool::new();
    let compressed_file = File::create(compressed_file_path)?;
    let mut compressed_writer = BufWriter::new(&compressed_file);

    compression_tool.compress(&mut reader, &mut compressed_writer);

    compressed_writer.flush()?; // Flush the buffer to disk
    drop(compressed_writer); // This is important to ensure that the file is finalized
    

    // Step 3: Decompress the compressed file and save to the decompressed file
    let compressed_file = File::open(compressed_file_path)?; // Reopen the compressed file for reading
    let mut reader = BufReader::new(&compressed_file);
    let decompressed_file = File::create(decompressed_file_path)?;
    let mut decompressed_writer = BufWriter::new(&decompressed_file);
    let decompression_tool = DecompressionTool::new();

    decompression_tool.decompress(&mut reader, &mut decompressed_writer);

    println!("Compression and decompression completed successfully!");

    Ok(())
}
