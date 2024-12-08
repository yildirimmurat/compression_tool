use crate::huffman::{HuffmanLeafNode, HuffmanInternalNode, HuffmanNode};
use std::{collections::{BinaryHeap, HashMap}, fs::File, io::{self, Write}};
use std::collections::BTreeMap;
pub struct CompressionTool {
    input: String, // todo: should also support streams
}

impl CompressionTool {
    pub fn new(i: &str) -> Self {
        CompressionTool {
            input: i.to_string(),
        }
    }

    fn generate_frequency_map(&self) -> BTreeMap<char, i32> {
        let mut map: BTreeMap<char, i32> = BTreeMap::new();

        for ch in self.input.chars() {
            let counter: &mut i32 = map.entry(ch).or_insert(0);
            *counter += 1;
        }

        map
    }

    fn write_header(&self, file: &mut File, frequency_map: BTreeMap<char, i32>) -> io::Result<()> {
        let num_chars: u32 = frequency_map.len() as u32;
        file.write_all(&num_chars.to_le_bytes())?;
    
        for (ch, count) in frequency_map {
            file.write_all(&[ch as u8])?;
            file.write_all(&count.to_le_bytes())?;
        }
    
        file.write_all(&[0x00])?;
        Ok(())
    }
    

    pub fn compress(&mut self, output_file: &str) -> Result<Vec<u8>, String> {
        let mut file: File = File::create(output_file).map_err(|e| e.to_string())?;

        let frequency_map: BTreeMap<char, i32> = self.generate_frequency_map();

        self.write_header(&mut file, frequency_map.clone())
            .map_err(|e| format!("Error writing header: {}", e))?;

        

        let mut heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();
        for (ch, count) in frequency_map {
            let leaf: HuffmanLeafNode = HuffmanLeafNode::new(count, ch);
            heap.push(HuffmanNode::Leaf(leaf));
        }

        // Build the Huffman tree
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
        let compressed_data = self.compressed_data(&codes);
        file.write_all(&compressed_data).map_err(|e| e.to_string())?;


        Ok(compressed_data)
    }

    fn compressed_data(&self, codes: &HashMap<char, String>) -> Vec<u8> {
        let mut compressed_bits: String = String::new();
        
        // Generate the compressed binary string
        for ch in self.input.chars() {
            compressed_bits.push_str(&codes[&ch]);  // Append the Huffman code for each character
        }
    
        // Pad the compressed data to be a multiple of 8 bits if necessary
        let padding_bits: usize = 8 - compressed_bits.len() % 8;
    
        // Convert the binary string to a byte vector
        let mut result: Vec<u8> = Vec::new();
        for chunk in compressed_bits.as_bytes().chunks(8) {
            let byte: u8 = chunk.iter().fold(0, |acc, &bit| (acc << 1) | (bit - b'0'));
            result.push(byte);
        }
    
        // Store the padding information to be used during decompression (as a header or in the data itself)
        result.insert(0, padding_bits as u8); // Add padding byte at the beginning of the result
    
        result
    }
    
    
}
