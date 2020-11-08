/// define a custom error type for the program
use std::{error, fmt};

pub type GraphOperationResult<T> = Result<T, GraphOperationError>;

#[derive(Debug)]
pub enum GraphOperationError {
    FileError(String),
    IdAlreadyExist(String),
    NodesNotExist(String, String),
    EdgeNotExist(String, String),
    PathNotExist(String),
    OrientationNotExists(String),
    Unknown,
}

impl fmt::Display for GraphOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphOperationError as GE;
        match self {
            GE::FileError(file) => write!(f, "{}", file),
            GE::IdAlreadyExist(id) => {
                write!(f, "The Id provided ({}) already exists", id)
            }
            GE::NodesNotExist(node_left, node_right) => write!(
                f,
                "Cannot find the node(s): {} {}",
                node_left, node_right
            ),
            GE::EdgeNotExist(l, r) => {
                write!(f, "The Edge ({} -> {}) did not exist", l, r)
            }
            GE::PathNotExist(path) => {
                write!(f, "The Path ({}) did not exist", path)
            }
            GE::OrientationNotExists(orientation) => write!(
                f,
                "Segment reference Id ({}) did not include orientation",
                orientation
            ),
            GE::Unknown => {
                write!(f, "Unknown error while operating on the graph")
            }
        }
    }
}

impl error::Error for GraphOperationError {}