pub mod error;
pub mod types;

// Without re-exports, users would need to write core::types::SomeType instead of just core::SomeType. Re-exports simplify the API by flattening the module hierarchy. The * means "everything public" from that module.
pub use error::*;
pub use types::*;
