struct HuffmannNode {
    is_leaf: bool,
    element: Option<char>,
    weight: i32,
    left: Option<Box<HuffmannNode>>,
    right: Option<Box<HuffmannNode>>,
}

impl HuffmannNode {
    pub fn new_leaf(el: char, wt: i32) -> Self {
        HuffmannNode {
            is_leaf: true,
            element: Some(el),
            weight: wt,
            left: None,
            right: None,
        }
    }

    pub fn new_internal(wt: i32, left: HuffmannNode, right: HuffmannNode) -> Self {
        HuffmannNode {
            is_leaf: false,
            element: None,
            weight: wt,
            left: Some(Box::new(HuffmannNode)),
            right: Some(Box::new(HuffmannNode)),
        }
    }

    pub fn isLeaf(&self) -> bool {
        self.isLeaf
    }

    pub fn value(&self) -> Option<char> {
        self.element
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }

    pub fn left(&self) -> Option<&HuffmannNode> {
        self.left.as_ref().map(|node| &**node)
    }

    pub fn right(&self) -> Option<&HuffmannNode> {
        self.right.as_ref().map(|node| &**node)
    }
}