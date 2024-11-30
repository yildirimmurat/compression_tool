use compression_tool::huffman::HuffmanNode;
use compression_tool::compression::CompressionTool;

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to extract the leaf nodes from the Huffman tree for validation
    fn extract_leaves(node: &HuffmanNode, leaves: &mut Vec<(char, i32)>) {
        match node {
            HuffmanNode::Leaf(leaf) => {
                leaves.push((leaf.value(), leaf.weight()));
            }
            HuffmanNode::Internal(internal) => {
                extract_leaves(&internal.left(), leaves);
                extract_leaves(&internal.right(), leaves);
            }
        }
    }

    // Test for a simple input string
    #[test]
    fn test_huffman_tree_structure() {
        let input = "abacab";
        let mut tool = CompressionTool::new(input);
        
        // Compress the input to get the Huffman tree root
        let root = tool.compress().expect("Compression failed");
        
        // Extract all leaf nodes and their frequencies
        let mut leaves = Vec::new();
        extract_leaves(&root, &mut leaves);

        // The expected frequencies for "a" and "b" in the string "abacab"
        let expected_frequencies = vec![('a', 3), ('b', 2), ('c', 1)];
        
        // Sort both vectors so we can compare them
        leaves.sort_by(|a, b| a.0.cmp(&b.0));  // Sort by character
        let mut expected_frequencies = expected_frequencies;
        expected_frequencies.sort_by(|a, b| a.0.cmp(&b.0));  // Sort by character

        // Check that the leaves match the expected frequencies
        assert_eq!(leaves, expected_frequencies);
    }

    // Test for a case with a more complex string
    #[test]
    fn test_complex_huffman_tree() {
        let input = "this is an example of huffman compression";
        let mut tool = CompressionTool::new(input);
        
        // Compress the input to get the Huffman tree root
        let root = tool.compress().expect("Compression failed");
        
        // Extract all leaf nodes and their frequencies
        let mut leaves = Vec::new();
        extract_leaves(&root, &mut leaves);
        
        // Expected frequencies for a more complex string (you can manually calculate or expect a certain structure)
        let expected_frequencies = vec![
            (' ', 6), ('a', 3), ('e', 3), ('s', 4), ('i', 3), 
            ('n', 3), ('t', 1), ('h', 2), ('m', 3), ('o', 3),
            ('f', 3), ('l', 1), ('x', 1), ('p', 2), ('c', 1),
            ('r', 1), ('u', 1),
        ];
        
        // Sort both vectors so we can compare them
        leaves.sort_by(|a, b| a.0.cmp(&b.0));  // Sort by character
        let mut expected_frequencies = expected_frequencies;
        expected_frequencies.sort_by(|a, b| a.0.cmp(&b.0));  // Sort by character

        // Check that the leaves match the expected frequencies
        assert_eq!(leaves, expected_frequencies);
    }

    // Test to check if the tree is properly built (you can manually check if internal nodes are correct)
    #[test]
    fn test_tree_structure() {
        let input = "aaabbbcc";
        let mut tool = CompressionTool::new(input);

        // Compress the input to get the Huffman tree root
        let root = tool.compress().expect("Compression failed");

        // We should have a tree with only two internal nodes (since we only have three distinct characters)
        let internal_count = count_internal_nodes(&root);
        assert_eq!(internal_count, 2, "The tree should have 2 internal nodes");

        // Also check the total weight of the tree (should be equal to the sum of character frequencies)
        let total_weight = sum_weights(&root);
        assert_eq!(total_weight, input.len() as i32, "The total weight should be equal to the length of the input string");
    }

    // Helper function to count the number of internal nodes
    fn count_internal_nodes(node: &HuffmanNode) -> i32 {
        match node {
            HuffmanNode::Leaf(_) => 0,
            HuffmanNode::Internal(internal) => {
                1 + count_internal_nodes(&internal.left()) + count_internal_nodes(&internal.right())
            }
        }
    }

    // Helper function to sum the weights of all nodes (leaf and internal)
    fn sum_weights(node: &HuffmanNode) -> i32 {
        match node {
            HuffmanNode::Leaf(leaf) => leaf.weight(),
            HuffmanNode::Internal(internal) => sum_weights(&internal.left()) + sum_weights(&internal.right()),
        }
    }
}
