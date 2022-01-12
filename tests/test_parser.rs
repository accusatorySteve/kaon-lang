use kaon_lang::analysis::SemanticAnalyzer;
use kaon_lang::ast::{ASTNode, BinExpr, Expr, Stmt, Op, AST};
use kaon_lang::error::SyntaxError;
use kaon_lang::lexer::Lexer;
use kaon_lang::parser::Parser;
use kaon_lang::source::Source;
use kaon_lang::span::Span;

use std::path::PathBuf;
use std::rc::Rc;

fn new_parser(input: &str) -> Result<(AST, Rc<Source>), SyntaxError> {
    let source = Source::new(input, &PathBuf::from("./main"));
    let tokens = Lexer::new(source.clone()).tokenize().unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let mut parser = Parser::new(tokens);
    Ok((parser.parse(&mut analyzer).unwrap(), source.clone()))
}

#[test]
fn parse_literal() {
    let (ast, source) = new_parser("7").unwrap();
    assert_eq!(
        ast.nodes[0],
        ASTNode::from(Stmt::Expr(Expr::Number(7.0, Span::new(0, 1, &source))))
    );
}

#[test]
fn parse_bin_expr() {
    let (ast, source) = new_parser("1 + 2").unwrap();
    assert_eq!(
        ast.nodes[0],
        ASTNode::from(Stmt::Expr(Expr::BinExpr(
            Box::new(BinExpr::new(
                Op::Add,
                Expr::Number(1.0, Span::new(0, 1, &source)),
                Expr::Number(2.0, Span::new(4, 1, &source))
            )),
            Span::new(5, 1, &source)
        )))
    )
}

#[test]
fn test_parser() {
    let input = r#"
        1 + 2
        10 - 9
        2 * 3
        8 / 4
    "#;
    let res = new_parser(input);
    assert_eq!(res.is_ok(), true);
}
