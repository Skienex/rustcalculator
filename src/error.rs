use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid binary operation")]
    InvalidBinOp(),
    #[error("Invalid unary operation")]
    InvalidUnaryOp(),
    #[error("Invalid identifier")]
    InvalidIdent(),
    #[error("Invalid number: {0:?}")]
    InvalidNumber(String),
    #[error("Unexpected character: {0:?}")]
    UnexpectedChar(char),
}
