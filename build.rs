use std::path::Path;
use std::process::Command;

fn main() {
    let antlr_path = "antlr_runtime/antlr4-4.8-2-SNAPSHOT-complete.jar";
    if !Path::new(antlr_path).exists() {
        let _cmd = Command::new("curl")
            .arg("-L")
            .arg(
                "https://github.com/rrevenantt/antlr4rust/releases/download/\
                antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar",
            )
            .arg("-o")
            .arg(antlr_path)
            .arg("--create-dirs")
            .spawn()
            .expect("Couldn't download `antlr4-4.8-2-SNAPSHOT-complete.jar` runtime.")
            .wait_with_output();
    }

    println!("cargo:rerun-if-changed=src/qasm3/parser/antlr/qasm3Lexer.g4");
    println!("cargo:rerun-if-changed=src/qasm3/parser/antlr/qasm3.g4");

    let _cmd = Command::new("java")
        .arg("-jar")
        .arg(antlr_path)
        .arg("-Dlanguage=Rust")
        .arg("-visitor")
        .arg("src/qasm3/parser/antlr/qasm3Lexer.g4")
        .arg("src/qasm3/parser/antlr/qasm3.g4")
        .spawn()
        .expect("Antlr couldn't generate the parser and lexer files.")
        .wait_with_output();

    let _cmd = Command::new("sed")
        .arg("s/qasm3ParserContext/qasm3Context/g")
        .arg("-i")
        .arg("src/qasm3/parser/antlr/qasm3parser.rs")
        .spawn()
        .expect("Couldn't replace qasm3ParserContext with qasm3Context in qasm3parser.rs.")
        .wait_with_output();

    let _cmd = Command::new("sed")
        .arg("s/use super::qasm3/use super::qasm3parser/g")
        .arg("-i")
        .arg("src/qasm3/parser/antlr/qasm3visitor.rs")
        .spawn()
        .expect("Couldn't replace super::qasm3 with super::qasm3parser in qasm3visitor.rs.")
        .wait_with_output();

    let _cmd = Command::new("sed")
        .arg("s/use super::qasm3/use super::qasm3parser/g")
        .arg("-i")
        .arg("src/qasm3/parser/antlr/qasm3listener.rs")
        .spawn()
        .expect("Couldn't replace super::qasm3 with super::qasm3parser in qasm3listener.rs.")
        .wait_with_output();

    let _cmd = Command::new("sed")
        .arg("2i\\#![allow(unused_parens)]")
        .arg("-i")
        .arg("src/qasm3/parser/antlr/qasm3parser.rs")
        .spawn()
        .expect("Couldn't add #![allow(unused_parens)] to qasm3parser.rs.")
        .wait_with_output();

    let _cmd = Command::new("rm")
        .arg("src/qasm3/parser/antlr/qasm3Lexer.interp")
        .arg("src/qasm3/parser/antlr/qasm3Lexer.tokens")
        .arg("src/qasm3/parser/antlr/qasm3.interp")
        .arg("src/qasm3/parser/antlr/qasm3.tokens")
        .spawn()
        .expect("Couldn't remove the .interp and .tokens files.")
        .wait_with_output();

    let _cmd = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("src/qasm3/parser/antlr/*.rs")
        .spawn()
        .expect("Couldn't format the generated files.")
        .wait_with_output();
}
