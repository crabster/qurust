use crate::qasm::expressions::Expression;
use crate::qasm::AsQasmStr;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct U3Gate {
    theta: Expression,
    phi: Expression,
    lambda: Expression,
    qubit: Expression,
}

impl U3Gate {
    #[allow(clippy::new_ret_no_self)]
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

#[derive(Clone, Debug)]
pub struct GPGate {
    delta: Expression,
}

impl GPGate {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<GPGate>>(delta: Expression) -> T {
        Self { delta }.into()
    }
}

impl AsQasmStr for GPGate {
    fn as_qasm_str(&self) -> String {
        format!("gphase({})", self.delta.as_qasm_str())
    }
}

#[derive(Clone, Debug)]
pub struct CustomGate {
    name: String,
    params: Vec<Expression>,
    args: Vec<Expression>,
}

impl CustomGate {
    #[allow(clippy::new_ret_no_self)]
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
            .join(" ");

        if params.is_empty() && args.is_empty() {
            self.name.clone()
        } else if params.is_empty() {
            format!("{} {}", self.name, args)
        } else if args.is_empty() {
            format!("{}({})", self.name, params)
        } else {
            format!("{}({}) {}", self.name, params, args)
        }
    }
}

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u3_gate_as_qasm_str() {
        let gate = U3Gate::new::<Gate>(
            "t".to_string().into(),
            "p".to_string().into(),
            "l".to_string().into(),
            "q".to_string().into(),
        );
        assert_eq!(gate.as_qasm_str(), "U(t, p, l) q".to_string());
    }

    #[test]
    fn gp_gate_as_qasm_str() {
        let gate = GPGate::new::<Gate>("d".to_string().into());
        assert_eq!(gate.as_qasm_str(), "gphase(d)".to_string());
    }

    #[test]
    fn custom_gate_as_qasm_str() {
        assert_eq!(
            CustomGate::new::<Gate>("name".to_string(), vec![], vec![],).as_qasm_str(),
            "name".to_string()
        );
        assert_eq!(
            CustomGate::new::<Gate>(
                "name".to_string(),
                vec!["l1".to_string().into(), "l2".to_string().into()],
                vec![],
            )
            .as_qasm_str(),
            "name(l1, l2)".to_string()
        );
        assert_eq!(
            CustomGate::new::<Gate>(
                "name".to_string(),
                vec![],
                vec!["q1".to_string().into(), "q2".to_string().into()],
            )
            .as_qasm_str(),
            "name q1 q2".to_string()
        );
        assert_eq!(
            CustomGate::new::<Gate>(
                "name".to_string(),
                vec!["l1".to_string().into(), "l2".to_string().into()],
                vec!["q1".to_string().into(), "q2".to_string().into()],
            )
            .as_qasm_str(),
            "name(l1, l2) q1 q2".to_string()
        );
    }
}
