use compression_tool::compression::CompressionTool;
use compression_tool::decompression::DecompressionTool;
use std::fs::remove_file;

// Test compression and decompression of a file
#[test]
fn test_compress_decompress() {
    // Sample input string
    let input = "Hello, World!";

    // Step 1: Compress the input string
    let mut tool = CompressionTool::new(input);
    
    // Output compressed file path
    let compressed_file_path = "compressed_output.dat";
    
    // Compress and save the compressed data to a file
    tool.compress(compressed_file_path).expect("Compression failed");

    // Step 2: Decompress the compressed file
    let decompression_tool = DecompressionTool::new(compressed_file_path);
    let decompressed_data = decompression_tool.decompress().expect("Decompression failed");

    // Step 3: Ensure the decompressed data matches the original input
    assert_eq!(input, decompressed_data);

    // Clean up test files
    remove_file(compressed_file_path).expect("Failed to delete test file");
}
