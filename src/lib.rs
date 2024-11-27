#[derive(Debug)]
pub struct CompressionTool {
    input: String,
}

impl CompressionTool {
    pub fn new(input: &str) -> Self {
        CompressionTool {
            input: input.to_string(),
        }
    }

    pub fn compress(&mut self) -> Result<String, String> {
        Ok("done".to_string())
    }
}