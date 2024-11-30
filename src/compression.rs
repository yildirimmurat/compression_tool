use crate::huffman::{HuffmanLeafNode, HuffmanInternalNode, HuffmanNode};
use std::collections::HashMap;

pub struct CompressionTool {
    input: String, // todo: should also support streams
}

impl CompressionTool {
    pub fn new(i: &str) -> Self {
        CompressionTool {
            input: i.to_string(),
        }
    }

    pub fn compress(&mut self) -> Result<HashMap<char, i32>, String> {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in self.input.chars() {
            let counter: &mut i32 = map.entry(ch).or_insert(0);
            *counter += 1;
        }
    // Create leaf nodes
    let leaf_a = HuffmanLeafNode::new(5, 'a');
    let leaf_b = HuffmanLeafNode::new(12, 'b');
    let leaf_c = HuffmanLeafNode::new(13, 'c');
    let leaf_d = HuffmanLeafNode::new(14, 'd');

    // Create internal nodes with leaf nodes as children
    let internal_1 = HuffmanInternalNode::new(17, HuffmanNode::Leaf(leaf_a), HuffmanNode::Leaf(leaf_b));
    let internal_2 = HuffmanInternalNode::new(27, HuffmanNode::Leaf(leaf_c), HuffmanNode::Leaf(leaf_d));

    // Create an internal node with other internal nodes as children
    let root = HuffmanInternalNode::new(44, HuffmanNode::Internal(internal_1), HuffmanNode::Internal(internal_2));

    // Wrap nodes in the enum
    let huffman_tree = HuffmanNode::Internal(root);

    // Accessing the weight of the root
    println!("Root weight: {}", huffman_tree.weight());

    // Accessing the value of leaf nodes
    if let Some(value) = huffman_tree.left().unwrap().left().unwrap().value() {
        println!("Left leaf value: {}", value);
    }

    if let Some(value) = huffman_tree.right().unwrap().left().unwrap().value() {
        println!("Right leaf value: {}", value);
    }

        Ok(map)
    }
}
