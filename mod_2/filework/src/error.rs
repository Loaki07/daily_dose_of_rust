#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Message(&'static str),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Message(e)
    }
}