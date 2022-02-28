//! AST node definitions

use std::collections::HashMap;

#[derive(Debug)]
/// ST variable, with enum variants representing ST types and holding the corresponding value
pub enum VariableType {
    INT(i16),
    BOOL(bool),
    REAL(f32),
}

#[derive(Debug)]
/// Different 'kinds' of ST variables, such as input, output, etc.
pub enum VariableKind {
    NORMAL,
    INPUT,
    OUTPUT,
    IN_OUT,
    EXTERNAL,
    GLOBAL,
}

#[derive(Debug)]
/// A list of variable declarations of a certain variable kind
pub enum VarsDec {
    DecList(VariableKind, Box<HashMap<Box<String>, VariableType>>),
}

#[derive(Debug)]
/// A single assignment statement.
pub enum Assignment {
    Asgn(Box<String>, VariableType),
}

/// AST root node containing an entire ST program.
/// First arg is name, Second arg is varlist, third is statement list
#[derive(Debug)]
pub enum Program {
    Prog(Box<String>, Option<Vec<Box<VarsDec>>>, Vec<Assignment>),
}
