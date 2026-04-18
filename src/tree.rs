
// Need to think on how to separate classification and regression trees using rust features.
// Node isnt final.


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Node {
    Leaf,
    Split {
        feature: u8,
        threshold: f32,
    },
}

impl Default for Node{
    fn default() -> Self {
        Node::Leaf
    }
}


pub struct Tree {
    // todo
}
