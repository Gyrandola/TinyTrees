

#[derive(Debug, PartialEq)]
pub struct Node<L> {

    /// Determines if it is a leaf or split node.
    pub is_leaf: bool,

    /// Valid if is_leaf is true.
    pub prediction: L,

    /// Valid if is_leaf is false.
    pub feature_index: u8,

    /// Valid if is_leaf is false.
    pub threshold: f32,

    /// Index of the left child in the tree's node array. (values <= threshold)
    pub left_child_index: usize,

    /// Index of the right child in the tree's node array. (values > threshold)
    pub right_child_index: usize,
}

impl<L> Node<L> {

    pub fn new_leaf(prediction: L) -> Self {
        Node {
            is_leaf: true,
            prediction,
            feature_index: 0,
            threshold: 0.0,
            left_child_index: 0,
            right_child_index: 0,
        }
    }

    pub fn new_split(
        feature_index: u8,
        threshold: f32,
        left_child_index: usize,
        right_child_index: usize)
        -> Self where L: Default {
        Node {
            is_leaf: false,
            prediction: L::default(),
            feature_index,
            threshold,
            left_child_index,
            right_child_index,
        }
    }
}


pub struct Tree<L, const MAX_NODES: usize> {
    nodes: [Node<L>; MAX_NODES],
    num_nodes: usize,
}


