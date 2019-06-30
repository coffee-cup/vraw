use crate::utils::*;

pub type Ident = String;
pub type Arg = Ident;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub decls: Vec<Decl>,
    pub end: Pos,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    ShapeDecl(Shape),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Shape {
    pub name: Ident,
    pub args: Vec<Arg>,
    pub block: Block,
    pub pos: Pos,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub calls: Vec<FunCall>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedArg {
    pub name: Ident,
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr, Pos),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Name(Ident, Range),
    Literal(Literal, Range),
    Binary(Box<Expr>, BinOp, Box<Expr>, Pos),
    Unary(UnOp, Box<Expr>, Pos),
    Grouping(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunCall {
    pub ident: Ident,
    pub args: Vec<NamedArg>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnOp {
    Neg,
}

pub fn prec(p: Precedence) -> u32 {
    p as u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Precedence {
    Conditional = 20,
    Sum = 30,
    Product = 40,
    Exponent = 50,
    Prefix = 60,
    Postfix = 70,
    Call = 80,
}

impl HasPos for Expr {
    fn pos(&self) -> Pos {
        match self {
            Expr::Name(_, range) => range.start,
            Expr::Literal(_, range) => range.start,
            Expr::Binary(_, _, _, p) => *p,
            Expr::Unary(_, _, p) => *p,
            Expr::Grouping(ref e) => e.pos(),
        }
    }
}

impl HasPos for Stmt {
    fn pos(&self) -> Pos {
        match self {
            Stmt::Expr(_, p) => *p,
        }
    }
}

impl HasPos for FunCall {
    fn pos(&self) -> Pos {
        self.range.start
    }
}

impl HasPos for Shape {
    fn pos(&self) -> Pos {
        self.pos
    }
}
