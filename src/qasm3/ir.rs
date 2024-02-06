pub mod expressions;
pub mod program;
pub mod statements;
pub mod types;

pub use expressions::*;
pub use program::*;
pub use statements::*;
pub use types::*;

use std::fmt::Debug;

/// Trait for types convertible to QASM3 string.
pub trait AsQasmStr: Debug {
    fn as_qasm_str(&self) -> String;
}
