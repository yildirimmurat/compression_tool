use crate::huffman::{HuffmanLeafNode, HuffmanInternalNode, HuffmanNode};
use std::{collections::{BinaryHeap, BTreeMap}, fs::File, io::{self, Read}};

pub struct DecompressionTool {
    file_path: String,
}

impl DecompressionTool {
    pub fn new(file_path: &str) -> Self {
        DecompressionTool {
            file_path: file_path.to_string(),
        }
    }

    pub fn read_header(file: &mut File) -> io::Result<BTreeMap<char, i32>> {
        let mut frequency_map: BTreeMap<char, i32> = BTreeMap::new();
    
        let mut num_chars_bytes: [u8; 4] = [0u8; 4];
        file.read_exact(&mut num_chars_bytes)?;
        let num_chars: usize = u32::from_le_bytes(num_chars_bytes) as usize;
    
        for _ in 0..num_chars {
            let mut char_buffer = [0u8; 1];
            file.read_exact(&mut char_buffer)?;
            let ch = char_buffer[0] as char;
    
            let mut count_bytes = [0u8; 4];
            file.read_exact(&mut count_bytes)?;
            let count = i32::from_le_bytes(count_bytes);
    
            frequency_map.insert(ch, count);
        }
    
        // Skip the delimiter
        file.read_exact(&mut [0u8; 1])?;
    
        Ok(frequency_map)
    }
    

    pub fn rebuild_tree(&self, frequency_map: &BTreeMap<char, i32>) -> Option<HuffmanNode> {
        let mut heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();

        // Push each character into the heap as a leaf node
        for (ch, count) in frequency_map {
            let leaf = HuffmanLeafNode::new(*count, *ch);
            heap.push(HuffmanNode::Leaf(leaf));
        }

        // Rebuild the Huffman tree
        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            let combined_weight = left.weight() + right.weight();
            let internal_node = HuffmanInternalNode::new(combined_weight, left, right);
            heap.push(HuffmanNode::Internal(internal_node));
        }

        heap.pop() // root
    }

    fn read_compressed_data(&self, file: &mut File) -> io::Result<Vec<u8>> {
        let mut compressed_data = Vec::new();
        file.read_to_end(&mut compressed_data)?;
        Ok(compressed_data)
    }

    fn decode_data(&self, tree: &HuffmanNode, compressed_data: Vec<u8>) -> String {
        let mut current_node: &HuffmanNode = tree;
        let mut decoded_string: String = String::new();

        // Extract the padding bits from the first byte (which indicates how many bits were padded)
        let padding_bits = compressed_data[0] as usize; // First byte indicates padding
        let mut bits: Vec<bool> = compressed_data[1..]
            .iter() // Use `iter()` to iterate over the bytes
            .flat_map(|byte| (0..8).map(move |i| (byte >> (7u8 - i)) & 1u8 == 1u8)) // Convert bytes to bits
            .collect();

        // Handle the last byte padding
        if padding_bits > 0 {
            // We want to remove the padding bits from the start of the last byte.
            let last_byte = &compressed_data[compressed_data.len() - 1];
            let mut last_byte_bits = (0..8)
                .map(|i| (last_byte >> (7 - i)) & 1 == 1)
                .collect::<Vec<bool>>();

            // Truncate the padding bits from the start of the last byte
            last_byte_bits = last_byte_bits[padding_bits..].to_vec();

            // Remove the last byte's bits from the original bitstream and append the truncated bits
            bits = bits[..bits.len() - 8].to_vec();
            bits.extend(last_byte_bits);
        }

        // Traverse the Huffman tree to decode the bits
        let mut current_bit = 0;
        while current_bit < bits.len() {
            let bit = bits[current_bit];
            current_bit += 1;

            // Move down the tree based on the bit
            current_node = match current_node {
                HuffmanNode::Leaf(leaf) => {
                    decoded_string.push(leaf.value());  // Append the decoded character
                    tree // Reset to the root of the tree for the next character
                },
                HuffmanNode::Internal(internal) => {
                    // Traverse the internal node based on the bit (0 = left, 1 = right)
                    if bit {
                        let next_node = &internal.right(); // Move to the right child if the bit is 1
                        if let HuffmanNode::Leaf(leaf) = next_node {
                            decoded_string.push(leaf.value());
                            tree // Reset to the root of the tree for the next character
                        } else {
                            next_node // Continue moving down the internal node tree
                        }
                    } else {
                        let next_node = &internal.left(); // Move to the left child if the bit is 0
                        if let HuffmanNode::Leaf(leaf) = next_node {
                            decoded_string.push(leaf.value());
                            tree // Reset to the root of the tree for the next character
                        } else {
                            next_node // Continue moving down the internal node tree
                        }
                    }
                },
            };
        }

        decoded_string
    }
    
    

    pub fn decompress(&self) -> Result<String, String> {
        let mut file = File::open(&self.file_path).map_err(|e| e.to_string())?;

        // Read the frequency map from the header
        let frequency_map = DecompressionTool::read_header(&mut file).map_err(|e| e.to_string())?;

        // Rebuild the Huffman tree from the frequency map
        let huffman_tree = self.rebuild_tree(&frequency_map).ok_or("Failed to rebuild the Huffman tree")?;

        // Read the compressed data
        let compressed_data = self.read_compressed_data(&mut file).map_err(|e| e.to_string())?;

        // Decode the data using the Huffman tree
        let decoded_string = self.decode_data(&huffman_tree, compressed_data);

        Ok(decoded_string)
    }
}
