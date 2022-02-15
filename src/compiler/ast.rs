use crate::common::Span;

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

pub struct StmtBlock(Vec<Stmt>, Span);

/// A statment
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    /// `if` expr `{` body `}` `else` `{`body `}`
    IfStatement(Expr, Box<(Stmt, Option<Stmt>)>, Span),
    /// `while` expr `{` body `}`
    WhileStatement(Expr, Box<Stmt>, Span),
    /// `loop` `{` body `}`
    LoopStatement(Box<Stmt>, Span),
    /// `{` body `}`
    Block(Box<Vec<Stmt>>, Span),
    /// `var` id [`:` type] `=` expr
    VarDeclaration(Ident, Option<Expr>, Option<Expr>, Span),
    /// `con` id [`:` type] `=` expr
    ConDeclaration(Ident, Expr, Option<Expr>, Span),
    /// expr `=` expr
    AssignStatement(Expr, Expr, Span),
    /// `fun` id `(` ...args `)` `{` body `}`
    ScriptFun(Box<ScriptFun>, Span),
    /// `class` id `{` method | field `}`
    Class(Class, Span),
    /// `fun` method `(` self, ...args `)` `{` body `}`
    //Method(Box<Method>, Span),
    /// `const` name `(` ...args `)` `{` body `}`
    Constructor(Box<Constructor>, Span),
    /// `return` expr
    Return(Expr, Span),
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
            Self::Block(_, span) => span,
            Self::VarDeclaration(_, _, _, span) => span,
            Self::ConDeclaration(_, _, _, span) => span,
            Self::AssignStatement(_, _, span) => span,
            Self::ScriptFun(_, span) => span,
            Self::Class(_, span) => span,
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
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: Stmt,
    pub access: FunAccess,
}

impl ScriptFun {
    pub fn new(name: Ident, params: Vec<Ident>, body: Stmt, access: FunAccess) -> Self {
        ScriptFun {
            name,
            params,
            body,
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
    pub op: Op,
    pub lhs: Expr,
    pub rhs: Expr,
}

impl BinExpr {
    pub fn new(op: Op, lhs: Expr, rhs: Expr) -> Self {
        BinExpr { op, lhs, rhs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    EqualTo,
    NotEqual,
    Bang,
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
    /// id
    Identifier(Ident),
    /// expr `op` expr
    BinExpr(Box<BinExpr>, Span),
    /// `-`|`+`|`!` expr
    UnaryExpr(Op, Box<Expr>, Span),
    /// expr `[` expr `]`
    Index(Box<Expr>, Box<Expr>, Span),
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
    /// type
    Type(Ident),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self.clone() {
            Self::Number(_, span)
            | Self::String(_, span)
            | Self::Boolean(_, span)
            | Self::Unit(span)
            | Self::Nil(span)
            | Self::BinExpr(_, span)
            | Self::UnaryExpr(_, _, span)
            | Self::Index(_, _, span)
            | Self::List(_, span)
            | Self::Tuple(_, span)
            | Self::Map(_, span)
            | Self::Or(_, _, span)
            | Self::And(_, _, span)
            | Self::FunCall(_, _, span)
            | Self::MemberExpr(_, _, span) => span,
            Self::Identifier(x) | Self::Type(x) => x.span(),
        }
    }
}
