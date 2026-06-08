use std::fmt;
use std::io;
use std::num::ParseIntError;

/// Application-level error type that unifies all error sources.
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Validation(String),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {e}"),
            AppError::Parse(e) => write!(f, "parse error: {e}"),
            AppError::Validation(msg) => write!(f, "validation error: {msg}"),
            AppError::NotFound(item) => write!(f, "not found: {item}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io_error() {
        let err = AppError::Io(io::Error::new(io::ErrorKind::NotFound, "gone"));
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn display_parse_error() {
        let inner: ParseIntError = "abc".parse::<i32>().unwrap_err();
        let err = AppError::Parse(inner);
        assert!(err.to_string().contains("parse error"));
    }

    #[test]
    fn display_validation_error() {
        let err = AppError::Validation("bad input".into());
        assert!(err.to_string().contains("validation error: bad input"));
    }

    #[test]
    fn display_not_found_error() {
        let err = AppError::NotFound("user 42".into());
        assert!(err.to_string().contains("not found: user 42"));
    }

    #[test]
    fn from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Io(_)));
    }

    #[test]
    fn from_parse_error() {
        let parse_err: ParseIntError = "xyz".parse::<i32>().unwrap_err();
        let app_err: AppError = parse_err.into();
        assert!(matches!(app_err, AppError::Parse(_)));
    }

    #[test]
    fn source_chain() {
        use std::error::Error;

        let io_err = io::Error::new(io::ErrorKind::Other, "inner");
        let app_err = AppError::Io(io_err);
        assert!(app_err.source().is_some());

        let val_err = AppError::Validation("msg".into());
        assert!(val_err.source().is_none());
    }
}
