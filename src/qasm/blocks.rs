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
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<GateDeclaration>>(gate: CustomGate, body: Vec<Statement>) -> T {
        Self { gate, body }.into()
    }
}

impl AsQasmStr for GateDeclaration {
    fn as_qasm_str(&self) -> String {
        let body = self
            .body
            .iter()
            .map(|stmt| {
                let mut stmt_str = stmt.as_qasm_str();
                if !stmt_str.is_empty() {
                    stmt_str = format!("    {}", stmt_str)
                }
                stmt_str
            })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qasm::statements::{Comment, EmptyLine, Statement};

    #[test]
    fn test_gate_declaration() {
        let gate = CustomGate::new("foo".to_string(), vec![], vec![]);
        let body = vec![
            EmptyLine::new::<Statement>(),
            Comment::new::<Statement>("comment".to_string()),
            EmptyLine::new::<Statement>(),
        ];
        assert_eq!(
            GateDeclaration::new::<Block>(gate, body).as_qasm_str(),
            "gate foo {\n\n    // comment\n\n}"
        );
    }
}
