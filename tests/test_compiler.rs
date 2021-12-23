use kaon_lang::analysis::SemanticAnalyzer;
use kaon_lang::compiler::Compiler;
use kaon_lang::parser::{Parser, ParserRes};
use kaon_lang::data::Data;

fn new_parser(src: String) -> ParserRes {
    let mut analyzer = SemanticAnalyzer::new();
    let ast = Parser::new(src).parse(&mut analyzer);
    return ast;
}

fn new_compiler() -> Compiler {
    Compiler::build()
}

#[test]
fn test_compiler() {
    let src = r#"
        1 + 2
        5 * (10 - 3)
        -4 - -8
    "#
    .to_string();
    let ast = new_parser(src).unwrap();
    let mut compiler = new_compiler();
    assert_eq!(compiler.run(&ast).is_ok(), true);
}

#[test]
fn compile_number() {
    let ast = new_parser("123".to_string()).unwrap();
    let mut compiler = new_compiler();
    let chunk = compiler.run(&ast).unwrap();
    assert_eq!(chunk.opcodes, vec![0, 0, 19]);
    assert_eq!(chunk.constants, vec![Data::Number(123.0)]);
}

#[test]
fn compile_binary() {
    let ast = new_parser("1 + 2".to_string()).unwrap();
    let mut compiler = new_compiler();
    let chunk = compiler.run(&ast).unwrap();
    assert_eq!(chunk.opcodes, vec![0, 0, 0, 1, 1, 19]);
    assert_eq!(chunk.constants, vec![Data::Number(2.0), Data::Number(1.0)]);
}

#[test]
fn compile_unary() {
    let ast = new_parser("-8".to_string()).unwrap();
    let mut compiler = new_compiler();
    let chunk = compiler.run(&ast).unwrap();
    assert_eq!(chunk.opcodes, vec![0, 0, 6, 19]);
    assert_eq!(chunk.constants, vec![Data::Number(8.0)]);
}
