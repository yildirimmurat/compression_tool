use crate::huffman::{HuffmanLeafNode, HuffmanInternalNode, HuffmanNode};
use std::collections::{BinaryHeap, HashMap};

pub struct CompressionTool {
    input: String, // todo: should also support streams
}

impl CompressionTool {
    pub fn new(i: &str) -> Self {
        CompressionTool {
            input: i.to_string(),
        }
    }

    pub fn compress(&mut self) -> Result<HuffmanNode, String> {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in self.input.chars() {
            let counter: &mut i32 = map.entry(ch).or_insert(0);
            *counter += 1;
        }

        let mut heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();
        for (ch, count) in map {
            let leaf: HuffmanLeafNode = HuffmanLeafNode::new(count, ch);
            heap.push(HuffmanNode::Leaf(leaf));
        }

        while heap.len() > 1 {
            // Pop the two nodes with the smallest frequencies
            let left: HuffmanNode = heap.pop().unwrap();
            let right: HuffmanNode = heap.pop().unwrap();
                
            // Combine the two nodes into an internal node
            let combined_weight: i32 = left.weight() + right.weight();
            let internal_node: HuffmanInternalNode = HuffmanInternalNode::new(combined_weight, left, right);
            heap.push(HuffmanNode::Internal(internal_node));
        }

        let root = heap.pop().unwrap();

        let mut codes: HashMap<char, String> = HashMap::new();
        root.generate_prefix_codes(&mut codes);
        for (ch, code) in &codes {
            println!("Character: '{}' -> code: {}", ch, code);
        }

        Ok(root)
    }
}
