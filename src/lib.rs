use std::collections::HashMap;
use compression_tool::HuffmannNode;

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

        let node_a = HuffmannNode::new_leaf('A', 5);
        let node_b = HuffmannNode::new_leaf('B', 7);
        let node_c = HuffmannNode::new_leaf('C', 10);
    
        // Create an internal node combining 'A' and 'B'
        let internal_node = HuffmannNode::new_internal(12, node_a, node_b);
    
        // Create the root node combining the internal node and 'C'
        let root_node = HuffmannNode::new_internal(22, internal_node, node_c);
    
        // Print out the structure of the tree
        //println!("{:?}", root_node);

        Ok(map)
    }
}