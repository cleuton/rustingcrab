pub use csv_schema_validator_derive::ValidateCsv;

pub use serde;
pub use csv;

#[doc(hidden)]
pub mod __private {
    pub use once_cell;
    pub use regex;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

