//! AST (abstract-syntax tree). 

use crate::common::Span;
use smallvec::SmallVec;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct AST {
    pub nodes: Vec<ASTNode>,
    pub span: Span,
}

impl AST {
    pub fn new(nodes: Vec<ASTNode>, span: Span) -> Self {
        AST { nodes, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Stmt(Stmt),
    Expr(Expr),
}

impl From<Stmt> for ASTNode {
    fn from(stmt: Stmt) -> ASTNode {
        ASTNode::Stmt(stmt)
    }
}

impl From<&Stmt> for ASTNode {
    fn from(stmt: &Stmt) -> ASTNode {
        ASTNode::Stmt(stmt.clone())
    }
}

impl From<Expr> for ASTNode {
    fn from(expr: Expr) -> ASTNode {
        ASTNode::Expr(expr)
    }
}

impl From<&Expr> for ASTNode {
    fn from(expr: &Expr) -> ASTNode {
        ASTNode::Expr(expr.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub name: Ident,
    pub params: (Vec<Ident>, Vec<Option<Expr>>),
    pub return_typ: Option<Expr>,
    pub span: Span,
}

pub struct StmtBlock(SmallVec<[Stmt; 4]>, Span);

#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub methods: Vec<TraitMethod>,
    pub span: Span,
}

/// A `trait` method, including the function's signature and an
/// optional default block. 
#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    pub sig: Signature,
    pub default: Option<Stmt>,
    pub span: Span,
}

/// A statment
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    /// `if` expr `{` body `}` `else` `{`body `}`
    IfStatement(Expr, Box<(Stmt, Option<Stmt>)>, Span),
    /// `while` expr `{` body `}`
    WhileStatement(Expr, Box<Stmt>, Span),
    /// `loop` `{` body `}`
    LoopStatement(Box<Stmt>, Span),
    /// import statement
    ImportStatement(Expr, Span),
    /// `{` body `}`
    Block(Box<Vec<Stmt>>, Span),
    /// `var` id [`:` type] `=` expr
    VarDeclaration(Ident, Option<Expr>, Option<Expr>, Span),
    /// `con` id [`:` type] `=` expr
    ConDeclaration(Ident, Expr, Option<Expr>, Span),
    /// expr `=` expr
    AssignStatement(Expr, Expr, Span),
    /// `fun` id `(` ...args `)` `{` body `}`
    Function(Box<ScriptFun>, Span),
    /// `class` id `{` method | field `}`
    Class(Class, Span),
    Trait(Trait),
    /// `const` name `(` ...args `)` `{` body `}`
    Constructor(Box<Constructor>, Span),
    /// `return` expr
    Return(Option<Expr>, Span),
    /// `break`
    Break(Span),
    /// `continue`
    Continue(Span),
    /// [expression][Expr]
    Expr(Expr),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self.clone() {
            Self::IfStatement(_, _, span) => span,
            Self::WhileStatement(_, _, span) => span,
            Self::LoopStatement(_, span) => span,
            Self::ImportStatement(_, span) => span,
            Self::Block(_, span) => span,
            Self::VarDeclaration(_, _, _, span) => span,
            Self::ConDeclaration(_, _, _, span) => span,
            Self::AssignStatement(_, _, span) => span,
            Self::Function(_, span) => span,
            Self::Class(_, span) => span,
            Self::Trait(trait_) => trait_.span,
            Self::Constructor(_, span) => span,
            Self::Return(_, span) => span,
            Self::Break(span) => span,
            Self::Continue(span) => span,
            Self::Expr(expr) => expr.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptFun {
    /// Function name.
    pub name: Ident,
    /// Function parameters.
    pub params: Vec<Ident>,
    /// Body of the function.
    pub body: Stmt,
    /// Function parameters' type information.
    pub params_typ: Vec<Option<Expr>>,
    /// Function return type.
    pub return_typ: Option<Expr>,
    /// Function access modifier (e.g. `public`).
    pub access: FunAccess,
}

impl ScriptFun {
    pub fn new(
        name: Ident,
        params: Vec<Ident>,
        body: Stmt,
        params_typ: Vec<Option<Expr>>,
        return_typ: Option<Expr>,
        access: FunAccess,
    ) -> Self {
        ScriptFun {
            name,
            params,
            body,
            params_typ,
            return_typ,
            access,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunAccess {
    Public,
    Private,
}

/// A class declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    /// The name of the class.
    pub name: Ident,
    /// Name of parent class, if any.
    pub parent: Option<Ident>,
    /// List of fields.
    pub fields: Vec<Stmt>,
    /// List of methods.
    pub methods: Vec<Stmt>,
    /// List of constructors.
    pub constructors: Vec<Stmt>,
}

impl Class {
    pub fn new(
        name: Ident,
        parent: Option<Ident>,
        fields: Vec<Stmt>,
        methods: Vec<Stmt>,
        constructors: Vec<Stmt>,
    ) -> Self {
        Self {
            name,
            parent,
            fields,
            methods,
            constructors,
        }
    }

    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Constructor {
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: Stmt,
    pub class: String,
}

impl Constructor {
    pub fn new(name: Ident, params: Vec<Ident>, body: Stmt, class: String) -> Self {
        Self {
            name,
            params,
            body,
            class,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinExpr {
    /// The binary operator.
    pub op: Op,
    /// The left-hand side of the expression.
    pub lhs: Expr,
    /// The right-hand side of the expression.
    pub rhs: Expr,
}

impl BinExpr {
    pub fn new(op: Op, lhs: Expr, rhs: Expr) -> Self {
        BinExpr { op, lhs, rhs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// Addition a + b
    Add,
    /// Subtraction a - b
    Subtract,
    /// Multiplication a * b
    Multiply,
    /// Division a / b
    Divide,
    /// Remainder a % b
    Remainder,
    /// Greater-than check a > b
    GreaterThan,
    /// Greater-than or equal check a >= b
    GreaterThanEquals,
    /// Less-than check a < b
    LessThan,
    /// Less-than or equal check a <= b
    LessThanEquals,
    /// Equality check a == b
    EqualTo,
    /// Inequality check a != b
    NotEqual,
    /// Bitwise and a & b
    BitwiseAnd,
    /// Bitwise or a | b
    BitwiseOr,
    /// Bitwise xor a ^ b
    BitwiseXor,
    /// Falsy check !a
    Bang,
}

impl From<&str> for Op {
    fn from(op: &str) -> Self {
        match op {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            "%" => Op::Remainder,
            ">" => Op::GreaterThan,
            ">=" => Op::GreaterThanEquals,
            "<" => Op::LessThan,
            "<=" => Op::LessThanEquals,
            "==" => Op::EqualTo,
            "!=" => Op::NotEqual,
            "&" => Op::BitwiseAnd,
            "|" => Op::BitwiseOr,
            "^" => Op::BitwiseXor,
            "!" => Op::Bang,
            _ => unreachable!(),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => f.write_str("+"),
            Op::Subtract => f.write_str("-"),
            Op::Multiply => f.write_str("*"),
            Op::Divide => f.write_str("/"),
            Op::Remainder => f.write_str("%"),
            Op::GreaterThan => f.write_str(">"),
            Op::GreaterThanEquals => f.write_str(">="),
            Op::LessThan => f.write_str("<"),
            Op::LessThanEquals => f.write_str("<="),
            Op::EqualTo => f.write_str("=="),
            Op::NotEqual => f.write_str("!="),
            Op::BitwiseAnd => f.write_str("&"),
            Op::BitwiseOr => f.write_str("|"),
            Op::BitwiseXor => f.write_str("^"),
            Op::Bang => f.write_str("!"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

/// An expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// f64
    Number(f64, Span),
    /// String
    String(String, Span),
    /// true|false
    Boolean(bool, Span),
    /// ()
    Unit(Span),
    /// nil
    Nil(Span),
    /// self
    SelfExpr(Span),
    /// id
    Identifier(Ident),
    /// expr `op` expr
    BinExpr(Box<BinExpr>, Span),
    /// `-`|`+`|`!` expr
    UnaryExpr(Op, Box<Expr>, Span),
    /// expr `[` expr `]`
    Index(Box<Expr>, Box<Expr>, Span),
    /// `(` [Expr] `)`
    ParenExpr(Box<Expr>, Span),
    /// [ expr, ... ]
    List(Box<Vec<Expr>>, Span),
    /// `(` expr, ... `)`
    Tuple(Box<Vec<Expr>>, Span),
    /// `{` expr : expr, ... `}`
    Map(Box<Vec<(Expr, Expr)>>, Span),
    /// expr `or` expr
    Or(Box<Expr>, Box<Expr>, Span),
    /// expr `and` expr
    And(Box<Expr>, Box<Expr>, Span),
    /// expr `(` expr, ... `)`
    FunCall(Box<Expr>, Box<Vec<Expr>>, Span),
    /// expr `.` expr
    MemberExpr(Box<Expr>, Box<Expr>, Span),
    /// expr `:` expr
    AssocExpr(Box<Expr>, Box<Expr>, Span),
    /// type
    Type(TypePath, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self.clone() {
            Self::Number(_, span)
            | Self::String(_, span)
            | Self::Boolean(_, span)
            | Self::Unit(span)
            | Self::Nil(span)
            | Self::SelfExpr(span)
            | Self::BinExpr(_, span)
            | Self::UnaryExpr(_, _, span)
            | Self::ParenExpr(_, span)
            | Self::Index(_, _, span)
            | Self::List(_, span)
            | Self::Tuple(_, span)
            | Self::Map(_, span)
            | Self::Or(_, _, span)
            | Self::And(_, _, span)
            | Self::FunCall(_, _, span)
            | Self::MemberExpr(_, _, span)
            | Self::AssocExpr(_, _, span)
            | Self::Type(_, span) => span,
            Self::Identifier(x) => x.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypePath {
    pub ident: Ident,
    pub arguments: Option<Box<TypePath>>,
}
