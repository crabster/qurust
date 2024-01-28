use crate::qasm3::gates::CustomGate;
use crate::qasm3::statements::*;
use crate::qasm3::AsQasmStr;

/// QASM3 gate declaration block.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
    fn as_qasm3_str(&self) -> String {
        let body = self
            .body
            .iter()
            .map(|stmt| {
                let mut stmt_str = stmt.as_qasm3_str();
                if !stmt_str.is_empty() {
                    stmt_str = format!("    {}", stmt_str)
                }
                stmt_str
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("gate {} {{\n{}\n}}", self.gate.as_qasm3_str(), body)
    }
}

/// QASM3 block.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Block {
    GateDeclaration(GateDeclaration),
    Statement(Statement),
}

impl AsQasmStr for Block {
    fn as_qasm3_str(&self) -> String {
        match self {
            Block::GateDeclaration(gate) => gate.as_qasm3_str(),
            Block::Statement(stmt) => stmt.as_qasm3_str(),
        }
    }
}

impl From<GateDeclaration> for Block {
    fn from(gate: GateDeclaration) -> Block {
        Block::GateDeclaration(gate)
    }
}

impl From<EmptyLine> for Block {
    fn from(empty_line: EmptyLine) -> Block {
        Block::Statement(empty_line.into())
    }
}

impl From<Comment> for Block {
    fn from(comment: Comment) -> Block {
        Block::Statement(comment.into())
    }
}

impl From<VersionDeclaration> for Block {
    fn from(version_declaration: VersionDeclaration) -> Block {
        Block::Statement(version_declaration.into())
    }
}

impl From<VariableDeclaration> for Block {
    fn from(variable_declaration: VariableDeclaration) -> Block {
        Block::Statement(variable_declaration.into())
    }
}

impl From<VariableAssignment> for Block {
    fn from(variable_assignment: VariableAssignment) -> Block {
        Block::Statement(variable_assignment.into())
    }
}

impl From<GateApplication> for Block {
    fn from(gate_application: GateApplication) -> Block {
        Block::Statement(gate_application.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qasm3::statements::{Comment, EmptyLine, Statement};

    #[test]
    fn test_gate_declaration() {
        let gate = CustomGate::new("foo".to_string(), vec![], vec![]);
        let body = vec![
            EmptyLine::new::<Statement>(),
            Comment::new::<Statement>("comment".to_string()),
            EmptyLine::new::<Statement>(),
        ];
        assert_eq!(
            GateDeclaration::new::<Block>(gate, body).as_qasm3_str(),
            "gate foo {\n\n    // comment\n\n}"
        );
    }
}
