use thiserror::Error;

/// Error type for rendering failures.
#[derive(Debug, Error)]
pub enum Error {
    /// Attempted to write an attribute when not in TagOpened state.
    #[error("attempted to write an attribute outside of a tag")]
    AttributeOutsideTag,

    /// Attempted to close an element when no element is open.
    #[error("attempted to close an element when no element is open")]
    NoElementToClose,

    /// Attempted to write content to a void element.
    #[error("attempted to write content to a void element")]
    ContentInVoidElement,
}
