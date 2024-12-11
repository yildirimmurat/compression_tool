use std::{io::{Write}};
use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek, SeekFrom};
use std::collections::BinaryHeap;
use crate::huffman::{HuffmanInternalNode, HuffmanLeafNode, HuffmanNode};

pub struct CompressionTool {
}

impl CompressionTool {
    pub fn new() -> Self {
        CompressionTool {
        }
    }    

    pub fn compress<R: Read + Seek, W: Write>(&mut self, reader: &mut R, writer: &mut W) {
        let mut buffer: [u8; 1024] = [0u8; 1024];
        let mut frequency_map: BTreeMap<char, i32> = BTreeMap::new();

        // Count frequencies
        loop {
            let bytes_read = reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }

            // Count character frequencies in the buffer
            for &byte in &buffer[..bytes_read] {
                let ch = byte as char;
                *frequency_map.entry(ch).or_insert(0) += 1;
            }
        }

        // Write frequency map size to header
        let num_chars: u32 = frequency_map.len() as u32; // Correct size of the map
        let _ = writer.write_all(&num_chars.to_le_bytes()); // Write the size as a little-endian 4-byte integer
        
        // Now write map to header
        for (ch, count) in &frequency_map {
            let _ = writer.write_all(&[*ch as u8]); // Write the character (1 byte)
            let _ = writer.write_all(&count.to_le_bytes()); // Write the frequency (4 bytes)
        }

        let _ = writer.write_all(&[0x00]);

        let mut heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();
        for (ch, count) in frequency_map {
            let leaf: HuffmanLeafNode = HuffmanLeafNode::new(count, ch);
            heap.push(HuffmanNode::Leaf(leaf));
        }

        // Now, generate the Huffman tree, encode the data, and write it incrementally
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

        // Generate the prefix codes for each character
        let mut codes: HashMap<char, String> = HashMap::new();
        root.generate_prefix_codes(&mut codes);

        // Now write the compressed data after the header
        let mut compressed_bits: Vec<u8> = Vec::new();

        // Go back to the start of the file
        reader.seek(SeekFrom::Start(0)).unwrap();
        loop {
            let bytes_read = reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }

            for &byte in &buffer[..bytes_read] {
                if let Some(code) = codes.get(&(byte as char)) {
                    for bit in code.chars() {
                        compressed_bits.push(if bit == '1' { 1 } else { 0 });
                    }
                }
            }
        }

        // Calculate padding to make the bit length a multiple of 8
        let padding_bits = (8 - compressed_bits.len() % 8) % 8;

        if padding_bits > 0 {
            // Prepend padding bits (insert them before the last byte)
            // Insert `padding_bits` number of zeroes at the beginning of compressed_bits
            let padding = vec![0; padding_bits]; // Create the padding vector
            compressed_bits.splice(compressed_bits.len() - compressed_bits.len() % 8..compressed_bits.len() - compressed_bits.len() % 8, padding);
        }

        // Convert the bit vector to a byte vector
        let mut result: Vec<u8> = Vec::new();
        for chunk in compressed_bits.chunks(8) {
            let mut byte: u8 = 0;
            for (i, &bit) in chunk.iter().enumerate() {
                byte |= bit << (7 - i); // Shift each bit into the correct position
            }
            result.push(byte);
        }

        // Insert the padding information at the beginning of the result
        result.insert(0, padding_bits as u8);

        // Write the result to the writer (file or other output)
        writer.write_all(&result).unwrap();
    }
}
