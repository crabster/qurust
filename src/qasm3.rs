//! QASM3 meta program representation and parsing.
//!
//! This module contains structures and enums for representing QASM3 programs as well as a parser.
//! The structures and enums are designed to be convertible to QASM3 string.
//!
//! # Example
//!
//! ```rust
//! use qurust::qasm3::ir;
//! use qurust::qasm3::ir::AsQasmStr;
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
//!
//! fn random_bit_program() -> String {
//!     ir::Program::new(
//!         Some(ir::Version::new("3.0".to_string())),
//!         vec![
//!             ir::Gate::newt(
//!                 ir::Identifier::new("h".to_string()),
//!                 vec![],
//!                 vec![ir::Identifier::new("q".to_string())],
//!                 ir::Scope::newt(vec![
//!                     ir::GateCall::newt(
//!                         vec![],
//!                         ir::Identifier::new("U".to_string()),
//!                         vec![
//!                             ir::BinaryOperation::newt(
//!                                 ir::BinaryOperator::Div,
//!                                 ir::Identifier::newt("pi".to_string()),
//!                                 ir::Literal::DecimalInteger(2).into(),
//!                             ),
//!                             ir::Literal::DecimalInteger(0).into(),
//!                             ir::Identifier::newt("pi".to_string()),
//!                         ],
//!                         None,
//!                         vec![ir::Identifier::newt("q".to_string())],
//!                     ),
//!                     ir::GateCall::newt(
//!                         vec![],
//!                         ir::Identifier::new("gphase".to_string()),
//!                         vec![ir::BinaryOperation::newt(
//!                             ir::BinaryOperator::Div,
//!                             ir::Identifier::newt("pi".to_string()),
//!                             ir::Literal::DecimalInteger(-4).into(),
//!                         )],
//!                         None,
//!                         vec![],
//!                     ),
//!                 ]),
//!             ),
//!             ir::QuantumDeclaration::newt(
//!                 ir::Qubit::newt(None),
//!                 ir::Identifier::new("q".to_string()),
//!             ),
//!             ir::GateCall::newt(
//!                 vec![],
//!                 ir::Identifier::new("h".to_string()),
//!                 vec![],
//!                 None,
//!                 vec![ir::Identifier::newt("q".to_string())],
//!             ),
//!             ir::ClassicalDeclaration::newt(
//!                 ir::Scalar::Bit(None).into(),
//!                 ir::Identifier::newt("c".to_string()),
//!                 Some(ir::Measure::newt(ir::Identifier::newt("q".to_string()))),
//!             ),
//!         ],
//!     ).as_qasm_str()
//! }
//! ```

pub mod ir;
pub mod parser;
