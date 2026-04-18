use crate::tree::Node;
use crate::errors::ArenaError;

struct NodeArena<const N: usize> {
    nodes: [Option<Node>; N],
    len: u16, // number of nodes in the arena (max 65,535).
}

impl <const N: usize> Default for NodeArena<N> {
    fn default() -> Self {
        NodeArena {
            nodes: [None; N],
            len: 0,
        }
    }
}


impl <const N: usize> NodeArena<N> {

    pub fn new() -> Self {
        NodeArena::default()
    }

    /// Allocates a node in the arena and returns its index.
    pub fn alloc(&mut self, node: Node) -> Result<usize, ArenaError>  {

        if self.len as usize >= N {
            return Err(ArenaError::Full);
        }

        let index = self.len as usize;
        self.nodes[self.len as usize] = Some(node);
        self.len += 1;
        
        Ok(index)
    }

    /// Returns a reference to a node in the arena.
    pub fn get(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index).and_then(|option| option.as_ref())
    }

    /// Returns a mutable reference to a node in the arena.
    pub fn get_mut(&mut self, index: usize) ->  Option<&mut Node> {
        self.nodes.get_mut(index).and_then(|option| option.as_mut())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Node;
    use crate::tree::Node::Leaf;


    // Initialization

    #[test]
    fn test_new_arena_is_empty() {
        let mut arena: NodeArena<5> = NodeArena::new();
        assert!(arena.get(0).is_none());
        assert!(arena.get_mut(0).is_none());
    }

    #[test]
    fn test_default_equals_new() {
        let arena1: NodeArena<5> = NodeArena::new();
        let arena2: NodeArena<5> = NodeArena::default();
        assert_eq!(arena1.len, arena2.len);
    }


    // Allocation

    #[test]
    fn test_alloc_returns_sequential_indices() {
        let mut arena: NodeArena<4> = NodeArena::new();

        let i0 = arena.alloc(Node::Leaf).unwrap();
        let i1 = arena.alloc(Node::Split { feature: 0, threshold: 0.5 }).unwrap();
        let i2 = arena.alloc(Node::Split { feature: 255, threshold: -1.0 }).unwrap();

        assert_eq!([i0, i1, i2], [0, 1, 2]);
        assert_eq!(arena.len, 3);
    }

    #[test]
    fn test_alloc_when_full_returns_error() {
        let mut arena: NodeArena<2> = NodeArena::new();

        arena.alloc(Node::Leaf).unwrap();
        arena.alloc(Node::Leaf).unwrap();

        let result = arena.alloc(Leaf);
        assert_eq!(result, Err(ArenaError::Full));
        assert_eq!(arena.len, 2); // len shouldn't change
    }
    
}