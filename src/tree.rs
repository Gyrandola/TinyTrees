use crate::errors::TreeError;

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

impl<L, const MAX_NODES: usize> Tree<L, MAX_NODES> {

    /// Creates a new tree from an array of nodes and the number of nodes.
    pub fn from_nodes(nodes: [Node<L>; MAX_NODES], num_nodes: usize) -> Self {
        assert!(num_nodes <= MAX_NODES, "num_nodes cannot exceed MAX_NODES");
        Tree { nodes, num_nodes }
    }

    /// Returns the root node if the tree is not empty.
    pub fn root(&self) -> Option<&Node<L>> {
        if self.num_nodes > 0 {
            Some(&self.nodes[0])
        } else {
            None
        }
    }

    /// Returns a reference to the node at the given index.
    pub fn get_node(&self, index: usize) -> Option<&Node<L>> {
        if index < self.num_nodes {
            Some(&self.nodes[index])
        } else {
            None
        }
    }

    /// Returns the number of nodes in the tree.
    pub fn len(&self) -> usize {
        self.num_nodes
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.num_nodes == 0
    }

    /// Adds a node to the tree and returns its index.
    /// Returns None if the tree is full.
    pub fn add_node(&mut self, node: Node<L>) -> Option<usize> {
        if self.num_nodes >= MAX_NODES {
            return None;
        }

        let index = self.num_nodes;
        self.nodes[index] = node;
        self.num_nodes += 1;
        Some(index)
    }

    /// Makes a prediction by traversing the decision tree with the given features.
    /// Returns the prediction from the leaf node reached by following the splits.
    pub fn predict(&self, features: &[f32]) -> Result<Option<&L>, TreeError> {

        if self.is_empty() {
            return Ok(None);
        }

        let mut current_index = 0;

        loop {
            let current_node = match self.get_node(current_index) {
                Some(node) => node,
                None => return Err(TreeError::InvalidNodeIndex)
            };

            match current_node {
                // We have reached the end of the tree, return prediction.
                Node::Leaf { prediction } => {
                    return Ok(Some(prediction));
                }

                // Split node. Follow the left child if feature <= threshold, right child otherwise.
                Node::Split { feature_index, threshold, left_child_index, right_child_index, } => {
                    let feature_value = match features.get(*feature_index as usize) {
                        Some(value) => value,
                        None => return Err(TreeError::FeatureIndexOutOfBounds),
                    };

                    // Update index.
                    current_index = if *feature_value <= *threshold {
                        *left_child_index
                    } else {
                        *right_child_index
                    };
                }
            }
        }
    }
}
