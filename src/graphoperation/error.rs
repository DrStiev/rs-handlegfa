/// define a custom error type for the program
use std::{error, fmt};

pub type GraphOperationResult<T> = Result<T, GraphOperationError>;

#[derive(Debug)]
pub enum GraphOperationError {
    FileError(String),
    IdAlreadyExist(String),
    NodesNotExist(Vec<String>),
    OrientationNotExists(String),
    Unknown,  
}

impl fmt::Display for GraphOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphOperationError as GE;
        match self {
            GE::FileError(file) => write!(f, "{}", file),
            GE::IdAlreadyExist(id) => write!(f, "The Id provided ({}) already exists", id),
            GE::NodesNotExist(nodeid) => write!(f, "Cannot find the node(s): {} {}", nodeid[0], nodeid[1]),
            GE::OrientationNotExists(orientation) => write!(f, "Segment reference Id ({}) did not include orientation", orientation),
            GE::Unknown => write!(f, "Unknown error while operating on the graph"),
        }
    }
}

impl error::Error for GraphOperationError {}

