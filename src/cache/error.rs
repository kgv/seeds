use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("{message}")]
    OpenCV { code: i32, message: String },
}

impl From<opencv::Error> for Error {
    fn from(value: opencv::Error) -> Self {
        Self::OpenCV {
            code: value.code,
            message: value.message,
        }
    }
}
