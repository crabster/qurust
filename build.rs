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
    println!("cargo:rerun-if-changed=src/qasm3/parser/antlr/qasm3Parser.g4");
    let _cmd = Command::new("java")
        .arg("-jar")
        .arg(antlr_path)
        .arg("-Dlanguage=Rust")
        .arg("-visitor")
        .arg("src/qasm3/parser/antlr/qasm3Lexer.g4")
        .arg("src/qasm3/parser/antlr/qasm3Parser.g4")
        .spawn()
        .expect("Antlr couldn't generate the parser and lexer files.")
        .wait_with_output();

    let _cmd = Command::new("rm")
        .arg("src/qasm3/parser/antlr/qasm3Lexer.interp")
        .arg("src/qasm3/parser/antlr/qasm3Lexer.tokens")
        .arg("src/qasm3/parser/antlr/qasm3Parser.interp")
        .arg("src/qasm3/parser/antlr/qasm3Parser.tokens")
        .spawn()
        .expect("Couldn't remove the .interp and .tokens files.")
        .wait_with_output();
}
