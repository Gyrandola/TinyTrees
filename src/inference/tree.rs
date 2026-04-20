use crate::inference::error::InferenceError;
use crate::node::Node;

/// InferenceTree is a slice-based decision tree meant to be used strictly for inference.
pub struct InferenceTree<'a, L> {
    nodes: &'a [Node<L>],
}

impl<'a, L> InferenceTree<'a, L> {
    /// Creates a new inference tree from a slice of nodes.
    pub fn new(nodes: &'a [Node<L>]) -> Self {
        InferenceTree { nodes }
    }

    /// Returns the root node if the tree is not empty.
    pub fn root(&self) -> Option<&Node<L>> {
        self.nodes.first()
    }

    /// Returns a reference to the node at the given index.
    pub fn get_node(&self, index: usize) -> Option<&Node<L>> {
        self.nodes.get(index)
    }

    /// Returns the number of nodes in the tree.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Makes a prediction by traversing the decision tree with the given features.
    /// Returns the prediction from the leaf node reached by following the splits.
    pub fn predict(&self, features: &[f32]) -> Result<Option<&L>, InferenceError> {

        if self.is_empty() {
            return Ok(None);
        }

        let mut current_index = 0;

        loop {
            let current_node = match self.get_node(current_index) {
                Some(node) => node,
                None => return Err(InferenceError::InvalidNodeIndex)
            };

            match current_node {
                // We have reached the end of the tree, return prediction.
                Node::Leaf { prediction } => {
                    return Ok(Some(prediction));
                }

                // If it's a split node, decide which child to follow
                Node::Split { feature_index, threshold, left_child_index, right_child_index, } => {

                    let feature_value = match features.get(*feature_index as usize) {
                        Some(value) => value,
                        None => return Err(InferenceError::FeatureIndexOutOfBounds),
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