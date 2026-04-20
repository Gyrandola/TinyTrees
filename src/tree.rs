#[derive(Debug, PartialEq)]
pub enum Node<L> {
    Leaf {
        prediction: L,
    },
    Split {
        feature_index: u8,
        threshold: f32,

        /// Left child node (values <= threshold).
        left_child_index: usize,
        /// Right child node (values > threshold).
        right_child_index: usize,
    },
}

impl<L> Node<L> {

    /// Creates a new leaf node.
    pub fn new_leaf(prediction: L) -> Self {
        Node::Leaf { prediction }
    }

    /// Creates a new split node.
    pub fn new_split(feature_index: u8, threshold: f32, left_child_index: usize, right_child_index: usize,
    ) -> Self {
        Node::Split {
            feature_index,
            threshold,
            left_child_index,
            right_child_index,
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf { .. })
    }

    // Getters

    pub fn get_prediction(&self) -> Option<&L> {
        if let Node::Leaf { prediction } = self {
            Some(prediction)
        } else {
            None
        }
    }

    pub fn get_mut_prediction(&mut self) -> Option<&mut L> {
        if let Node::Leaf { prediction } = self {
            Some(prediction)
        } else {
            None
        }
    }

    pub fn get_split_details(&self) -> Option<(u8, f32, usize, usize)> {
        if let Node::Split { feature_index, threshold, left_child_index, right_child_index,
        } = self {
            Some((*feature_index, *threshold, *left_child_index, *right_child_index))
        } else {
            None
        }
    }
}

pub struct Tree<L, const MAX_NODES: usize> {
    nodes: [Node<L>; MAX_NODES],
    num_nodes: usize,
}
