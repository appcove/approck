//! Goals:
//! * Access to the original error.
//! * Ability to propagate almost anything with just a question mark. (To be able to convert any
//! kind of error into this error type).
//! * Most of the time in application code we don't care about precisely what the error is, however
//! we may care about certain properties of the error (is it transient, etc).
//! * Be able to attach information about the original error to the error type for later analysis
//! or logging.
//!   * Not necessarily the original type/value of the error, it could be just the error message
//!   (output of it's std::fmt::Display).

mod macros;

#[derive(Debug)]
pub enum ErrorKind {
    Authentication,
    Authorization,
    DataNotFound,
    InvalidOperation,
    ProcessError,
    Unexpected,
    Validation,
    WebsocketCommunication,
    WebsocketUpgrade,
}

pub struct Error {
    pub kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    location: &'static std::panic::Location<'static>,
    context: Vec<Box<dyn std::fmt::Display + Send + Sync>>,
    is_transient: Option<bool>,
    pub code: Option<String>,
    uri: Option<String>,
}

// LUKE: how do we automatically convert from a variety of types, like PgError, or IntOverflowError, type things?
impl Error {
    #[track_caller]
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            source: None,
            location: std::panic::Location::caller(),
            context: Vec::new(),
            is_transient: None,
            code: None,
            uri: None,
        }
    }

    pub fn add_context<C>(mut self, context: C) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.context.push(Box::new(context));
        self
    }

    pub fn set_transient(mut self, is_transient: bool) -> Self {
        self.is_transient = Some(is_transient);
        self
    }

    pub fn set_uri(mut self, uri: String) -> Self {
        self.uri = Some(uri);
        self
    }

    /// Convert this into a [`StdError`]
    pub fn into_std(self) -> StdError {
        self.into()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:#?}"))
    }
}

// Can't be implemented because we want to implement a generic From, which conflicts with the
// standard library generic implementations of From for std::error::Error
//
// impl std::error::Error for Error {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match &self.source {
//             Some(source) => Some(&**source),
//             None => None,
//         }
//     }
// }

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("source", &self.source)
            .field("uri", &self.uri)
            .field("location", &self.location)
            .field(
                "context",
                &self
                    .context
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            )
            .field("is_transient", &self.is_transient)
            .field("code", &self.code)
            .finish()
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn from(value: E) -> Self {
        Self {
            kind: ErrorKind::Unexpected,
            source: Some(Box::new(value)),
            location: std::panic::Location::caller(),
            context: Vec::new(),
            is_transient: None,
            code: None,
            uri: None,
        }
    }
}

/// A wrapper for [`Error`] which implements [`std::error::Error`].
pub struct StdError(pub Error);

impl From<Error> for StdError {
    fn from(value: Error) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for StdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0.source {
            Some(source) => Some(&**source),
            None => None,
        }
    }
}

pub trait ResultExt<T> {
    /// Calling on a `Result<T,E>` type, or a `Result<T, approck::Error>` type, allows you to add context to the error:
    /// * `any_result().amend(|e| e.add_context("my context"))`
    /// * if called on a non `approck::Error` result, the original `E` will be converted into `approck::Error`
    /// * if called on a `approck::Error` result, the closure will be called with same
    /// * the return of the closure will be the final error propagated
    /// * because all of the add_* and set_* methods on self::Error return self, you can chain them
    ///
    /// For example:  
    /// `.amend(|e| e.add_context("my context").set_transient(true))`
    fn amend<CLOSURE>(self, closure: CLOSURE) -> Result<T>
    where
        CLOSURE: Fn(Error) -> Error;
}

/// Implement the Context trait for std::result::Result, before it is converted to Result<self::Error> type
impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn amend<CLOSURE>(self, closure: CLOSURE) -> Result<T>
    where
        CLOSURE: Fn(Error) -> Error,
    {
        self.map_err(|e| closure(Error::from(e)))
    }
}

/// Implement the Context trait for our own Result<Error> type, after it has been converted to Result<self::Error> type
impl<T> ResultExt<T> for self::Result<T> {
    fn amend<CLOSURE>(self, closure: CLOSURE) -> Result<T>
    where
        CLOSURE: Fn(Error) -> Error,
    {
        self.map_err(closure)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use crate::ResultExt;

    use super::{Error, Result};

    fn using_error() -> Result<()> {
        std::fs::read("does_not_exist.txt").map_err(|e| Error::from(e).set_transient(true))?;
        Ok(())
    }

    fn using_using_error() -> Result<()> {
        using_error().amend(|e| e.add_context("my context"))?;

        Ok(())
    }

    #[test]
    fn test_using_error() {
        println!("{:#?}", using_using_error());
    }
}
