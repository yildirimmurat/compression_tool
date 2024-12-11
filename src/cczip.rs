use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use compression_tool::compression::CompressionTool;

fn main() -> io::Result<()> {
    // Get the arguments (input file and compressed output file)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cczip <input_file> [output_file]");
        return Ok(());
    }

    let input_file_path = &args[1];        // Path to the input file
    let output_file_path = if args.len() == 3 {
        &args[2]
    } else {
        &format!("{}.compressed", input_file_path)
    };

    let input_file = File::open(input_file_path)?;
    let mut reader = BufReader::new(input_file);

    let mut compression_tool = CompressionTool::new();
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(&output_file);

    compression_tool.compress(&mut reader, &mut writer);
    writer.flush()?;

    println!("Compression completed successfully!");
    Ok(())
}