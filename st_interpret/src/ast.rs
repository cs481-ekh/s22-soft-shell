//! AST node definitions

use crate::ast::Assignment::Asgn;
use crate::ast::Program::Prog;
use crate::ast::VarsDec::DecList;
use crate::prog_handle::ProgContext;
use chrono::naive::{NaiveDate, NaiveTime};
use std::collections::HashMap;
use std::time::Duration;

/// Trait containing functionality for executable AST nodes
trait AstNode {
    /// Execute this node in the given context
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue>;
}

#[derive(Debug, Clone, PartialEq)]
/// ST variable types, each holding a corresponding value of that type.
pub enum VariableValue {
    INT(i16),
    BOOL(bool),
    BYTE(u8),
    WORD(u16),
    UINT(u16),
    DWORD(u32),
    DINT(i32),
    REAL(f32),
    LREAL(f64),
    CHAR(u8),
    WCHAR(u16),
    STRING(String),
    TIME(Duration),
    LTIME(Duration),
    DATE(NaiveDate),
    TIME_OF_DAY(NaiveTime),
}

// Start of expressions

#[derive(Debug)]
/// Root node of expression branch.
/// Note that order of operations is captured in a leaf first resolution
/// of the various child nodes.
pub enum Expression {
    Expr(XOR_Expression, Option<XOR_Expression>),
}

#[derive(Debug)]
pub enum XOR_Expression {
    Xor(AND_Expression, Option<AND_Expression>),
}

#[derive(Debug)]
pub enum AND_Expression {
    And(Comparison, Option<Comparison>),
}

#[derive(Debug)]
pub enum Comparison {
    Comp(EquExpression, Option<EquExpression>),
}

#[derive(Debug)]
pub enum EquExpression {
    Equ(AddExpression, Option<(ComparisonOperator, AddExpression)>),
}

#[derive(Debug)]
pub enum AddExpression {
    Add(Term, Option<(AddOperator, Term)>),
}

#[derive(Debug)]
pub enum Term {
    Term(PowerExpression, Option<(MultiplyOperator, PowerExpression)>),
}

#[derive(Debug)]
pub enum PowerExpression {
    Power(UnaryExpression, Option<UnaryExpression>),
}

#[derive(Debug)]
pub enum UnaryExpression {
    Unary(PrimaryExpression, Option<UnaryOperator>), //Note order flipped for consistency
}

#[derive(Debug)]
pub enum PrimaryExpression {
    Const(VariableValue),
    Label(Box<String>),
    Expr(Box<Expression>),
}

// Start of expression operators
#[derive(Debug)]
/// Different comparison operators for resolving boolean expressions
pub enum ComparisonOperator {
    LESS_THAN,
    GREATER_THAN,
    LESS_EQUAL_THAN,
    GREATER_EQUAL_THAN,
}

#[derive(Debug)]
/// Operators at the precedence of addition
pub enum AddOperator {
    ADD,
    SUBTRACT,
}

#[derive(Debug)]
/// Operators at the precedence of multiplication
pub enum MultiplyOperator {
    MULTIPLY,
    DIVIDE,
    MODULO,
}

#[derive(Debug)]
/// Operators at the precedence of unary operations
pub enum UnaryOperator {
    NEGATIVE,
    NOT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Different 'kinds' of ST variables, such as input, output, etc.
pub enum VariableKind {
    NORMAL,
    INPUT,
    OUTPUT,
    IN_OUT,
    EXTERNAL,
    GLOBAL,
}
// End of expressions and operators

#[derive(Debug)]
/// A list of variable declarations of a certain variable kind
pub enum VarsDec {
    DecList(VariableKind, Box<HashMap<Box<String>, VariableValue>>),
}

impl AstNode for VarsDec {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let DecList(kind, decs) = self;
        for var_dec in decs.iter() {
            let var_name = (**var_dec.0).clone();
            let var_type = &*var_dec.1;

            context.add_var(var_name, kind, var_type.clone());
        }

        None
    }
}

#[derive(Debug)]
/// A single assignment statement.
pub enum Assignment {
    Asgn(Box<String>, Expression),
}
// TODO: Uncomment during subset 4 execution implementation, broken due change in the Assignment enum
// impl AstNode for Assignment {
//     fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
//         let Asgn(var_name, new_value) = self;
//         let var_name = *var_name;
//
//         context.update_var(&var_name, new_value.clone());
//
//         None
//     }
// }

/// AST root node containing an entire ST program.
/// First arg is name, Second arg is varlist, third is statement list
#[derive(Debug)]
pub enum Program {
    Prog(Box<String>, Option<Vec<Box<VarsDec>>>, Vec<Assignment>),
}

// TODO: Uncomment during subset 4 execution implementation, broken due change in the Assignment enum
// impl AstNode for Program {
//     fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
//         let Prog(_, all_dec_lists, statements) = self;
//
//         // process variable declarations lists if present
//         if let Some(program_dec_lists) = all_dec_lists {
//             for dec_list in program_dec_lists {
//                 dec_list.execute(context);
//             }
//         }
//
//         // execute all statements (assignments) sequentially
//         for statement in statements {
//             statement.execute(context);
//         }
//
//         // this is the top level, so no evaluation value
//         None
//     }
// }
