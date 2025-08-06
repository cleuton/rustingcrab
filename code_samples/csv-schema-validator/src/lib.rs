// csv-schema-validator/src/lib.rs
pub use csv_schema_validator_derive::ValidateCsv;

// Reexportações para conveniência
pub use serde;
pub use csv;

// --- Adicionado: Reexportações privadas para uso pelo código gerado da macro ---
#[doc(hidden)]
pub mod __private {
    pub use once_cell;
    pub use regex;
}
// --- Fim da adição ---

// Estrutura de erro
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

// Trait opcional (pode ser só o método gerado)
// pub trait ValidateCsv {
//     fn validate_csv(&self) -> Result<(), Vec<ValidationError>>;
// }
// A trait não é estritamente necessária se o método é gerado diretamente.
// Foi comentada como no exemplo original.