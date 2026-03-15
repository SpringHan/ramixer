// Error handle

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("[AppError/IO]: {0}")]
    IO(#[from] std::io::Error),

    #[error("[AppError]: {0}")]
    CreateString(#[from] std::string::FromUtf8Error),

    #[error("[AppError/ParseInt]: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("[AppError]: {0}")]
    Other(#[from] anyhow::Error),

    #[error("[AppError]: {0}")]
    Custom(String),
}
