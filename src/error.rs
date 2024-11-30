pub type Result<T> = std::result::Result<T, RError>;

#[derive(Debug)]
pub enum RError {
    RParseError,
}
