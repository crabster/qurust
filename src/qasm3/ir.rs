//! Module `ir` contains structures and enums for QASM3 programs representation.
//!
//! To learn about syntax of QASM3 programs, you can refer to the official
//! [QASM3 documentation](https://openqasm.com/versions/3.0/language/types.html) or
//! [QASM3 grammar files](https://github.com/openqasm/openqasm/tree/main/source/grammar).

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
