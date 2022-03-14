//! AST node definitions

use crate::ast::AND_Expression::And;
use crate::ast::AddExpression::Add;
use crate::ast::Assignment::Asgn;
use crate::ast::BoolOp::XOR;
use crate::ast::Comparison::Comp_Eq;
use crate::ast::ComparisonOperator::*;
use crate::ast::EquExpression::Equ;
use crate::ast::Expression::Expr;
use crate::ast::PowerExpression::Power;
use crate::ast::Program::Prog;
use crate::ast::Term::Term as TermInstance;
use crate::ast::UnaryExpression::Unary;
use crate::ast::VariableValue::*;
use crate::ast::VarsDec::DecList;
use crate::ast::XOR_Expression::Xor;
use crate::prog_handle::ProgContext;
use chrono::naive::{NaiveDate, NaiveTime};
use num_traits::checked_pow;
use std::collections::HashMap;
use std::time::Duration;

/// Trait containing functionality for executable AST nodes
pub trait AstNode {
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

#[derive(Debug, Clone)]
/// Root node of expression branch.
/// Note that order of operations is captured in a leaf first resolution
/// of the various child nodes.
pub enum Expression {
    Expr(XOR_Expression, Option<XOR_Expression>),
}

impl AstNode for Expression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Expr(left, right) = self;
        let right = match right {
            Some(expr) => Some(expr.execute(context).unwrap()),
            None => None,
        };
        Some(boolean_operation_result(
            left.execute(context).unwrap(),
            BoolOp::OR,
            right,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum XOR_Expression {
    Xor(AND_Expression, Option<AND_Expression>),
}

impl AstNode for XOR_Expression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Xor(left, right) = self;
        let right = match right {
            Some(expr) => Some(expr.execute(context).unwrap()),
            None => None,
        };
        Some(boolean_operation_result(
            left.execute(context).unwrap(),
            BoolOp::XOR,
            right,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum AND_Expression {
    And(Comparison, Option<Comparison>),
}

impl AstNode for AND_Expression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let And(left, right) = self;
        let right = match right {
            Some(expr) => Some(expr.execute(context).unwrap()),
            None => None,
        };
        Some(boolean_operation_result(
            left.execute(context).unwrap(),
            BoolOp::AND,
            right,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Comparison {
    Comp_Eq(EquExpression, Option<(bool, EquExpression)>),
}

impl AstNode for Comparison {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        // TODO: just performs an equals comparison right now, but this node should be able to represent both equals and not-equals comparison
        let Comp_Eq(left, op_and_right) = self;
        let left = left.execute(context).unwrap();
        if let Some((is_equals, right)) = op_and_right {
            let right = right.execute(context).unwrap();
            let result = if is_equals {
                left == right
            } else {
                left != right
            };
            Some(BOOL(result))
        } else {
            Some(left)
        }
    }
}

#[derive(Debug, Clone)]
pub enum EquExpression {
    Equ(AddExpression, Option<(ComparisonOperator, AddExpression)>),
}

impl AstNode for EquExpression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Equ(left, op_and_right) = self;
        let left = left.execute(context).unwrap();
        if let Some((op, right)) = op_and_right {
            let right = right.execute(context).unwrap();
            Some(math_operation_result(left, MathOp::Comparison(op), right))
        } else {
            Some(left)
        }
    }
}

#[derive(Debug, Clone)]
pub enum AddExpression {
    Add(Term, Option<(AddOperator, Term)>),
}

impl AstNode for AddExpression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Add(left, op_and_right) = self;
        let left = left.execute(context).unwrap();
        if let Some((op, right)) = op_and_right {
            let right = right.execute(context).unwrap();
            Some(math_operation_result(left, MathOp::Add(op), right))
        } else {
            Some(left)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    Term(PowerExpression, Option<(MultiplyOperator, PowerExpression)>),
}

impl AstNode for Term {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let TermInstance(left, op_and_right) = self;
        let left = left.execute(context).unwrap();
        if let Some((op, right)) = op_and_right {
            let right = right.execute(context).unwrap();
            Some(math_operation_result(left, MathOp::Multiply(op), right))
        } else {
            Some(left)
        }
    }
}

#[derive(Debug, Clone)]
pub enum PowerExpression {
    Power(UnaryExpression, Option<UnaryExpression>),
}

impl AstNode for PowerExpression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Power(left, right) = self;
        let left = left.execute(context).unwrap();
        let result = match right {
            Some(right) => {
                let right = right.execute(context).unwrap();
                let exponent = match right {
                    INT(x) => x,
                    _ => panic!("Only integers supported as exponents"),
                };
                math_operation_result(left, MathOp::Exponentiate, right)
            }
            None => left,
        };

        Some(result)
    }
}

#[derive(Debug, Clone)]
pub enum UnaryExpression {
    Unary(PrimaryExpression, Option<UnaryOperator>), //Note order flipped for consistency
}

impl AstNode for UnaryExpression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Unary(expression, operator) = self;
        let expressionValue = expression.execute(context).unwrap();
        let result = match operator {
            Some(op) => match op {
                UnaryOperator::NEGATIVE => match expressionValue {
                    INT(x) => INT(-x),
                    DINT(x) => DINT(-x),
                    REAL(x) => REAL(-x),
                    LREAL(x) => LREAL(-x),
                    _ => {
                        panic!("Attempted to negate a type that cannot be negated")
                    }
                },
                UnaryOperator::NOT => match expressionValue {
                    BOOL(x) => BOOL(!x),
                    _ => {
                        panic!("Attempted to invert a non-boolean value")
                    }
                },
            },
            None => expressionValue,
        };

