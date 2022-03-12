use crate::Variable;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parsing(String),
    TypeMismatch(Variable, Variable),
}

impl From<std::io::Error> for Error {
    fn from(from: std::io::Error) -> Self {
        Self::Io(from)
    }
}

impl From<nom::error::Error<&str>> for Error {
    fn from(from: nom::error::Error<&str>) -> Self {
        Self::Parsing(from.to_string())
    }
}
