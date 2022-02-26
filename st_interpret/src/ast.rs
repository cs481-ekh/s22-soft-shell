use std::collections::HashMap;

#[derive(Debug)]
pub enum VariableType {
    INT(i16),
    BOOL(bool),
    REAL(f32),
}

#[derive(Debug)]
pub enum VariableKind {
    NORMAL,
    INPUT,
    OUTPUT,
    IN_OUT,
    EXTERNAL,
    GLOBAL,
}

#[derive(Debug)]
pub enum VarsDec {
    DecList(VariableKind, Box<HashMap<Box<String>, VariableType>>),
}

#[derive(Debug)]
pub enum Assignment {
    Asgn(Box<String>, VariableType),
}

// First arg is name, Second arg is varlist, third is statement list
#[derive(Debug)]
pub enum Program {
    Prog(Box<String>, Option<Vec<Box<VarsDec>>>, Vec<Assignment>),
}
