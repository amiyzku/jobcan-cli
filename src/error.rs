#[derive(Debug, thiserror::Error)]
pub enum JobcanError {
    #[error("Login authentication failed")]
    AuthError,

    #[error("{message}({url}): details {raw_error}")]
    ReqwestError {
        message: String,
        url: String,
        raw_error: reqwest::Error,
    },

    #[error("{message}")]
    UnexpectedResponseError { message: String },

    #[error("{message}")]
    ElementExtractError { message: String },
}
