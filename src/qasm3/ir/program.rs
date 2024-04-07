//! Structures and enums for QASM3 program representation.
//!
//! To learn more about QASM3 programs, please refer to the official
//! [QASM3 documentation](https://openqasm.com/versions/3.0/language/index.html)
//! and [QASM3 grammar files](https://github.com/openqasm/openqasm/tree/main/source/grammar).

use crate::qasm3::ir::statements::StatementOrScope;
use crate::qasm3::ir::AsQasmStr;

/// QASM3 version declaration.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// QASM3 program representation.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

        if self.version.is_none() {
            format!("{}\n", parts_str)
        } else if parts_str.is_empty() {
            format!("{}\n", self.version.as_ref().unwrap().as_qasm_str())
        } else {
            format!(
                "{}\n{}\n",
                self.version.as_ref().unwrap().as_qasm_str(),
                parts_str
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::qasm3::ir::statements::Statement;

    #[test]
    fn test_version() {
        assert_eq!(
            Version::new("3.0".to_string()).as_qasm_str(),
            "OPENQASM 3.0;"
        );
    }

    #[test]
    fn test_program() {
        assert_eq!(Program::new(None, vec![]).as_qasm_str(), "\n");
        assert_eq!(
            Program::new(Some(Version::new("3.0".to_string())), vec![]).as_qasm_str(),
            "OPENQASM 3.0;\n"
        );
        assert_eq!(
            Program::new(
                Some(Version::new("3.0".to_string())),
                vec![Statement::Break.into()]
            )
            .as_qasm_str(),
            "OPENQASM 3.0;\nbreak;\n"
        );
    }
}
