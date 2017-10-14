#![allow(dead_code)] // TODO delete when things are used
///! Abstract syntax tree representation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use value::{Value, IdType};
use op::{OpCode, CompOp};

/// A Roller expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    /// Value literal
    Val(Value),
    /// Identifier reference
    Id(IdType),
    /// Variable declaration
    Decl(IdType, Box<Expr>),
    /// Variable assignment
    Assign(IdType, Box<Expr>),
    /// Comparison
    Comp {
        op: CompOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>
    },
    /// Function call.
    /// Also applies operators since they are built-in functions.
    Op(FunCall),
    /// List of expressions, will evaluate to `Value::List`.
    List(Vec<Expr>),
    Set(BTreeSet<Expr>),
    Map(BTreeMap<Expr, Expr>),
    /// Control structures.
    Ctrl(Control),
    Distribution(Vec<(Expr, Expr)>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Control {
    Break,
    Continue,
    If {
        cond_expr: Box<Expr>,
        then_expr: Box<Expr>,
        elif_exprs: Vec<Expr>,
        else_expr: Box<Expr>,
    },
    Loop {
        body: Box<Expr>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    For {
        iterator: IdType,
        iterable: Box<Expr>,
        body: Box<Expr>,
    },
    Try {
        expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
}

/// A function application with ordered and/or named arguments.
/// 
/// Not to be confused with `PrankCall`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunCall {
    /// Name of the function or the operator.
    pub code: OpCode,
    /// The vector of the ordered arguments.
    pub args: Vec<Expr>,
    /// The vector of named arguments.
    pub kw_args: Vec<(IdType, Expr)>,
}

impl FunCall {
    /// Create a new function call with the given ordered, and named, arguments.
    pub fn new(code: OpCode,
               args: Vec<Expr>,
               kw_args: Vec<(IdType, Expr)>)
               -> Self
    {
        FunCall {
            code: code,
            args: args,
            kw_args: kw_args,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Val(ref x) => write!(f, "{}", x),
            &Expr::Id(ref x) => write!(f, "{}", x),
            _ => write!(f, "{:?}", self) // TODO
        }
    }
}
