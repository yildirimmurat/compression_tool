use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

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
            HuffmanNode::Leaf(_) => true,
            HuffmanNode::Internal(_) => false,
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
}

// Implementing Ord and PartialOrd for the HuffmanNode so we can use BinaryHeap
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
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
}

