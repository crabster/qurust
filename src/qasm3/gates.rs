//! Structures and enums for QASM3 gates representation.
//!
//! Gates are defined by the `Gate` enum. They are typically composed from expressions,
//! defined by the `expressions::Expression` enum. They define their declaration's header
//! and application's expression.

use crate::qasm3::expressions::Expression;
use crate::qasm3::AsQasmStr;

/// QASM3 U3 gate.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
    fn as_qasm3_str(&self) -> String {
        format!(
            "U({}, {}, {}) {}",
            self.theta.as_qasm3_str(),
            self.phi.as_qasm3_str(),
            self.lambda.as_qasm3_str(),
            self.qubit.as_qasm3_str()
        )
    }
}

/// QASM3 gphase gate.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
    fn as_qasm3_str(&self) -> String {
        format!("gphase({})", self.delta.as_qasm3_str())
    }
}

/// QASM3 custom gate.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
    fn as_qasm3_str(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|param| param.as_qasm3_str())
            .collect::<Vec<String>>()
            .join(", ");
        let args = self
            .args
            .iter()
            .map(|arg| arg.as_qasm3_str())
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

/// QASM3 gate.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Gate {
    U3(U3Gate),
    GP(GPGate),
    Custom(CustomGate),
}

impl AsQasmStr for Gate {
    fn as_qasm3_str(&self) -> String {
        match self {
            Gate::U3(gate) => gate.as_qasm3_str(),
            Gate::GP(gate) => gate.as_qasm3_str(),
            Gate::Custom(gate) => gate.as_qasm3_str(),
        }
    }
}

impl From<U3Gate> for Gate {
    fn from(gate: U3Gate) -> Self {
        Gate::U3(gate)
    }
}

impl From<GPGate> for Gate {
    fn from(gate: GPGate) -> Self {
        Gate::GP(gate)
    }
}

impl From<CustomGate> for Gate {
    fn from(gate: CustomGate) -> Self {
        Gate::Custom(gate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u3_gate_as_qasm3_str() {
        let gate = U3Gate::new::<Gate>(
            "t".to_string().into(),
            "p".to_string().into(),
            "l".to_string().into(),
            "q".to_string().into(),
        );
        assert_eq!(gate.as_qasm3_str(), "U(t, p, l) q".to_string());
    }

    #[test]
    fn gp_gate_as_qasm3_str() {
        let gate = GPGate::new::<Gate>("d".to_string().into());
        assert_eq!(gate.as_qasm3_str(), "gphase(d)".to_string());
    }

    #[test]
    fn custom_gate_as_qasm3_str() {
        assert_eq!(
            CustomGate::new::<Gate>("name".to_string(), vec![], vec![],).as_qasm3_str(),
            "name".to_string()
        );
        assert_eq!(
            CustomGate::new::<Gate>(
                "name".to_string(),
                vec!["l1".to_string().into(), "l2".to_string().into()],
                vec![],
            )
            .as_qasm3_str(),
            "name(l1, l2)".to_string()
        );
        assert_eq!(
            CustomGate::new::<Gate>(
                "name".to_string(),
                vec![],
                vec!["q1".to_string().into(), "q2".to_string().into()],
            )
            .as_qasm3_str(),
            "name q1 q2".to_string()
        );
        assert_eq!(
            CustomGate::new::<Gate>(
                "name".to_string(),
                vec!["l1".to_string().into(), "l2".to_string().into()],
                vec!["q1".to_string().into(), "q2".to_string().into()],
            )
            .as_qasm3_str(),
            "name(l1, l2) q1 q2".to_string()
        );
    }
}
