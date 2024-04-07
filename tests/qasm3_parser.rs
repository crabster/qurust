use indoc::indoc;
use qurust::qasm3::ir::AsQasmStr;

#[test]
fn parse_random_number_circuit() {
    let expected = indoc! {r#"
        OPENQASM 3.0;
        gate h q {
            U(pi / 2, 0, pi) q;
            gphase(pi / -4);
        }
        qubit q;
        h q;
        bit c = measure q;
    "#};

    let program = match qurust::qasm3::parser::parse(expected) {
        Ok(program) => program,
        Err(e) => panic!("Error parsing QASM3: {}", e),
    };
    assert_eq!(program.as_qasm_str(), expected);
}
