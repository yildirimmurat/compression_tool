use std::{fs::File, io::{self, BufReader, BufWriter}};
use compression_tool::decompression::DecompressionTool;

fn main() -> io::Result<()> {
    // Get the arguments (compressed file and decompressed output file)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: ccunzip <input_file> [output_file]");
        return Ok(());
    }

    let input_file_path = &args[1];    // Path to the compressed file
    let output_file_path = if args.len() == 3 {
        &args[2]
    } else {
        &format!("{}.decompressed", input_file_path)
    };

    let input_file = File::open(input_file_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(&output_file);

    let decompression_tool = DecompressionTool::new();
    decompression_tool.decompress(&mut reader, &mut writer);

    println!("Decompression completed successfully!");
    Ok(())
}
