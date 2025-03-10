use std::fmt::Display;

#[derive(Debug)]
pub struct RegexError(pub rand_regex::Error);

#[derive(Debug)]
pub enum GenError {
    FsError(std::io::Error),
    SerdeError(serde_json::Error),
    ParseError(RegexError),
}

impl From<std::io::Error> for GenError {
    fn from(error: std::io::Error) -> Self {
        GenError::FsError(error)
    }
}

impl From<serde_json::Error> for GenError {
    fn from(error: serde_json::Error) -> Self {
        GenError::SerdeError(error)
    }
}

impl From<RegexError> for GenError {
    fn from(error: RegexError) -> Self {
        GenError::ParseError(error)
    }
}

impl Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenError::FsError(e) => write!(f, "Filesystem error: {}", e),
            GenError::SerdeError(e) => write!(f, "Serde error: {}", e),
            GenError::ParseError(e) => write!(f, "Regex error: {}", e.0.to_string()),
        }
    }
}
