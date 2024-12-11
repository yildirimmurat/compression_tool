use crate::huffman::{HuffmanLeafNode, HuffmanInternalNode, HuffmanNode};
use std::{collections::{BinaryHeap, BTreeMap}, io::{Read, Seek, Write}};

pub struct DecompressionTool {
}

impl DecompressionTool {
    pub fn new() -> Self {
        DecompressionTool {
        }
    }
    
    pub fn decompress<R: Read + Seek, W: Write>(&self, reader: &mut R, writer: &mut W) {
        // Step 1: Read the frequency map from the header
        let mut frequency_map: BTreeMap<char, i32> = BTreeMap::new();

        let mut num_chars_bytes: [u8; 4] = [0u8; 4];
        let _ = reader.read_exact(&mut num_chars_bytes);
        let num_chars: usize = u32::from_le_bytes(num_chars_bytes) as usize;

        for _ in 0..num_chars {
            let mut char_buffer = [0u8; 1];
            let _  = reader.read_exact(&mut char_buffer);
            let ch = char_buffer[0] as char;

            let mut count_bytes = [0u8; 4];
            let _ = reader.read_exact(&mut count_bytes);
            let count = i32::from_le_bytes(count_bytes);

            frequency_map.insert(ch, count);
        }

        // Skip the delimiter
        let _  = reader.read_exact(&mut [0u8; 1]);

        // Step 2: Rebuild the Huffman tree from the frequency map
        let mut heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();

        // Push each character into the heap as a leaf node
        for (ch, count) in frequency_map {
            let leaf = HuffmanLeafNode::new(count, ch);
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

        let huffman_tree = heap.pop().unwrap(); // root

        // Step 3: Read the compressed data
        let mut compressed_data = Vec::new();
        let _ = reader.read_to_end(&mut compressed_data);

        // Step4: Decode the data using the Huffman tree
        let mut current_node: &HuffmanNode = &huffman_tree;

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
                .map(|i| (last_byte >> (7 - i)) & 1u8 == 1u8)
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
                    let _ = writer.write_all(&[leaf.value() as u8]);  // Append the decoded character
                    &huffman_tree // Reset to the root of the tree for the next character
                },
                HuffmanNode::Internal(internal) => {
                    // Traverse the internal node based on the bit (0 = left, 1 = right)
                    if bit {
                        let next_node = &internal.right(); // Move to the right child if the bit is 1
                        if let HuffmanNode::Leaf(leaf) = next_node {
                            let _ = writer.write_all(&[leaf.value() as u8]);
                            &huffman_tree // Reset to the root of the tree for the next character
                        } else {
                            next_node // Continue moving down the internal node tree
                        }
                    } else {
                        let next_node = &internal.left(); // Move to the left child if the bit is 0
                        if let HuffmanNode::Leaf(leaf) = next_node {
                            let _ = writer.write_all(&[leaf.value() as u8]);
                            &huffman_tree // Reset to the root of the tree for the next character
                        } else {
                            next_node // Continue moving down the internal node tree
                        }
                    }
                },
            };
        }
    }
}
