//! Structures and enums for QASM3 types representation.
//!
//! All types used in QASM3 programs are defined by the `Type` enum.

use crate::qasm3::ir::expressions::Expression;
use crate::qasm3::ir::AsQasmStr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Scalar {
    Bit(Option<Expression>),
    Int(Option<Expression>),
    UInt(Option<Expression>),
    Float(Option<Expression>),
    Angle(Option<Expression>),
    Bool,
    Duration,
    Stretch,
    Complex(Option<Box<Scalar>>),
}

impl AsQasmStr for Scalar {
    fn as_qasm_str(&self) -> String {
        let format_expr = |e: &Option<Expression>| match e {
            Some(expr) => format!("[{}]", expr.as_qasm_str()),
            None => "".to_string(),
        };

        match self {
            Scalar::Bit(expr) => format!("bit{}", format_expr(expr)),
            Scalar::Int(expr) => format!("int{}", format_expr(expr)),
            Scalar::UInt(expr) => format!("uint{}", format_expr(expr)),
            Scalar::Float(expr) => format!("float{}", format_expr(expr)),
            Scalar::Angle(expr) => format!("angle{}", format_expr(expr)),
            Scalar::Bool => "bool".to_string(),
            Scalar::Duration => "duration".to_string(),
            Scalar::Stretch => "stretch".to_string(),
            Scalar::Complex(scalar) => {
                if let Some(scalar) = scalar {
                    format!("complex[{}]", scalar.as_qasm_str())
                } else {
                    "complex".to_string()
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Qubit {
    expr: Option<Expression>,
}

impl Qubit {
    pub fn new(expr: Option<Expression>) -> Self {
        Qubit { expr }.into()
    }

    pub fn newt<T: From<Qubit>>(expr: Option<Expression>) -> T {
        Qubit::new(expr).into()
    }
}

impl AsQasmStr for Qubit {
    fn as_qasm_str(&self) -> String {
        match &self.expr {
            Some(expr) => format!("qubit[{}]", expr.as_qasm_str()),
            None => "qubit".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Reference {
    ReadOnly,
    Mutable,
}

impl AsQasmStr for Reference {
    fn as_qasm_str(&self) -> String {
        match self {
            Reference::ReadOnly => "readonly".to_string(),
            Reference::Mutable => "mutable".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Array {
    ref_type: Option<Reference>,
    scalar_type: Box<Scalar>,
    exprs: Vec<Expression>,
    dim: Option<Expression>,
}

impl Array {
    pub fn new(scalar_type: Scalar, exprs: Vec<Expression>) -> Self {
        Array {
            ref_type: None,
            scalar_type: Box::new(scalar_type),
            exprs,
            dim: None,
        }
    }

    pub fn newt<T: From<Array>>(scalar_type: Scalar, exprs: Vec<Expression>) -> T {
        Array::new(scalar_type, exprs).into()
    }

    pub fn with_reference(
        ref_type: Reference,
        scalar_type: Scalar,
        exprs: Vec<Expression>,
        dim: Option<Expression>,
    ) -> Self {
        Array {
            ref_type: Some(ref_type),
            scalar_type: Box::new(scalar_type),
            exprs,
            dim,
        }
    }
}

impl AsQasmStr for Array {
    fn as_qasm_str(&self) -> String {
        match &self.ref_type {
            Some(ref_type) => match &self.dim {
                Some(dim) => {
                    format!(
                        "{} array[{}, #dim = {}]",
                        ref_type.as_qasm_str(),
                        self.scalar_type.as_qasm_str(),
                        dim.as_qasm_str()
                    )
                }
                None => {
                    format!(
                        "{} array[{}, {}]",
                        ref_type.as_qasm_str(),
                        self.scalar_type.as_qasm_str(),
                        self.exprs
                            .iter()
                            .map(|e| e.as_qasm_str())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            },
            None => {
                let exprs_str = self
                    .exprs
                    .iter()
                    .map(|e| e.as_qasm_str())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("array[{}, {}]", self.scalar_type.as_qasm_str(), exprs_str)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Register {
    C,
    Q,
}

impl AsQasmStr for Register {
    fn as_qasm_str(&self) -> String {
        match self {
            Register::C => "creg".to_string(),
            Register::Q => "qreg".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Scalar(Box<Scalar>),
    Qubit(Box<Qubit>),
    Array(Box<Array>),
    Register(Register),
}

impl AsQasmStr for Type {
    fn as_qasm_str(&self) -> String {
        match self {
            Type::Scalar(scalar) => scalar.as_qasm_str(),
            Type::Qubit(qubit) => qubit.as_qasm_str(),
            Type::Array(array) => array.as_qasm_str(),
            Type::Register(reg) => reg.as_qasm_str(),
        }
    }
}

impl From<Scalar> for Type {
    fn from(scalar: Scalar) -> Self {
        Type::Scalar(Box::new(scalar))
    }
}

impl From<Qubit> for Type {
    fn from(qubit: Qubit) -> Self {
        Type::Qubit(Box::new(qubit))
    }
}

impl From<Array> for Type {
    fn from(array: Array) -> Self {
        Type::Array(Box::new(array))
    }
}

impl From<Register> for Type {
    fn from(reg: Register) -> Self {
        Type::Register(reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::qasm3::ir::expressions::Literal;

    #[test]
    fn scalar_as_qasm3_str() {
        assert_eq!(Scalar::Bit(None).as_qasm_str(), "bit");
        assert_eq!(
            Scalar::Bit(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "bit[1]"
        );
        assert_eq!(Scalar::Int(None).as_qasm_str(), "int");
        assert_eq!(
            Scalar::Int(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "int[1]"
        );
        assert_eq!(Scalar::UInt(None).as_qasm_str(), "uint");
        assert_eq!(
            Scalar::UInt(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "uint[1]"
        );
        assert_eq!(Scalar::Float(None).as_qasm_str(), "float");
        assert_eq!(
            Scalar::Float(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "float[1]"
        );
        assert_eq!(Scalar::Angle(None).as_qasm_str(), "angle");
        assert_eq!(
            Scalar::Angle(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "angle[1]"
        );
        assert_eq!(Scalar::Bool.as_qasm_str(), "bool");
        assert_eq!(Scalar::Duration.as_qasm_str(), "duration");
        assert_eq!(Scalar::Stretch.as_qasm_str(), "stretch");
        assert_eq!(Scalar::Complex(None).as_qasm_str(), "complex");
        assert_eq!(
            Scalar::Complex(Some(Box::new(Scalar::Bit(None)))).as_qasm_str(),
            "complex[bit]"
        );
    }

    #[test]
    fn qubit_as_qasm3_str() {
        assert_eq!(Qubit::new(None).as_qasm_str(), "qubit");
        assert_eq!(
            Qubit::new(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "qubit[1]"
        );
    }

    #[test]
    fn reference_as_qasm3_str() {
        assert_eq!(Reference::ReadOnly.as_qasm_str(), "readonly");
        assert_eq!(Reference::Mutable.as_qasm_str(), "mutable");
    }

    #[test]
    fn array_as_qasm3_str() {
        assert_eq!(
            Array::new(Scalar::Bit(None), vec![Literal::DecimalInteger(1).into()]).as_qasm_str(),
            "array[bit, 1]"
        );
        assert_eq!(
            Array::new(
                Scalar::Bit(None),
                vec![
                    Literal::DecimalInteger(1).into(),
                    Literal::DecimalInteger(2).into()
                ]
            )
            .as_qasm_str(),
            "array[bit, 1, 2]"
        );
        assert_eq!(
            Array::with_reference(
                Reference::ReadOnly,
                Scalar::Bit(None),
                vec![Literal::DecimalInteger(1).into()],
                None
            )
            .as_qasm_str(),
            "readonly array[bit, 1]"
        );
        assert_eq!(
            Array::with_reference(
                Reference::ReadOnly,
                Scalar::Bit(None),
                vec![Literal::DecimalInteger(1).into()],
                Some(Literal::DecimalInteger(2).into())
            )
            .as_qasm_str(),
            "readonly array[bit, #dim = 2]"
        );
    }

    #[test]
    fn register_as_qasm3_str() {
        assert_eq!(Register::C.as_qasm_str(), "creg");
        assert_eq!(Register::Q.as_qasm_str(), "qreg");
    }

    #[test]
    fn type_as_qasm3_str() {
        assert_eq!(
            Type::Scalar(Box::new(Scalar::Bit(None))).as_qasm_str(),
            "bit"
        );
        assert_eq!(
            Type::Qubit(Box::new(Qubit::new(None))).as_qasm_str(),
            "qubit"
        );
        assert_eq!(
            Type::Array(
                Box::new(Array::new(
                    Scalar::Bit(None),
                    vec![Literal::DecimalInteger(1).into()]
                ))
                .into()
            )
            .as_qasm_str(),
            "array[bit, 1]"
        );
        assert_eq!(Type::Register(Register::C).as_qasm_str(), "creg");
    }
}
