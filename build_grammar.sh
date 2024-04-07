#!/bin/bash

# This script is used to generate the lexer and parser for the QASM3 language and fix bugs in the generated files.

# Download the antlr4 jar file
antlr_path="antlr_runtime/antlr4-4.8-2-SNAPSHOT-complete.jar"
curl -L \
    "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar" \
    -o $antlr_path --create-dirs

# Generate the lexer and parser
java -jar $antlr_path -Dlanguage=Rust -visitor src/qasm3/parser/antlr/qasm3Lexer.g4 src/qasm3/parser/antlr/qasm3.g4

# Fix the generated files
sed 's/qasm3ParserContext/qasm3Context/g' -i src/qasm3/parser/antlr/qasm3parser.rs
sed 's/use super::qasm3/use super::qasm3parser/g' -i src/qasm3/parser/antlr/qasm3visitor.rs
sed 's/use super::qasm3/use super::qasm3parser/g' -i src/qasm3/parser/antlr/qasm3listener.rs
sed '2i\#![allow(unused_parens)]' -i src/qasm3/parser/antlr/qasm3parser.rs

# Remove the generated files we don't need
rm src/qasm3/parser/antlr/qasm3Lexer.interp \
    src/qasm3/parser/antlr/qasm3Lexer.tokens \
    src/qasm3/parser/antlr/qasm3.interp \
    src/qasm3/parser/antlr/qasm3.tokens

# Format the generated files
cargo fmt -- src/qasm3/parser/antlr/*.rs
