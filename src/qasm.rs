pub mod blocks;
pub mod expressions;
pub mod gates;
pub mod program;
pub mod statements;
pub mod types;

pub trait AsQasmStr {
    fn as_qasm_str(&self) -> String;
}
