use core::fmt;

pub enum TreeError {
    InvalidNodeIndex,
    FeatureIndexOutOfBounds
}

impl fmt::Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreeError::InvalidNodeIndex => write!(f, "Invalid node index. Tree is malformed."),
            TreeError::FeatureIndexOutOfBounds => write!(f, "Feature index out of bounds."),
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum ArenaError {
    Full,
    // InvalidNode
}

impl fmt::Display for ArenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArenaError::Full => write!(f, "Arena is full"),
            // ArenaError::InvalidNode => write!(f, "Invalid node"),
        }
    }
}