        Some(result)
    }
}

#[derive(Debug, Clone)]
pub enum PrimaryExpression {
    Const(VariableValue),
    VarName(Box<String>),
    Expr(Box<Expression>),
}

impl AstNode for PrimaryExpression {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        match self {
            PrimaryExpression::Const(value) => Some(value),
            PrimaryExpression::VarName(var_name) => {
                Some(context.get_var(*var_name).unwrap().var_value.clone())
            }
            PrimaryExpression::Expr(expression) => Some(expression.execute(context).unwrap()),
        }
    }
}

// Start of expression operators
#[derive(Debug, Clone)]
/// Different comparison operators for resolving boolean expressions
pub enum ComparisonOperator {
    LESS_THAN,
    GREATER_THAN,
    LESS_EQUAL_THAN,
    GREATER_EQUAL_THAN,
}

#[derive(Debug, Clone)]
/// Operators at the precedence of addition
pub enum AddOperator {
    ADD,
    SUBTRACT,
}

#[derive(Debug, Clone)]
/// Operators at the precedence of multiplication
pub enum MultiplyOperator {
    MULTIPLY,
    DIVIDE,
    MODULO,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
/// A single assignment statement.
pub enum Assignment {
    Asgn(Box<String>, Expression),
}

impl AstNode for Assignment {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Asgn(var_name, new_value) = self;
        let var_name = *var_name;

        let new_value = new_value.execute(context).unwrap();
        context.update_var(&var_name, new_value);

        None
    }
}

/// AST root node containing an entire ST program.
/// First arg is name, Second arg is varlist, third is statement list
#[derive(Debug, Clone)]
pub enum Program {
    Prog(Box<String>, Option<Vec<Box<VarsDec>>>, Vec<Assignment>),
}

impl AstNode for Program {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Prog(_, all_dec_lists, statements) = self;

        // process variable declarations lists if present
        if let Some(program_dec_lists) = all_dec_lists {
            for dec_list in program_dec_lists {
                dec_list.execute(context);
            }
        }

        // execute all statements (assignments) sequentially
        for statement in statements {
            statement.execute(context);
        }

        // this is the top level, so no evaluation value
        None
    }
}

pub enum MathOp {
    Multiply(MultiplyOperator),
    Add(AddOperator),
    Exponentiate,
    Comparison(ComparisonOperator),
}

pub enum BoolOp {
    XOR,
    OR,
    AND,
}

