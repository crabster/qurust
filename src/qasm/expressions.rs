use crate::qasm::AsQasmStr;

use std::rc::Rc;

pub type Identifier = String;

impl AsQasmStr for Identifier {
    fn as_qasm_str(&self) -> String {
        self.clone()
    }
}

#[derive(Clone)]
pub enum Literal {
    Bit(bool),
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    Pi,
    Complex((f64, f64)),
}

impl AsQasmStr for Literal {
    fn as_qasm_str(&self) -> String {
        match self {
            Literal::Bit(lit) => {
                if *lit {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
            Literal::Bool(lit) => lit.to_string(),
            Literal::Int(lit) => lit.to_string(),
            Literal::Uint(lit) => lit.to_string(),
            Literal::Float(lit) => lit.to_string(),
            Literal::Pi => "pi".to_string(),
            Literal::Complex(lit) => format!("({}) + ({}im)", lit.0, lit.1),
        }
    }
}

pub type Array = Vec<Expression>;

impl AsQasmStr for Array {
    fn as_qasm_str(&self) -> String {
        format!(
            "{{{}}}",
            self.iter()
                .map(|e| e.as_qasm_str())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Clone)]
pub struct ArrayAccess {
    identifier: Identifier,
    indices: Vec<isize>,
}

impl ArrayAccess {
    pub fn new<T: From<ArrayAccess>>(identifier: Identifier, indices: Vec<isize>) -> T {
        Self {
            identifier,
            indices,
        }
        .into()
    }
}

impl AsQasmStr for ArrayAccess {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}[{}]",
            self.identifier,
            self.indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Clone)]
pub struct Measurement {
    expression: Expression,
}

impl Measurement {
    pub fn new<T: From<Measurement>>(expression: Expression) -> T {
        Self { expression }.into()
    }
}

impl AsQasmStr for Measurement {
    fn as_qasm_str(&self) -> String {
        format!("measure {}", self.expression.as_qasm_str())
    }
}

#[derive(Clone)]
enum BinOpType {
    Plus,
    Minus,
    Times,
    Div,
}

impl AsQasmStr for BinOpType {
    fn as_qasm_str(&self) -> String {
        match self {
            BinOpType::Plus => "+".to_string(),
            BinOpType::Minus => "-".to_string(),
            BinOpType::Times => "*".to_string(),
            BinOpType::Div => "/".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct BinOp {
    op: BinOpType,
    lhs: Expression,
    rhs: Expression,
}

impl BinOp {
    fn new<T: From<BinOp>>(op: BinOpType, lhs: Expression, rhs: Expression) -> T {
        Self { op, lhs, rhs }.into()
    }
}

impl AsQasmStr for BinOp {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}{}{}",
            self.lhs.as_qasm_str(),
            self.op.as_qasm_str(),
            self.rhs.as_qasm_str()
        )
    }
}

pub struct PlusOp(BinOp);

impl PlusOp {
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Plus, lhs, rhs).into()
    }
}

pub struct MinusOp(BinOp);

impl MinusOp {
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Minus, lhs, rhs).into()
    }
}

pub struct TimesOp(BinOp);

impl TimesOp {
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Times, lhs, rhs).into()
    }
}

pub struct DivOp(BinOp);

impl DivOp {
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Div, lhs, rhs).into()
    }
}

trait ExpressionTrait: AsQasmStr {}

impl ExpressionTrait for Literal {}
impl ExpressionTrait for Identifier {}
impl ExpressionTrait for Array {}
impl ExpressionTrait for ArrayAccess {}
impl ExpressionTrait for Measurement {}
impl ExpressionTrait for BinOp {}

pub struct Expression {
    expression: Rc<dyn ExpressionTrait>,
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        Self {
            expression: self.expression.clone(),
        }
    }
}

impl AsQasmStr for Expression {
    fn as_qasm_str(&self) -> String {
        self.expression.as_qasm_str()
    }
}

impl<T: ExpressionTrait + 'static> From<T> for Expression {
    fn from(expression: T) -> Self {
        Self {
            expression: Rc::new(expression),
        }
    }
}
