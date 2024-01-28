use crate::qasm3::expressions::Expression;
use crate::qasm3::gates::Gate;
use crate::qasm3::types::Type;
use crate::qasm3::AsQasmStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EmptyLine {}

/// QASM3 empty line statement.
impl EmptyLine {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<EmptyLine>>() -> T {
        Self {}.into()
    }
}

impl AsQasmStr for EmptyLine {
    fn as_qasm3_str(&self) -> String {
        "".to_string()
    }
}

/// QASM3 comment statement.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Comment {
    comment: String,
}

impl Comment {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Comment>>(comment: String) -> T {
        Self { comment }.into()
    }
}

impl AsQasmStr for Comment {
    fn as_qasm3_str(&self) -> String {
        format!("// {}", self.comment)
    }
}

/// QASM3 version declaration statement.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VersionDeclaration {
    version: String,
}

impl VersionDeclaration {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<VersionDeclaration>>(version: String) -> T {
        Self { version }.into()
    }
}

impl AsQasmStr for VersionDeclaration {
    fn as_qasm3_str(&self) -> String {
        format!("OPENQASM {};", self.version)
    }
}

/// QASM3 variable declaration statement.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct VariableDeclaration {
    type_: Type,
    name: String,
    expr: Option<Expression>,
}

impl VariableDeclaration {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<VariableDeclaration>>(
        type_: Type,
        name: String,
        expr: Option<Expression>,
    ) -> T {
        Self { type_, name, expr }.into()
    }
}

impl AsQasmStr for VariableDeclaration {
    fn as_qasm3_str(&self) -> String {
        let var_decl = format!("{} {}", self.type_.as_qasm3_str(), self.name);
        match &self.expr {
            Some(expr) => format!("{} = {};", var_decl, expr.as_qasm3_str()),
            None => format!("{};", var_decl),
        }
    }
}

/// QASM3 variable assignment statement.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct VariableAssignment {
    left_expr: Expression,
    right_expr: Expression,
}

impl VariableAssignment {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<VariableAssignment>>(left_expr: Expression, right_expr: Expression) -> T {
        Self {
            left_expr,
            right_expr,
        }
        .into()
    }
}

impl AsQasmStr for VariableAssignment {
    fn as_qasm3_str(&self) -> String {
        format!(
            "{} = {};",
            self.left_expr.as_qasm3_str(),
            self.right_expr.as_qasm3_str()
        )
    }
}

/// QASM3 gate application statement.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct GateApplication {
    gate: Gate,
}

impl GateApplication {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<GateApplication>>(gate: Gate) -> T {
        Self { gate }.into()
    }
}

impl AsQasmStr for GateApplication {
    fn as_qasm3_str(&self) -> String {
        format!("{};", self.gate.as_qasm3_str())
    }
}

/// QASM3 statement.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Statement {
    EmptyLine(EmptyLine),
    Comment(Comment),
    VersionDeclaration(VersionDeclaration),
    VariableDeclaration(VariableDeclaration),
    VariableAssignment(VariableAssignment),
    GateApplication(GateApplication),
}

impl AsQasmStr for Statement {
    fn as_qasm3_str(&self) -> String {
        match self {
            Self::EmptyLine(empty_line) => empty_line.as_qasm3_str(),
            Self::Comment(comment) => comment.as_qasm3_str(),
            Self::VersionDeclaration(version_declaration) => version_declaration.as_qasm3_str(),
            Self::VariableDeclaration(variable_declaration) => variable_declaration.as_qasm3_str(),
            Self::VariableAssignment(variable_assignment) => variable_assignment.as_qasm3_str(),
            Self::GateApplication(gate_application) => gate_application.as_qasm3_str(),
        }
    }
}

impl From<EmptyLine> for Statement {
    fn from(empty_line: EmptyLine) -> Self {
        Statement::EmptyLine(empty_line)
    }
}

impl From<Comment> for Statement {
    fn from(comment: Comment) -> Statement {
        Statement::Comment(comment)
    }
}

impl From<VersionDeclaration> for Statement {
    fn from(version_declaration: VersionDeclaration) -> Statement {
        Statement::VersionDeclaration(version_declaration)
    }
}

impl From<VariableDeclaration> for Statement {
    fn from(variable_declaration: VariableDeclaration) -> Statement {
        Statement::VariableDeclaration(variable_declaration)
    }
}

impl From<VariableAssignment> for Statement {
    fn from(variable_assignment: VariableAssignment) -> Statement {
        Statement::VariableAssignment(variable_assignment)
    }
}

impl From<GateApplication> for Statement {
    fn from(gate_application: GateApplication) -> Statement {
        Statement::GateApplication(gate_application)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qasm3::expressions::Literal;
    use crate::qasm3::gates::CustomGate;
    use crate::qasm3::types::Primitive;

    #[test]
    fn empty_line_as_qasm3_str() {
        assert_eq!(EmptyLine::new::<Statement>().as_qasm3_str(), "");
    }

    #[test]
    fn comment_as_qasm3_str() {
        assert_eq!(
            Comment::new::<Statement>("comment".to_string()).as_qasm3_str(),
            "// comment"
        );
    }

    #[test]
    fn version_declaration_as_qasm3_str() {
        assert_eq!(
            VersionDeclaration::new::<Statement>("3.0".to_string()).as_qasm3_str(),
            "OPENQASM 3.0;"
        );
    }

    #[test]
    fn variable_declaration_as_qasm3_str() {
        assert_eq!(
            VariableDeclaration::new::<Statement>(Primitive::Bit.into(), "a".to_string(), None)
                .as_qasm3_str(),
            "bit a;"
        );
        assert_eq!(
            VariableDeclaration::new::<Statement>(
                Primitive::Bit.into(),
                "a".to_string(),
                Some(Literal::Bit(true).into())
            )
            .as_qasm3_str(),
            "bit a = 1;"
        );
    }

    #[test]
    fn variable_assignment_as_qasm3_str() {
        assert_eq!(
            VariableAssignment::new::<Statement>("a".to_string().into(), Literal::Bit(true).into())
                .as_qasm3_str(),
            "a = 1;"
        );
    }

    #[test]
    fn gate_application_as_qasm3_str() {
        assert_eq!(
            GateApplication::new::<Statement>(CustomGate::new(
                "gh".to_string(),
                vec!["l1".to_string().into(), "l2".to_string().into()],
                vec!["q1".to_string().into(), "q2".to_string().into()]
            ))
            .as_qasm3_str(),
            "gh(l1, l2) q1 q2;"
        );
    }
}
