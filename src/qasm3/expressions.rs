//! Structures and enums for QASM3 expressions representation.
//!
//! Expressions represent all values used in QASM3 programs. They are defined by the `Expression` enum.

use crate::qasm3::AsQasmStr;

/// QASM3 identifier expression.
pub type Identifier = String;

impl AsQasmStr for Identifier {
    fn as_qasm3_str(&self) -> String {
        self.clone()
    }
}

/// QASM3 literal expression.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
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
    fn as_qasm3_str(&self) -> String {
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
            Literal::Complex(lit) => match lit.1.signum() == 1.0 {
                true => format!("{} + {}im", lit.0, lit.1),
                false => format!("{} - {}im", lit.0, lit.1.abs()),
            },
        }
    }
}

/// A QASM3 array expression.
pub type Array = Vec<Expression>;

impl AsQasmStr for Array {
    fn as_qasm3_str(&self) -> String {
        format!(
            "{{{}}}",
            self.iter()
                .map(|e| e.as_qasm3_str())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// QASM3 array access expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ArrayAccess {
    identifier: Identifier,
    indices: Vec<isize>,
}

impl ArrayAccess {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<ArrayAccess>>(identifier: Identifier, indices: Vec<isize>) -> T {
        Self {
            identifier,
            indices,
        }
        .into()
    }
}

impl AsQasmStr for ArrayAccess {
    fn as_qasm3_str(&self) -> String {
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

/// QASM3 measurement expression.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Measurement {
    expr: Box<Expression>,
}

impl Measurement {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Measurement>>(expr: Expression) -> T {
        let expr = Box::new(expr);
        Self { expr }.into()
    }
}

impl AsQasmStr for Measurement {
    fn as_qasm3_str(&self) -> String {
        format!("measure {}", self.expr.as_qasm3_str())
    }
}

/// QASM3 binary operator type;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum BinOpType {
    Plus,
    Minus,
    Times,
    Div,
}

impl AsQasmStr for BinOpType {
    fn as_qasm3_str(&self) -> String {
        match self {
            BinOpType::Plus => "+".to_string(),
            BinOpType::Minus => "-".to_string(),
            BinOpType::Times => "*".to_string(),
            BinOpType::Div => "/".to_string(),
        }
    }
}

/// QASM3 binary operation expression.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct BinOp {
    op: BinOpType,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl BinOp {
    #[allow(clippy::new_ret_no_self)]
    fn new<T: From<BinOp>>(op: BinOpType, lhs: Expression, rhs: Expression) -> T {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        Self { op, lhs, rhs }.into()
    }
}

impl AsQasmStr for BinOp {
    fn as_qasm3_str(&self) -> String {
        format!(
            "{} {} {}",
            self.lhs.as_qasm3_str(),
            self.op.as_qasm3_str(),
            self.rhs.as_qasm3_str()
        )
    }
}

/// QASM3 addition expression.
pub struct PlusOp(BinOp);

impl PlusOp {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Plus, lhs, rhs)
    }
}

/// QASM3 subtraction expression.
pub struct MinusOp(BinOp);

impl MinusOp {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Minus, lhs, rhs)
    }
}

/// QASM3 multiplication expression.
pub struct TimesOp(BinOp);

impl TimesOp {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Times, lhs, rhs)
    }
}

/// QASM3 division expression.
pub struct DivOp(BinOp);

impl DivOp {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<BinOp>>(lhs: Expression, rhs: Expression) -> T {
        BinOp::new::<T>(BinOpType::Div, lhs, rhs)
    }
}

/// QASM3 expression.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    Array(Array),
    ArrayAccess(ArrayAccess),
    Measurement(Measurement),
    BinOp(BinOp),
}

impl AsQasmStr for Expression {
    fn as_qasm3_str(&self) -> String {
        match self {
            Expression::Literal(lit) => lit.as_qasm3_str(),
            Expression::Identifier(id) => id.as_qasm3_str(),
            Expression::Array(array) => array.as_qasm3_str(),
            Expression::ArrayAccess(array_access) => array_access.as_qasm3_str(),
            Expression::Measurement(measurement) => measurement.as_qasm3_str(),
            Expression::BinOp(bin_op) => bin_op.as_qasm3_str(),
        }
    }
}

impl From<Literal> for Expression {
    fn from(literal: Literal) -> Self {
        Expression::Literal(literal)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Expression::Identifier(identifier)
    }
}

impl From<Array> for Expression {
    fn from(array: Array) -> Self {
        Expression::Array(array)
    }
}

impl From<ArrayAccess> for Expression {
    fn from(array_access: ArrayAccess) -> Self {
        Expression::ArrayAccess(array_access)
    }
}

impl From<Measurement> for Expression {
    fn from(measurement: Measurement) -> Self {
        Expression::Measurement(measurement)
    }
}

impl From<BinOp> for Expression {
    fn from(bin_op: BinOp) -> Self {
        Expression::BinOp(bin_op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_as_qasm3_str() {
        assert_eq!(Identifier::from("a").as_qasm3_str(), "a");
    }

    #[test]
    fn literal_as_qasm3_str() {
        assert_eq!(Literal::Bit(true).as_qasm3_str(), "1");
        assert_eq!(Literal::Bit(false).as_qasm3_str(), "0");
        assert_eq!(Literal::Bool(true).as_qasm3_str(), "true");
        assert_eq!(Literal::Bool(false).as_qasm3_str(), "false");
        assert_eq!(Literal::Int(1).as_qasm3_str(), "1");
        assert_eq!(Literal::Uint(1).as_qasm3_str(), "1");
        assert_eq!(Literal::Float(1.0).as_qasm3_str(), "1");
        assert_eq!(Literal::Pi.as_qasm3_str(), "pi");
        assert_eq!(Literal::Complex((1.0, 1.0)).as_qasm3_str(), "1 + 1im");
        assert_eq!(Literal::Complex((1.0, -1.0)).as_qasm3_str(), "1 - 1im");
    }

    #[test]
    fn array_as_qasm3_str() {
        assert_eq!(Array::new().as_qasm3_str(), "{}");
        assert_eq!(Array::from([Literal::Uint(1).into()]).as_qasm3_str(), "{1}");
        assert_eq!(
            Array::from([Literal::Uint(2).into(), Literal::Uint(3).into()]).as_qasm3_str(),
            "{2, 3}"
        );
    }

    #[test]
    fn array_access_as_qasm3_str() {
        assert_eq!(
            ArrayAccess::new::<Expression>("a".to_string(), vec![1]).as_qasm3_str(),
            "a[1]"
        );
        assert_eq!(
            ArrayAccess::new::<Expression>("a".to_string(), vec![1, 2]).as_qasm3_str(),
            "a[1, 2]"
        );
    }

    #[test]
    fn measurement_as_qasm3_str() {
        assert_eq!(
            Measurement::new::<Measurement>("a".to_string().into()).as_qasm3_str(),
            "measure a"
        );
    }

    #[test]
    fn bin_op_as_qasm3_str() {
        assert_eq!(
            PlusOp::new::<BinOp>("a".to_string().into(), "b".to_string().into()).as_qasm3_str(),
            "a + b"
        );
        assert_eq!(
            MinusOp::new::<BinOp>("a".to_string().into(), "b".to_string().into()).as_qasm3_str(),
            "a - b"
        );
        assert_eq!(
            TimesOp::new::<BinOp>("a".to_string().into(), "b".to_string().into()).as_qasm3_str(),
            "a * b"
        );
        assert_eq!(
            DivOp::new::<BinOp>("a".to_string().into(), "b".to_string().into()).as_qasm3_str(),
            "a / b"
        );
    }
}
