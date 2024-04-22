use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error)]
pub enum Error {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("{message}")]
    OpenCV { code: i32, message: String },
    // #[error(transparent)]
    // Utf8(#[from] Utf8Error),
}

impl From<opencv::Error> for Error {
    fn from(value: opencv::Error) -> Self {
        Self::OpenCV {
            code: value.code,
            message: value.message,
        }
    }
}
