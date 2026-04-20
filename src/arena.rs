use crate::tree::Node;
use crate::errors::ArenaError;

#[derive(Debug)]
pub struct NodeArena<L, const N: usize> {
    nodes: [Option<Node<L>>; N],
    len: u16, // number of nodes in the arena (max 65,535).
}

impl<L, const N: usize> Default for NodeArena<L, N> {
    fn default() -> Self {

        // Prevent overlow
        assert!(N <= u16::MAX as usize, "Arena initialized with capacity N >= u16::MAX.");

        NodeArena {
            nodes: [const { None }; N],
            len: 0,
        }
    }
}


impl<L, const N: usize> NodeArena<L, N> {

    pub fn new() -> Self {
        Self::default()
    }

    /// Allocates a node in the arena and returns its index.
    pub fn alloc(&mut self, node: Node<L>) -> Result<usize, ArenaError>  {

        if self.len as usize >= N {
            return Err(ArenaError::Full);
        }

        let index = self.len as usize;
        self.nodes[index] = Some(node);
        self.len += 1;
        
        Ok(index)
    }

    /// Returns a reference to a node in the arena.
    pub fn get(&self, index: usize) -> Option<&Node<L>> {
        self.nodes.get(index).and_then(|option| option.as_ref())
    }

    /// Returns a mutable reference to a node in the arena.
    pub fn get_mut(&mut self, index: usize) ->  Option<&mut Node<L>> {
        self.nodes.get_mut(index).and_then(|option| option.as_mut())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;
    use crate::errors::ArenaError;


    #[test]
    fn test_alloc_and_retrieval_integrity() {
        let mut arena: NodeArena<i32, 3> = NodeArena::new();
        let i0 = arena.alloc(Node::new_leaf(10)).unwrap();
        let i1 = arena.alloc(Node::new_leaf(20)).unwrap();
        assert_eq!(i0, 0);
        assert_eq!(i1, 1);
        assert_eq!(arena.len, 2);
        assert_eq!(arena.get(i0).unwrap().prediction, 10);
        assert_eq!(arena.get(i1).unwrap().prediction, 20);
    }

    #[test]
    fn test_mutation_in_place() {
        let mut arena: NodeArena<&str, 2> = NodeArena::new();
        let idx = arena.alloc(Node::new_leaf("initial")).unwrap();
        if let Some(node) = arena.get_mut(idx) {
            node.prediction = "changed";
        }
        assert_eq!(arena.get(idx).unwrap().prediction, "changed");
    }

    #[test]
    fn test_saturation_and_bounds() {
        let mut arena: NodeArena<f32, 2> = NodeArena::new();
        assert!(arena.alloc(Node::new_leaf(1.1)).is_ok());
        assert!(arena.alloc(Node::new_leaf(2.2)).is_ok());
        let result = arena.alloc(Node::new_leaf(3.3));
        assert_eq!(result, Err(ArenaError::Full));
        assert!(arena.get(2).is_none());
        assert!(arena.get(99).is_none());
    }

    #[test]
    fn test_non_copy_label_support() {
        #[derive(Debug, PartialEq)]
        struct Unique(u32);
        let mut arena: NodeArena<Unique, 1> = NodeArena::new();
        let idx = arena.alloc(Node::new_leaf(Unique(42))).unwrap();
        assert_eq!(arena.get(idx).unwrap().prediction, Unique(42));
    }

    #[test]
    fn test_zero_capacity_edge_case() {
        let mut arena: NodeArena<u8, 0> = NodeArena::new();
        assert_eq!(arena.alloc(Node::new_leaf(1)), Err(ArenaError::Full));
    }
}