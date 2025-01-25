// Exporta o módulo e reexporta o trait
pub mod activation;  // Declara o módulo
pub use activation::Activation;  // Reexporta o trait
pub mod relu;  // Declara o módulo
pub use relu::ReLU;  // Reexporta a estrutura
pub mod sigmoid;  // Declara o módulo
pub use sigmoid::Sigmoid;  // Reexporta a estrutura
pub mod sinapse;  // Declara o módulo
pub use sinapse::Sinapse;  // Reexporta a estrutura
pub mod node;  // Declara o módulo
pub use node::Node;  // Reexporta a estrutura
pub mod layer;  // Declara o módulo
pub use layer::Layer;  // Reexporta a estrutura
pub mod model;  // Declara o módulo
pub use model::Model;  // Reexporta a estrutura



