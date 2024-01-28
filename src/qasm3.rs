pub mod blocks;
pub mod expressions;
pub mod gates;
pub mod program;
pub mod statements;
pub mod types;

use std::fmt::Debug;

/// Trait for types convertible to QASM3 string.
pub trait AsQasmStr: Debug {
    fn as_qasm3_str(&self) -> String;
}
