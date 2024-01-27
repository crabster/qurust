use crate::qasm::expressions::Expression;
use crate::qasm::AsQasmStr;

use std::rc::Rc;

#[derive(Clone)]
pub struct U3Gate {
    theta: Expression,
    phi: Expression,
    lambda: Expression,
    qubit: Expression,
}

impl U3Gate {
    pub fn new<T: From<U3Gate>>(
        theta: Expression,
        phi: Expression,
        lambda: Expression,
        qubit: Expression,
    ) -> T {
        Self {
            theta,
            phi,
            lambda,
            qubit,
        }
        .into()
    }
}

impl AsQasmStr for U3Gate {
    fn as_qasm_str(&self) -> String {
        format!(
            "U({}, {}, {}) {}",
            self.theta.as_qasm_str(),
            self.phi.as_qasm_str(),
            self.lambda.as_qasm_str(),
            self.qubit.as_qasm_str()
        )
    }
}

#[derive(Clone)]
pub struct GPGate {
    delta: Expression,
}

impl GPGate {
    pub fn new<T: From<GPGate>>(delta: Expression) -> T {
        Self { delta }.into()
    }
}

impl AsQasmStr for GPGate {
    fn as_qasm_str(&self) -> String {
        format!("gphase({})", self.delta.as_qasm_str())
    }
}

#[derive(Clone)]
pub struct CustomGate {
    name: String,
    params: Vec<Expression>,
    args: Vec<Expression>,
}

impl CustomGate {
    pub fn new<T: From<CustomGate>>(
        name: String,
        params: Vec<Expression>,
        args: Vec<Expression>,
    ) -> T {
        Self { name, params, args }.into()
    }
}

impl AsQasmStr for CustomGate {
    fn as_qasm_str(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|param| param.as_qasm_str())
            .collect::<Vec<String>>()
            .join(", ");
        let args = self
            .args
            .iter()
            .map(|arg| arg.as_qasm_str())
            .collect::<Vec<String>>()
            .join(", ");
        if params.is_empty() {
            format!("{} {}", self.name, args)
        } else {
            format!("{}({}) {}", self.name, params, args)
        }
    }
}

#[derive(Clone)]
pub struct Gate {
    gate: Rc<dyn GateTrait>,
}

trait GateTrait: AsQasmStr {}

impl GateTrait for U3Gate {}
impl GateTrait for GPGate {}
impl GateTrait for CustomGate {}

impl AsQasmStr for Gate {
    fn as_qasm_str(&self) -> String {
        self.gate.as_qasm_str()
    }
}

impl<T: GateTrait + 'static> From<T> for Gate {
    fn from(gate: T) -> Self {
        Self {
            gate: Rc::new(gate),
        }
    }
}