fn boolean_operation_result(
    left: VariableValue,
    op: BoolOp,
    right: Option<VariableValue>,
) -> VariableValue {
    if let Some(right) = right {
        let (left, right) = match (left, right) {
            (BOOL(left), BOOL(right)) => (left, right),
            (_, _) => panic!("Attempted boolean operation with non-boolean values"),
        };
        BOOL(match op {
            BoolOp::XOR => left ^ right,
            BoolOp::OR => left | right,
            BoolOp::AND => left & right,
        })
    } else {
        left
    }
}

fn math_operation_result(left: VariableValue, op: MathOp, right: VariableValue) -> VariableValue {
    match op {
        MathOp::Multiply(MultiplyOperator::MULTIPLY) => match (left, right) {
            (INT(x), INT(y)) => INT(x * y),
            (BYTE(x), BYTE(y)) => BYTE(x * y),
            (WORD(x), WORD(y)) => WORD(x * y),
            (UINT(x), UINT(y)) => UINT(x * y),
            (DWORD(x), DWORD(y)) => DWORD(x * y),
            (DINT(x), DINT(y)) => DINT(x * y),
            (REAL(x), REAL(y)) => REAL(x * y),
            (LREAL(x), LREAL(y)) => LREAL(x * y),
            (CHAR(x), CHAR(y)) => CHAR(x * y),
            (WCHAR(x), WCHAR(y)) => WCHAR(x * y),
            (_, _) => panic!("Attempted to multiply incompatible types"),
        },
        MathOp::Multiply(MultiplyOperator::DIVIDE) => match (left, right) {
            (INT(x), INT(y)) => INT(x / y),
            (BYTE(x), BYTE(y)) => BYTE(x / y),
            (WORD(x), WORD(y)) => WORD(x / y),
            (UINT(x), UINT(y)) => UINT(x / y),
            (DWORD(x), DWORD(y)) => DWORD(x / y),
            (DINT(x), DINT(y)) => DINT(x / y),
            (REAL(x), REAL(y)) => REAL(x / y),
            (LREAL(x), LREAL(y)) => LREAL(x / y),
            (CHAR(x), CHAR(y)) => CHAR(x / y),
            (WCHAR(x), WCHAR(y)) => WCHAR(x / y),
            (_, _) => panic!("Attempted to divide incompatible types"),
        },
        MathOp::Multiply(MultiplyOperator::MODULO) => match (left, right) {
            (INT(x), INT(y)) => INT(x % y),
            (_, _) => panic!("Attempted to take mod of incompatible types"),
        },
        MathOp::Add(AddOperator::ADD) => match (left, right) {
            (INT(x), INT(y)) => INT(x + y),
            (BYTE(x), BYTE(y)) => BYTE(x + y),
            (WORD(x), WORD(y)) => WORD(x + y),
            (UINT(x), UINT(y)) => UINT(x + y),
            (DWORD(x), DWORD(y)) => DWORD(x + y),
            (DINT(x), DINT(y)) => DINT(x + y),
            (REAL(x), REAL(y)) => REAL(x + y),
            (LREAL(x), LREAL(y)) => LREAL(x + y),
            (CHAR(x), CHAR(y)) => CHAR(x + y),
            (WCHAR(x), WCHAR(y)) => WCHAR(x + y),
            (_, _) => panic!("Attempted to add incompatible types"),
        },
        MathOp::Add(AddOperator::SUBTRACT) => match (left, right) {
            (INT(x), INT(y)) => INT(x - y),
            (BYTE(x), BYTE(y)) => BYTE(x - y),
            (WORD(x), WORD(y)) => WORD(x - y),
            (UINT(x), UINT(y)) => UINT(x - y),
            (DWORD(x), DWORD(y)) => DWORD(x - y),
            (DINT(x), DINT(y)) => DINT(x - y),
            (REAL(x), REAL(y)) => REAL(x - y),
            (LREAL(x), LREAL(y)) => LREAL(x - y),
            (CHAR(x), CHAR(y)) => CHAR(x - y),
            (WCHAR(x), WCHAR(y)) => WCHAR(x - y),
            (_, _) => panic!("Attempted to subtract incompatible types"),
        },
        MathOp::Exponentiate => match (left, right) {
            (BYTE(x), BYTE(y)) => BYTE(checked_pow(x, y as usize).unwrap()),
            (WORD(x), WORD(y)) => WORD(checked_pow(x, y as usize).unwrap()),
            (UINT(x), UINT(y)) => UINT(checked_pow(x, y as usize).unwrap()),
            (DWORD(x), DWORD(y)) => DWORD(checked_pow(x, y as usize).unwrap()),
            (CHAR(x), CHAR(y)) => CHAR(checked_pow(x, y as usize).unwrap()),
            (WCHAR(x), WCHAR(y)) => WCHAR(checked_pow(x, y as usize).unwrap()),
            (_, _) => panic!("Attempted to exponentiate incompatible types"),
        },
        MathOp::Comparison(comparison) => match comparison {
            LESS_THAN => match (left, right) {
                (INT(x), INT(y)) => BOOL(x < y),
                (BYTE(x), BYTE(y)) => BOOL(x < y),
                (WORD(x), WORD(y)) => BOOL(x < y),
                (UINT(x), UINT(y)) => BOOL(x < y),
                (DWORD(x), DWORD(y)) => BOOL(x < y),
                (DINT(x), DINT(y)) => BOOL(x < y),
                (REAL(x), REAL(y)) => BOOL(x < y),
                (LREAL(x), LREAL(y)) => BOOL(x < y),
                (CHAR(x), CHAR(y)) => BOOL(x < y),
                (WCHAR(x), WCHAR(y)) => BOOL(x < y),
                (_, _) => panic!("Attempted to add incompatible types"),
            },
            GREATER_THAN => match (left, right) {
                (INT(x), INT(y)) => BOOL(x > y),
                (BYTE(x), BYTE(y)) => BOOL(x > y),
                (WORD(x), WORD(y)) => BOOL(x > y),
                (UINT(x), UINT(y)) => BOOL(x > y),
                (DWORD(x), DWORD(y)) => BOOL(x > y),
                (DINT(x), DINT(y)) => BOOL(x > y),
                (REAL(x), REAL(y)) => BOOL(x > y),
                (LREAL(x), LREAL(y)) => BOOL(x > y),
                (CHAR(x), CHAR(y)) => BOOL(x > y),
                (WCHAR(x), WCHAR(y)) => BOOL(x > y),
                (_, _) => panic!("Attempted to add incompatible types"),
            },
            LESS_EQUAL_THAN => match (left, right) {
                (INT(x), INT(y)) => BOOL(x <= y),
                (BYTE(x), BYTE(y)) => BOOL(x <= y),
                (WORD(x), WORD(y)) => BOOL(x <= y),
                (UINT(x), UINT(y)) => BOOL(x <= y),
                (DWORD(x), DWORD(y)) => BOOL(x <= y),
                (DINT(x), DINT(y)) => BOOL(x <= y),
                (REAL(x), REAL(y)) => BOOL(x <= y),
                (LREAL(x), LREAL(y)) => BOOL(x <= y),
                (CHAR(x), CHAR(y)) => BOOL(x <= y),
                (WCHAR(x), WCHAR(y)) => BOOL(x <= y),
                (_, _) => panic!("Attempted to add incompatible types"),
            },
            GREATER_EQUAL_THAN => match (left, right) {
                (INT(x), INT(y)) => BOOL(x >= y),
                (BYTE(x), BYTE(y)) => BOOL(x >= y),
                (WORD(x), WORD(y)) => BOOL(x >= y),
                (UINT(x), UINT(y)) => BOOL(x >= y),
                (DWORD(x), DWORD(y)) => BOOL(x >= y),
                (DINT(x), DINT(y)) => BOOL(x >= y),
                (REAL(x), REAL(y)) => BOOL(x >= y),
                (LREAL(x), LREAL(y)) => BOOL(x >= y),
                (CHAR(x), CHAR(y)) => BOOL(x >= y),
                (WCHAR(x), WCHAR(y)) => BOOL(x >= y),
                (_, _) => panic!("Attempted to add incompatible types"),
            },
        },
    }
}
