//! QASM3 meta program representation.
//!
//! This module contains structures and enums for representing QASM3 programs.
//! The structures and enums are designed to be easily convertible to QASM3 string.
//!
//! # Example
//!
//! ```rust
//! use qurust::qasm3::{expressions::*, types::*, *};
//!
//! // Generate a random bit program:
//! // ```
//! // OPENQASM 3.0;
//! //
//! // gate h q {
//! //     U(pi / 2, 0, pi) q;
//! //     gphase(pi / -4);
//! // }
//! //
//! // qubit q;
//! // h q;
//! // bit c = measure q;
//! // ```
//! fn random_bit_program() -> String {
//!     program::Program::new(vec![
//!         blocks::GateDeclaration::new(
//!             gates::CustomGate::new("h".to_string(), vec![], vec!["q".to_string().into()]),
//!             vec![
//!                 statements::GateApplication::new(gates::U3Gate::new(
//!                     DivOp::new(Literal::Pi.into(), Literal::Uint(2).into()),
//!                     Literal::Uint(0).into(),
//!                     Literal::Pi.into(),
//!                     "q".to_string().into(),
//!                 )),
//!                 statements::GateApplication::new(gates::GPGate::new(DivOp::new(
//!                     Literal::Pi.into(),
//!                     Literal::Int(-4).into(),
//!                 ))),
//!             ],
//!         ),
//!         statements::EmptyLine::new(),
//!         statements::VariableDeclaration::new(Primitive::Qubit.into(), "q".to_string().into(), None),
//!         statements::GateApplication::new(gates::CustomGate::new(
//!             "h".to_string(),
//!             vec![],
//!             vec!["q".to_string().into()],
//!         )),
//!         statements::VariableDeclaration::new(
//!             Primitive::Bit.into(),
//!             "c".to_string(),
//!             Some(Measurement::new(Identifier::from("q".to_string()).into())),
//!         ),
//!     ]).as_qasm3_str()
//! }
//! ```

pub mod blocks;
pub mod expressions;
pub mod gates;
pub mod program;
pub mod statements;
pub mod types;

pub mod ir;
pub mod parser;

use std::fmt::Debug;

/// Trait for types convertible to QASM3 string.
pub trait AsQasmStr: Debug {
    fn as_qasm3_str(&self) -> String;
}
