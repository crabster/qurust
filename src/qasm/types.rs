use crate::qasm::AsQasmStr;

use std::fmt::Debug;

/// QASM3 primitive type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Primitive {
    Qubit,
    Bit,
    Bool,
    Int(usize),
    Uint(usize),
    Float(usize),
    Angle(usize),
    Complex(Option<usize>),
}

impl AsQasmStr for Primitive {
    fn as_qasm_str(&self) -> String {
        match self {
            Primitive::Qubit => "qubit".to_string(),
            Primitive::Bit => "bit".to_string(),
            Primitive::Bool => "bool".to_string(),
            Primitive::Int(size) => format!("int[{}]", size),
            Primitive::Uint(size) => format!("uint[{}]", size),
            Primitive::Float(size) => format!("float[{}]", size),
            Primitive::Angle(size) => format!("angle[{}]", size),
            Primitive::Complex(size_opt) => match size_opt {
                Some(size) => format!("complex[{}]", size),
                None => "complex".to_string(),
            },
        }
    }
}

/// QASM3 array type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Array {
    primitive: Primitive,
    dimensions: Vec<usize>,
}

impl Array {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Array>>(primitive: Primitive, dimensions: Vec<usize>) -> T {
        Self {
            primitive,
            dimensions,
        }
        .into()
    }
}

impl AsQasmStr for Array {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}[{}]",
            self.primitive.as_qasm_str(),
            self.dimensions
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// QASM3 type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Type {
    Primitive(Primitive),
    Array(Array),
}

impl AsQasmStr for Type {
    fn as_qasm_str(&self) -> String {
        match self {
            Type::Primitive(p) => p.as_qasm_str(),
            Type::Array(a) => a.as_qasm_str(),
        }
    }
}

impl From<Primitive> for Type {
    fn from(t: Primitive) -> Type {
        Type::Primitive(t)
    }
}

impl From<Array> for Type {
    fn from(t: Array) -> Type {
        Type::Array(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_as_qasm_str() {
        assert_eq!(Primitive::Qubit.as_qasm_str(), "qubit");
        assert_eq!(Primitive::Bit.as_qasm_str(), "bit");
        assert_eq!(Primitive::Bool.as_qasm_str(), "bool");
        assert_eq!(Primitive::Int(1).as_qasm_str(), "int[1]");
        assert_eq!(Primitive::Uint(1).as_qasm_str(), "uint[1]");
        assert_eq!(Primitive::Float(1).as_qasm_str(), "float[1]");
        assert_eq!(Primitive::Angle(1).as_qasm_str(), "angle[1]");
        assert_eq!(Primitive::Complex(Some(1)).as_qasm_str(), "complex[1]");
        assert_eq!(Primitive::Complex(None).as_qasm_str(), "complex");
    }

    #[test]
    fn array_as_qasm_str() {
        assert_eq!(
            Array::new::<Type>(Primitive::Qubit.into(), vec![1]).as_qasm_str(),
            "qubit[1]"
        );
        assert_eq!(
            Array::new::<Type>(Primitive::Bit.into(), vec![2, 3]).as_qasm_str(),
            "bit[2, 3]"
        );
    }
}
