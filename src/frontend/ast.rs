pub type Name = String;

#[derive(Debug, Clone)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    And,
    Or,
}

type Args = Vec<Expr>;
type TypArgs = Vec<Type>;

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i32),
    BoolLit(bool),
    StrLit(String),
    BinOp(Box<Expr>, Bop, Box<Expr>),
    Var(Name),
    FuncCall(Option<Box<Expr>>, Name, Option<TypArgs>, Args), // optional callee
}

#[derive(Debug, Clone)]
pub enum PreType {
    I32,
    Bool,
    String,
    List(Box<Type>),
}

pub type Type = (bool, PreType);

pub enum Stmt {
    Let(Name, Option<Type>, Expr),
    Assign(Expr, Expr),
    IfThenElse(Expr, Body, Body),
    ForEach(Expr, Body),
}
pub enum Body {
    Block(Vec<Stmt>),
    ExprBody(Expr),
}

type Params = Vec<(Name, Option<Type>)>;
type TypParams = Vec<Name>;
type Ctors = Vec<(Name, Vec<Type>)>;

pub enum Toplvl {
    Func(Name, TypParams, Params, Body),
    Enum(Name, TypParams, Ctors),
}
