use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("入出力エラー: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("設定エラー: {0}")]
    ConfigError(String),
    
    #[error("認証エラー: {0}")]
    AuthError(String),
    
    #[error("ネットワークエラー: {0}")]
    NetworkError(String),
    
    #[error("不明なエラー: {0}")]
    Unknown(String),
}

pub fn config_error<T: Into<String>>(message: T) -> CliError {
    CliError::ConfigError(message.into())
}

pub fn auth_error<T: Into<String>>(message: T) -> CliError {
    CliError::AuthError(message.into())
}

pub fn network_error<T: Into<String>>(message: T) -> CliError {
    CliError::NetworkError(message.into())
}

pub fn unknown_error<T: Into<String>>(message: T) -> CliError {
    CliError::Unknown(message.into())
}