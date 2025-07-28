use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseHandlerError {
    #[error("Library database not defined")]
    LibraryNotDefined,
    #[error("User[{0}] database not defined")]
    UserNotDefined(String),
}
