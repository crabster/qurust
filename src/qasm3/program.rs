//! Structures and enums for QASM3 programs representation.
//!
//! A `Program` is composed from blocks, defined by the `blocks::Block` enum.

use crate::qasm3::blocks::Block;
use crate::qasm3::statements::{EmptyLine, VersionDeclaration};
use crate::qasm3::AsQasmStr;

const QASM_VERSION: &str = "3.0";

/// A QASM3 program.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Program {
    blocks: Vec<Block>,
}

impl Program {
    pub fn new(blocks: Vec<Block>) -> Self {
        let mut blocks = blocks;
        let version_prefix: Vec<Block> = vec![
            VersionDeclaration::new(QASM_VERSION.to_string()),
            EmptyLine::new(),
        ];
        blocks.splice(0..0, version_prefix);
        Self { blocks }
    }
}

impl AsQasmStr for Program {
    fn as_qasm3_str(&self) -> String {
        self.blocks
            .iter()
            .map(|block| block.as_qasm3_str())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qasm3::statements::Comment;

    #[test]
    fn test_program() {
        assert_eq!(Program::new(vec![]).as_qasm3_str(), "OPENQASM 3.0;\n");
        assert_eq!(
            Program::new(vec![Comment::new("comment".to_string())]).as_qasm3_str(),
            "OPENQASM 3.0;\n\n// comment"
        );
    }
}
