use core::fmt;

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