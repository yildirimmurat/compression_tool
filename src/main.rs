use std::fs::File;
use std::io::Read;

use compression_tool::CompressionTool;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path: String = args[1].to_string();

    let mut file: File = File::open(file_path).unwrap();
    
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);

    let mut tool = CompressionTool::new(&content);
    match tool.compress() {
        Ok(_) => println!("valid"),
        Err(_) => println!("error")
    }
}
