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
    Asgn(Box<String>, VariableValue),
}

impl AstNode for Assignment {
    fn execute(self, context: &mut ProgContext) -> Option<VariableValue> {
        let Asgn(var_name, new_value) = self;
        let var_name = *var_name;

        context.update_var(&var_name, new_value.clone());

        None
    }
}

/// AST root node containing an entire ST program.
/// First arg is name, Second arg is varlist, third is statement list
#[derive(Debug)]
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
