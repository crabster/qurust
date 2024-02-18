mod antlr;
mod visitor;

use crate::qasm3::ir;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::tree::ParseTreeVisitorCompat;

pub fn parse(input: &str) -> Result<ir::Program, String> {
    let input = InputStream::new(input);
    let mut _lexer = antlr::qasm3lexer::qasm3Lexer::new(input);
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = antlr::qasm3parser::qasm3::new(token_source);

    let root = parser.program().unwrap();

    let mut visitor = visitor::Visitor::new();
    Ok(visitor.visit(&*root).result())
}
