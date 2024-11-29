use std::collections::HashMap;

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

    pub fn compress(&mut self) -> Result<HashMap<char, i32>, String> {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in self.input.chars() {
            let counter: &mut i32 = map.entry(ch).or_insert(0);
            *counter += 1;
        }

        Ok(map)
    }
}