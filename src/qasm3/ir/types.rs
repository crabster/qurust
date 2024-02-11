//! Structures and enums for QASM3 types representation.
//!
//! All types used in QASM3 programs are defined by the `Type` enum.

use crate::qasm3::ir::expressions::Expression;
use crate::qasm3::ir::AsQasmStr;

use std::fmt::Debug;

#[derive(Debug)]
pub enum Scalar {
    Bit(Option<Expression>),
    Int(Option<Expression>),
    Uint(Option<Expression>),
    Float(Option<Expression>),
    Angle(Option<Expression>),
    Bool,
    Duration,
    Stretch,
    Complex(Option<Expression>),
}

impl AsQasmStr for Scalar {
    fn as_qasm_str(&self) -> String {
        let format_expr = |e: &Option<Expression>| {
            match e {
                Some(expr) => expr.as_qasm_str(),
                None => "".to_string(),
            }
        };

        match self {
            Scalar::Bit(expr) => format_expr(expr),
            Scalar::Int(expr) => format_expr(expr),
            Scalar::Uint(expr) => format_expr(expr),
            Scalar::Float(expr) => format_expr(expr),
            Scalar::Angle(expr) => format_expr(expr),
            Scalar::Bool => "bool".to_string(),
            Scalar::Duration => "duration".to_string(),
            Scalar::Stretch => "stretch".to_string(),
            Scalar::Complex(expr) => format_expr(expr),
        }
    }
}

#[derive(Debug)]
pub struct Qubit {
    expr: Option<Expression>,
}

impl Qubit {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Qubit>>(expr: Option<Expression>) -> T {
        Qubit { expr }.into()
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Array {
    ref_type: Option<Reference>,
    scalar_type: Box<Scalar>,
    dimensions: Option<Expression>,
    exprs: Vec<Expression>,
}

impl Array {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Array>>(scalar_type: Scalar, exprs: Vec<Expression>) -> T {
        Array {
            ref_type: None,
            scalar_type: Box::new(scalar_type),
            dimensions: None,
            exprs,
        }
        .into()
    }

    pub fn with_reference<T: From<Array>>(
        ref_type: Reference,
        scalar_type: Scalar,
        dimensions: Expression,
    ) -> T {
        Array {
            ref_type: Some(ref_type),
            scalar_type: Box::new(scalar_type),
            dimensions: Some(dimensions),
            exprs: vec![],
        }
        .into()
    }
}

impl AsQasmStr for Array {
    fn as_qasm_str(&self) -> String {
        match &self.ref_type {
            Some(ref_type) => match &self.dimensions {
                Some(dimensions) => {
                    format!(
                        "{} array[{}, #dim = {}]",
                        ref_type.as_qasm_str(),
                        self.scalar_type.as_qasm_str(),
                        dimensions.as_qasm_str()
                    )
                }
                None => {
                    format!(
                        "array[{}, {}]",
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

                format!("array{}[{}]", self.scalar_type.as_qasm_str(), exprs_str)
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
