use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0} Null or Empty")]
    NullError(String),
    #[error("Request failed with error code {0}")]
    RequestFailedError(StatusCode),
    #[error("Error occured in reqwest lib {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    AppError(String),
    #[error("{0}")]
    Html2TextError(#[from] html2text::Error)
}

pub type AppResult<T> = Result<T, Error>;
