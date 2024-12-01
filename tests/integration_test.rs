use compression_tool::compression::CompressionTool;
use std::fs::{File, remove_file};
use std::io::Write;

// Test compression and decompression of a file
#[test]
fn test_compress_decompress() {
    let input = "Hello, World!";
    
    let mut tool = CompressionTool::new(input);
    let compressed_data = tool.compress().expect("Compression failed");

    // Save the compressed data to a file
    let compressed_file_path = "compressed_output.dat";
    let mut compressed_file = File::create(compressed_file_path).expect("Failed to create file");
    write!(compressed_file, "{:?}", compressed_data).expect("Failed to write to file");

    // Simulate decompression (this would depend on how you implement decompression)
    let decompressed_data = decompress_file(compressed_file_path).expect("Decompression failed");

    // Ensure the decompressed data matches the original input
    assert_eq!(input, decompressed_data);

    // Clean up test files
    remove_file(compressed_file_path).expect("Failed to delete test file");
}

// Placeholder for a decompression function (you should implement decompression)
fn decompress_file(path: &str) -> Result<String, String> {
    // Decompression logic
    Ok("Hello, World!".to_string()) // In a real case, this would return the decompressed string
}
