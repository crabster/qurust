pub mod blocks;
pub mod expressions;
pub mod gates;
pub mod program;
pub mod statements;
pub mod types;

use std::fmt::Debug;

pub trait AsQasmStr: Debug {
    fn as_qasm_str(&self) -> String;
}
