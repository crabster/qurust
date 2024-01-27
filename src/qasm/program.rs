use crate::qasm::blocks::Block;
use crate::qasm::statements::{EmptyLine, VersionDeclaration};
use crate::qasm::AsQasmStr;

const QASM_VERSION: &str = "3.0";

pub struct Program {
    blocks: Vec<Block>,
}

impl Program {
    pub fn new(blocks: Vec<Block>) -> Self {
        let mut blocks = blocks;
        let mut version_prefix: Vec<Block> = vec![];
        version_prefix.push(VersionDeclaration::new(QASM_VERSION.to_string()));
        version_prefix.push(EmptyLine::new());
        blocks.splice(0..0, version_prefix);
        Self { blocks }
    }
}

impl AsQasmStr for Program {
    fn as_qasm_str(&self) -> String {
        self.blocks
            .iter()
            .map(|block| block.as_qasm_str())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
