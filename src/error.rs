#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parsing(String),
}

impl From<std::io::Error> for Error {
    fn from(from: std::io::Error) -> Self {
        Self::Io(from)
    }
}
