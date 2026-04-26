#[derive(Debug)]
pub enum CryptoError {
    InvalidToken,
}

impl CryptoError {
    pub fn kind(&self) -> &'static str {
        match self {
            CryptoError::InvalidToken => "Invalid Token",
        }
    }
}
