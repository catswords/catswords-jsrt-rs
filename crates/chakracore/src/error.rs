use chakracore_sys::JsErrorCode;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(pub JsErrorCode);

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn ok(code: JsErrorCode) -> Result<()> {
    if code == JsErrorCode::JsNoError {
        Ok(())
    } else {
        Err(Error(code))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChakraCore JsRT error: {:?}", self.0)
    }
}

impl std::error::Error for Error {}
