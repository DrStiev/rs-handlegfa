/// define a custom error struct
#[derive(Debug)]
pub struct GFAError {
    // TODO: add a way to describe what kind of error has occured
    message: String,
}

impl GFAError {
    fn new(msg: &str) -> GFAError {
        GFAError {
            message: msg.to_string(),
        }
    }
}

/// implement the trait From<nom::Err<(&str, nom::error::ErrorKind)>> for GFAError
/// to use the GFAError struct properly
impl From<nom::Err<(&str, nom::error::ErrorKind)>> for GFAError {
    fn from(err: nom::Err<(&str, nom::error::ErrorKind)>) -> Self {
        GFAError::new(&err.to_string())
    }
}

/// implement the trait From<std::io::Error> for GFAError
/// to use the GFAError struct properly
impl From<std::io::Error> for GFAError {
    fn from(error: std::io::Error) -> Self {
        GFAError::new(&error.to_string())
    }
}

/// implement the display trait for
impl std::fmt::Display for GFAError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}