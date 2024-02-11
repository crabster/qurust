use crate::qasm3::ir::statements::StatementOrScope;
use crate::qasm3::ir::AsQasmStr;

#[derive(Debug)]
pub struct Version {
    tag: String,
}

impl Version {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }
}

impl AsQasmStr for Version {
    fn as_qasm_str(&self) -> String {
        format!("OPENQASM {};", self.tag)
    }
}

#[derive(Debug)]
pub struct Program {
    version: Option<Version>,
    parts: Vec<StatementOrScope>,
}

impl Program {
    pub fn new(version: Option<Version>, parts: Vec<StatementOrScope>) -> Self {
        Self { version, parts }
    }
}

impl AsQasmStr for Program {
    fn as_qasm_str(&self) -> String {
        let parts_str = self
            .parts
            .iter()
            .map(|p| p.as_qasm_str())
            .collect::<Vec<_>>()
            .join("\n");
        match &self.version {
            Some(version) => format!("{}\n\n{}", version.as_qasm_str(), parts_str),
            None => parts_str,
        }
    }
}
