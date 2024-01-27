use crate::qasm::expressions::Expression;
use crate::qasm::gates::Gate;
use crate::qasm::types::Type;
use crate::qasm::AsQasmStr;

use std::rc::Rc;

#[derive(Clone)]
pub struct EmptyLine {}

impl EmptyLine {
    pub fn new<T: From<EmptyLine>>() -> T {
        Self {}.into()
    }
}

impl AsQasmStr for EmptyLine {
    fn as_qasm_str(&self) -> String {
        "".to_string()
    }
}

#[derive(Clone)]
pub struct Comment {
    comment: String,
}

impl Comment {
    pub fn new<T: From<Comment>>(comment: String) -> T {
        Self { comment }.into()
    }
}

impl AsQasmStr for Comment {
    fn as_qasm_str(&self) -> String {
        format!("// {}", self.comment)
    }
}

#[derive(Clone)]
pub struct VersionDeclaration {
    version: String,
}

impl VersionDeclaration {
    pub fn new<T: From<VersionDeclaration>>(version: String) -> T {
        Self { version }.into()
    }
}

impl AsQasmStr for VersionDeclaration {
    fn as_qasm_str(&self) -> String {
        format!("OPENQASM {};", self.version)
    }
}

#[derive(Clone)]
pub struct VariableDeclaration {
    type_: Type,
    name: String,
    expr: Option<Expression>,
}

impl VariableDeclaration {
    pub fn new<T: From<VariableDeclaration>>(
        type_: Type,
        name: String,
        expr: Option<Expression>,
    ) -> T {
        Self { type_, name, expr }.into()
    }
}

impl AsQasmStr for VariableDeclaration {
    fn as_qasm_str(&self) -> String {
        let var_decl = format!("{} {}", self.type_.as_qasm_str(), self.name);
        match &self.expr {
            Some(expr) => format!("{} = {};", var_decl, expr.as_qasm_str()),
            None => format!("{};", var_decl),
        }
    }
}

#[derive(Clone)]
pub struct VariableAssignment {
    left_expr: Expression,
    right_expr: Expression,
}

impl VariableAssignment {
    pub fn new<T: From<VariableAssignment>>(left_expr: Expression, right_expr: Expression) -> T {
        Self {
            left_expr,
            right_expr,
        }
        .into()
    }
}

impl AsQasmStr for VariableAssignment {
    fn as_qasm_str(&self) -> String {
        format!(
            "{} = {};",
            self.left_expr.as_qasm_str(),
            self.right_expr.as_qasm_str()
        )
    }
}

#[derive(Clone)]
pub struct GateApplication {
    gate: Gate,
}

impl GateApplication {
    pub fn new<T: From<GateApplication>>(gate: Gate) -> T {
        Self { gate }.into()
    }
}

impl AsQasmStr for GateApplication {
    fn as_qasm_str(&self) -> String {
        format!("{};", self.gate.as_qasm_str())
    }
}

#[derive(Clone)]
pub struct Statement {
    stmt: Rc<dyn AsQasmStr>,
}

pub trait StatementTrait: AsQasmStr {}

impl StatementTrait for EmptyLine {}
impl StatementTrait for Comment {}
impl StatementTrait for VersionDeclaration {}
impl StatementTrait for VariableDeclaration {}
impl StatementTrait for VariableAssignment {}
impl StatementTrait for GateApplication {}

impl AsQasmStr for Statement {
    fn as_qasm_str(&self) -> String {
        self.stmt.as_qasm_str()
    }
}

impl<T: StatementTrait + 'static> From<T> for Statement {
    fn from(stmt: T) -> Self {
        Self {
            stmt: Rc::new(stmt),
        }
    }
}
