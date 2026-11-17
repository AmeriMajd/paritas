/// The normalized authorization result of invoking an operation.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allowed,
    Denied,
    Unknown,
    Error,
}
