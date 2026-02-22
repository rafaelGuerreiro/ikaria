use std::error::Error as StdError;
use thiserror::Error;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("User: {0}")]
    User(String),
    #[error("Internal: {0}")]
    Internal(String),
    #[error("Server: {0}")]
    Server(String),
}

impl ClientError {
    pub fn user(message: impl Into<String>) -> Self {
        ClientError::User(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        ClientError::Internal(message.into())
    }

    pub fn server(message: impl Into<String>) -> Self {
        ClientError::Server(message.into())
    }
}

impl From<std::io::Error> for ClientError {
    fn from(error: std::io::Error) -> Self {
        ClientError::internal(error.to_string())
    }
}

pub trait ErrorMapper {
    fn map_user_error(self) -> ClientError;
    fn map_internal_error(self) -> ClientError;
    fn map_server_error(self) -> ClientError;
}

impl<E> ErrorMapper for E
where
    E: StdError + Send + Sync + 'static,
{
    fn map_user_error(self) -> ClientError {
        ClientError::User(self.to_string())
    }

    fn map_internal_error(self) -> ClientError {
        ClientError::Internal(self.to_string())
    }

    fn map_server_error(self) -> ClientError {
        ClientError::Server(self.to_string())
    }
}

pub trait ResultExt<T, E: StdError + Send + Sync + 'static> {
    fn map_user_error(self) -> ClientResult<T>;
    fn map_internal_error(self) -> ClientResult<T>;
    fn map_server_error(self) -> ClientResult<T>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn map_user_error(self) -> ClientResult<T> {
        self.map_err(|e| e.map_user_error())
    }

    fn map_internal_error(self) -> ClientResult<T> {
        self.map_err(|e| e.map_internal_error())
    }

    fn map_server_error(self) -> ClientResult<T> {
        self.map_err(|e| e.map_server_error())
    }
}
