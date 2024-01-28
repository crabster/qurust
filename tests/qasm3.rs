use qurust::qasm3::{expressions::*, types::*, *};

#[test]
fn random_number_circuit() {
    let expected = r#"
OPENQASM 3.0;

gate h q {
    U(pi / 2, 0, pi) q;
    gphase(pi / -4);
}

qubit q;
h q;
bit c = measure q;
    "#;

    let program = program::Program::new(vec![
        blocks::GateDeclaration::new(
            gates::CustomGate::new("h".to_string(), vec![], vec!["q".to_string().into()]),
            vec![
                statements::GateApplication::new(gates::U3Gate::new(
                    DivOp::new(Literal::Pi.into(), Literal::Uint(2).into()),
                    Literal::Uint(0).into(),
                    Literal::Pi.into(),
                    "q".to_string().into(),
                )),
                statements::GateApplication::new(gates::GPGate::new(DivOp::new(
                    Literal::Pi.into(),
                    Literal::Int(-4).into(),
                ))),
            ],
        ),
        statements::EmptyLine::new(),
        statements::VariableDeclaration::new(Primitive::Qubit.into(), "q".to_string().into(), None),
        statements::GateApplication::new(gates::CustomGate::new(
            "h".to_string(),
            vec![],
            vec!["q".to_string().into()],
        )),
        statements::VariableDeclaration::new(
            Primitive::Bit.into(),
            "c".to_string(),
            Some(Measurement::new(Identifier::from("q".to_string()).into())),
        ),
    ]);

    assert_eq!(program.as_qasm3_str(), expected.trim());
}
