pub mod types;
pub mod error;

// Without re-exports, users would need to write core::types::SomeType instead of just core::SomeType. Re-exports simplify the API by flattening the module hierarchy. The * means "everything public" from that module.
pub use types::*;
pub use error::*;