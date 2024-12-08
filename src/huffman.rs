use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::collections::HashMap;

struct HuffmanBaseNode {
    is_leaf: bool,
    weight :i32,
}

impl HuffmanBaseNode {
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }
    pub fn weight(&self) -> i32 {
        self.weight
    }
}

pub struct HuffmanLeafNode {
    base: HuffmanBaseNode,
    element: char,
}

impl HuffmanLeafNode {
    pub fn new(wt: i32, el: char) -> Self {
        HuffmanLeafNode {
            base: HuffmanBaseNode {
                is_leaf: true,
                weight: wt,
            },
            element: el,
        }
    }

    pub fn value(&self) -> char {
        self.element
    }

    pub fn weight(&self) -> i32 {
        self.base.weight
    }
}

pub struct HuffmanInternalNode {
    base: HuffmanBaseNode,
    left: Box<HuffmanNode>,
    right: Box<HuffmanNode>,
}

impl HuffmanInternalNode {
    pub fn new (wt: i32, l: HuffmanNode, r: HuffmanNode) -> Self {
        HuffmanInternalNode {
            base: HuffmanBaseNode {
                is_leaf: false,
                weight: wt,
            },
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn left(&self) -> &HuffmanNode {
        &self.left
    }
    pub fn right(&self) -> &HuffmanNode {
        &self.right
    }

    pub fn weight(&self) -> i32 {
        self.base.weight()
    }
}

pub enum HuffmanNode {
    Leaf(HuffmanLeafNode),
    Internal(HuffmanInternalNode),
}

impl HuffmanNode {
    pub fn weight(&self) -> i32 {
        match self {
            HuffmanNode::Leaf(leaf) => leaf.base.weight,
            HuffmanNode::Internal(internal) => internal.base.weight,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            HuffmanNode::Leaf(leaf) => leaf.base.is_leaf(),
            HuffmanNode::Internal(internal) => internal.base.is_leaf(),
        }
    }

    pub fn value(&self) -> Option<char> {
        match self {
            HuffmanNode::Leaf(leaf) => Some(leaf.value()),
            _ => None,
        }
    }

    pub fn left(&self) -> Option<&HuffmanNode> {
        match self {
            HuffmanNode::Leaf(_) => None,
            HuffmanNode::Internal(internal) => Some(&internal.left()),
        }
    }

    pub fn right(&self) -> Option<&HuffmanNode> {
        match self {
            HuffmanNode::Leaf(_) => None,
            HuffmanNode::Internal(internal) => Some(&internal.right()),
        }
    }

    // Function to print the tree structure from top to bottom
    pub fn print_tree(&self) -> String {
        self.print_tree_helper(0)
    }

    // Helper function for printing the tree with depth tracking
    fn print_tree_helper(&self, depth: usize) -> String {
        match self {
            HuffmanNode::Leaf(leaf) => {
                // Print leaf node with indentation based on depth
                format!("{}Leaf: {}\n", "  ".repeat(depth), leaf.value())
            },
            HuffmanNode::Internal(internal) => {
                // Start with the internal node label
                let mut result = format!("{}Internal Node\n", "  ".repeat(depth));

                // Recursively print left and right children with updated depth
                let left_str = internal.left.print_tree_helper(depth + 1);
                let right_str = internal.right.print_tree_helper(depth + 1);

                // Merge left and right strings and return
                result.push_str(&left_str);
                result.push_str(&right_str);

                result
            },
        }
    }

    // Function to generate the prefix codes
    pub fn generate_prefix_codes(&self, codes: &mut HashMap<char, String>) {
        self.generate_prefix_codes_helper("".to_string(), codes);
    }

    fn generate_prefix_codes_helper(&self, prefix: String, codes: &mut HashMap<char, String>) {
        match self {
            HuffmanNode::Leaf(leaf) => {
                // Store the prefix code for the leaf node
                codes.insert(leaf.value(), prefix);
            },
            HuffmanNode::Internal(internal) => {
                // For left child, append "0"
                internal.left.generate_prefix_codes_helper(prefix.clone() + "0", codes);
                
                // For right child, append "1"
                internal.right.generate_prefix_codes_helper(prefix.clone() + "1", codes);
            },
        }
    }
    
}

// Implementing Ord and PartialOrd for the HuffmanNode so we can use BinaryHeap
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight().cmp(&self.weight())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HuffmanNode {

}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

// For printing the tree 
impl Debug for HuffmanNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HuffmanNode::Leaf(leaf) => write!(f, "Leaf({} : {})", leaf.value(), leaf.base.weight()),
            HuffmanNode::Internal(internal) => write!(f, "Internal({}, left: {:?}, right: {:?})", internal.base.weight(), internal.left(), internal.right()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test HuffmanLeafNode creation
    #[test]
    fn test_leaf_node_creation() {
        let leaf = HuffmanLeafNode::new(3, 'a');
        assert_eq!(leaf.value(), 'a');
        assert_eq!(leaf.weight(), 3);
    }

    // Test HuffmanInternalNode creation
    #[test]
    fn test_internal_node_creation() {
        let left = HuffmanNode::Leaf(HuffmanLeafNode::new(2, 'b'));
        let right = HuffmanNode::Leaf(HuffmanLeafNode::new(3, 'a'));
        let internal_node = HuffmanInternalNode::new(5, left, right);
        assert_eq!(internal_node.base.weight, 5);
    }

    // Test Prefix Code Generation
    #[test]
    fn test_generate_prefix_codes() {
        // Create a simple Huffman tree:
        //        Internal
        //       /      \
        //   Leaf(a)  Internal
        //             /      \
        //        Leaf(b)   Leaf(c)

        let node_a = HuffmanNode::Leaf(HuffmanLeafNode::new(5, 'a'));
        let node_b = HuffmanNode::Leaf(HuffmanLeafNode::new(3, 'b'));
        let node_c = HuffmanNode::Leaf(HuffmanLeafNode::new(2, 'c'));

        let internal1 = HuffmanInternalNode::new(
            node_b.weight() + node_c.weight(),
            node_b,
            node_c
        );

        let root = HuffmanNode::Internal(internal1);
        let root = HuffmanNode::Internal(
            HuffmanInternalNode::new(node_a.weight() + root.weight(), node_a, root)
        );

        let mut codes = HashMap::new();
        root.generate_prefix_codes(&mut codes);

        // Expected prefix codes:
        let mut expected_codes = HashMap::new();
        expected_codes.insert('a', "0".to_string());
        expected_codes.insert('b', "10".to_string());
        expected_codes.insert('c', "11".to_string());

        // Test if the generated prefix codes match the expected ones
        assert_eq!(codes, expected_codes);
    }

    // Test with an empty tree (single node case)
    #[test]
    fn test_single_node_tree() {
        // Single node tree (just one leaf node)
        let node_a = HuffmanNode::Leaf(HuffmanLeafNode::new(5, 'a'));

        let mut codes = HashMap::new();
        node_a.generate_prefix_codes(&mut codes);

        let mut expected_codes = HashMap::new();
        expected_codes.insert('a', "".to_string());  // No prefix for a single node

        // Test if the generated prefix codes match the expected ones
        assert_eq!(codes, expected_codes);
    }

    // Test with a more complex tree
    #[test]
    fn test_complex_tree() {
        // Create a complex Huffman tree:
        //         Internal
        //        /      \
        //   Internal   Internal
        //   /    \     /     \
        //  Leaf(a) Leaf(b) Leaf(c) Leaf(d)

        let node_a = HuffmanNode::Leaf(HuffmanLeafNode::new(5, 'a'));
        let node_b = HuffmanNode::Leaf(HuffmanLeafNode::new(3, 'b'));
        let node_c = HuffmanNode::Leaf(HuffmanLeafNode::new(2, 'c'));
        let node_d = HuffmanNode::Leaf(HuffmanLeafNode::new(1, 'd'));

        let internal1 = HuffmanNode::Internal(HuffmanInternalNode::new(
            node_a.weight() + node_b.weight(),
            node_a,
            node_b,
        ));
        let internal2 = HuffmanNode::Internal(HuffmanInternalNode::new(
            node_c.weight() + node_d.weight(),
            node_c,
            node_d,
        ));

        let root = HuffmanNode::Internal(
            HuffmanInternalNode::new(
                internal1.weight() + internal2.weight(),
                internal1,
                internal2,
            )
        );

        let mut codes = HashMap::new();
        root.generate_prefix_codes(&mut codes);

        // Expected prefix codes for the given tree
        let mut expected_codes = HashMap::new();
        expected_codes.insert('a', "00".to_string());
        expected_codes.insert('b', "01".to_string());
        expected_codes.insert('c', "10".to_string());
        expected_codes.insert('d', "11".to_string());

        // Test if the generated prefix codes match the expected ones
        assert_eq!(codes, expected_codes);
    }
}

