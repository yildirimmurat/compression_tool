use compression_tool::compression::CompressionTool;
use compression_tool::decompression::DecompressionTool;
use std::fs::{remove_file, File};
use std::io::{BufReader, BufWriter, Read, Write};

// Test compression and decompression of a file
#[test]
fn test_compress_decompress() {
    let input_file_path = "tests/test.txt";
    let compressed_file_path = "compressed_output.dat";
    let decompressed_file_path = "decompressed_output.txt";

    // Step 1: Read the input file and compress it
    let input_file = File::open(input_file_path).unwrap();
    let mut reader = BufReader::new(input_file);

    let mut compression_tool = CompressionTool::new();
    let compressed_file = File::create(compressed_file_path).unwrap();
    let mut compressed_writer = BufWriter::new(compressed_file);
    compression_tool.compress(&mut reader, &mut compressed_writer);

    // **Ensure the compression writer is flushed to disk**
    compressed_writer.flush().unwrap();

    // Step 2: After flush, close the writer (drop the BufWriter)
    drop(compressed_writer);  // Ensures the writer is fully flushed and the file is closed

    // Step 3: Decompress the compressed file
    let compressed_file_for_decompression = File::open(compressed_file_path).unwrap();
    let decompressed_file = File::create(decompressed_file_path).unwrap();
    let decompression_tool = DecompressionTool::new();

    let mut decompressed_writer = BufWriter::new(decompressed_file);
    decompression_tool.decompress(&mut BufReader::new(compressed_file_for_decompression), &mut decompressed_writer);

    // **Ensure the decompressed writer is flushed to disk**
    decompressed_writer.flush().unwrap();

    // Step 4: Read and compare decompressed data with the original file
    let mut decompressed_file = File::open(decompressed_file_path).unwrap();
    let mut decompressed_content = Vec::new();
    decompressed_file.read_to_end(&mut decompressed_content).unwrap();

    let mut original_file = File::open(input_file_path).unwrap();
    let mut original_content = Vec::new();
    original_file.read_to_end(&mut original_content).unwrap();

    // Step 5: Assert that the decompressed content matches the original
    assert_eq!(original_content, decompressed_content);

    // Clean up test files
    remove_file(compressed_file_path).expect("Failed to delete compressed file");
    remove_file(decompressed_file_path).expect("Failed to delete decompressed file");
}
