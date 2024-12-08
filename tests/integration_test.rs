use compression_tool::compression::CompressionTool;
use compression_tool::decompression::DecompressionTool;
use std::fs::{remove_file, File};
use std::io::Read;

// Test compression and decompression of a file
#[test]
fn test_compress_decompress() {
    let mut input = String::new();

    let input_file_path = "tests/test.txt";
    let mut input_file = File::open(input_file_path).unwrap();
    input_file.read_to_string(&mut input).unwrap();

    // Step 1: Compress the input string
    let mut tool = CompressionTool::new(input.clone()); // @todo: better with reference
    
    // Output compressed file path
    let compressed_file_path = "compressed_output.dat";
    
    // Compress and save the compressed data to a file
    tool.compress(compressed_file_path).expect("Compression failed");

    // Step 2: Decompress the compressed file
    let decompression_tool = DecompressionTool::new(compressed_file_path);
    let decompressed_data = decompression_tool.decompress().expect("Decompression failed");

    // Clean up test files
    remove_file(compressed_file_path).expect("Failed to delete test file");

    // Step 3: Ensure the decompressed data matches the original input
    assert_eq!(input, decompressed_data);
}
