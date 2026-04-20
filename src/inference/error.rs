use core::fmt;

pub enum InferenceError {
    InvalidNodeIndex,
    FeatureIndexOutOfBounds
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferenceError::InvalidNodeIndex => write!(f, "Invalid node index. Tree is malformed."),
            InferenceError::FeatureIndexOutOfBounds => write!(f, "Feature index out of bounds."),
        }
    }
}
