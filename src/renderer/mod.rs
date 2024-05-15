use libmacchina::traits::ReadoutError;
use thiserror::Error;

pub mod macchina;
pub mod neofetch;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Failed to read config")]
    ReadoutError(ReadoutError),
    #[error("Failed to print")]
    PrintError(#[from] std::io::Error),
}

impl From<ReadoutError> for RendererError {
    fn from(err: ReadoutError) -> Self {
        RendererError::ReadoutError(err)
    }
}
