use crate::qasm::gates::CustomGate;
use crate::qasm::statements::{Statement, StatementTrait};
use crate::qasm::AsQasmStr;

use std::rc::Rc;

#[derive(Clone)]
pub struct GateDeclaration {
    gate: CustomGate,
    body: Vec<Statement>,
}

impl GateDeclaration {
    pub fn new<T: From<GateDeclaration>>(gate: CustomGate, body: Vec<Statement>) -> T {
        Self { gate, body }.into()
    }
}

impl AsQasmStr for GateDeclaration {
    fn as_qasm_str(&self) -> String {
        let body = self
            .body
            .iter()
            .map(|stmt| format!("    {}", stmt.as_qasm_str()))
            .collect::<Vec<String>>()
            .join("\n");
        format!("gate {} {{\n{}\n}}", self.gate.as_qasm_str(), body)
    }
}

#[derive(Clone)]
pub struct Block {
    block: Rc<dyn BlockTrait>,
}

trait BlockTrait: AsQasmStr {}

impl BlockTrait for GateDeclaration {}
impl<T: StatementTrait> BlockTrait for T {}

impl AsQasmStr for Block {
    fn as_qasm_str(&self) -> String {
        self.block.as_qasm_str()
    }
}

impl<T: BlockTrait + 'static> From<T> for Block {
    fn from(block: T) -> Self {
        Self {
            block: Rc::new(block),
        }
    }
}
