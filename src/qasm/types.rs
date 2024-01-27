use crate::qasm::AsQasmStr;

use std::rc::Rc;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Array {
    primitive: Primitive,
    dimensions: Vec<usize>,
}

impl Array {
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

trait TypeTrait: AsQasmStr {}

impl TypeTrait for Primitive {}
impl TypeTrait for Array {}

#[derive(Clone)]
pub struct Type {
    type_: Rc<dyn TypeTrait>,
}

impl AsQasmStr for Type {
    fn as_qasm_str(&self) -> String {
        self.type_.as_qasm_str()
    }
}

impl<T: TypeTrait + 'static> From<T> for Type {
    fn from(t: T) -> Type {
        Type { type_: Rc::new(t) }
    }
}
