use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(unused)]
pub struct ErrorMessage(pub String);

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorMessage {}

#[derive(Debug)]
pub struct ErrorContext(pub String, pub Box<dyn std::error::Error>);

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.1)
    }
}
