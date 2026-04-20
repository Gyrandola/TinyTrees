use crate::node::Node;

/// TreeBuffer is a mutable array-based tree used during training.
pub struct TrainingTree<L, const MAX_NODES: usize> {
    nodes: [Option<Node<L>>; MAX_NODES],
    num_nodes: usize,
}

impl<L: Copy, const MAX_NODES: usize> TrainingTree<L, MAX_NODES> {

    /// Creates a new empty tree.
    pub fn new() -> Self {
        TrainingTree {
            nodes: [const { None }; MAX_NODES],
            num_nodes: 0,
        }
    }

    /// Returns the root node if the tree is not empty.
    pub fn root(&self) -> Option<&Node<L>> {
        if self.num_nodes > 0 {
            self.nodes[0].as_ref()
        } else {
            None
        }
    }

    /// Returns a reference to the node at the given index.
    pub fn get_node(&self, index: usize) -> Option<&Node<L>> {
        if index < self.num_nodes {
            self.nodes[index].as_ref()
        } else {
            None
        }
    }

    /// Returns a mutable reference to the node at the given index.
    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<L>> {
        if index < self.num_nodes {
            self.nodes[index].as_mut()
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
        self.nodes[index] = Some(node);
        self.num_nodes += 1;
        Some(index)
    }

}

impl<L: Copy, const MAX_NODES: usize> Default for TrainingTree<L, MAX_NODES> {
    fn default() -> Self {
        Self::new()
    }
}