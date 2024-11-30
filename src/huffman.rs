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
