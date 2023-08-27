use thiserror::Error;
pub type AGResult<T> = Result<T, AGError>;

#[derive(Error, Debug)]
pub enum AGError {
    #[error("io Error:{0}")]
    Io(#[from] std::io::Error),
    #[error("decode failed")]
    Decode,
    #[error("Image Error:{0}")]
    Image(#[from] image::error::ImageError),
    #[error("Custom Error:{0}")]
    Custom(String),
}
