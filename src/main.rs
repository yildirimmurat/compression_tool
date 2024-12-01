use std::fs::File;
use std::io::Read;

use compression_tool::compression::CompressionTool;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <input_file> <output_file>");
        return;
    }
    let input_file: &String = &args[1];
    let output_file: &String = &args[2];

    let mut file: File = File::open(input_file).unwrap();    
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);

    let mut tool: CompressionTool = CompressionTool::new(&content);
    match tool.compress(&output_file) {
        Ok(_) => println!("Compression successfull, file written to '{}'", output_file),
        Err(e) => println!("Error: {}", e),
    }
}
