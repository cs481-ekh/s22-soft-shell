/// Program state and current execution information.
use crate::ast::{Program, VariableKind, VariableValue};
use crate::read_file;
lalrpop_mod!(pub parser);

use crate::ast::Assignment;
use crate::ast::AstNode;
use crate::ast::Program::Prog;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
/// All information stored about a variable.
pub struct VariableInfo {
    pub var_value: VariableValue,
    pub var_kind: VariableKind,
}

#[derive(Debug)]
/// Program context struct which stores the symbol table and other long-lived state information of
/// the ST program.
pub struct ProgContext {
    symbols: HashMap<String, VariableInfo>,
}

impl ProgContext {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    /// Adds a variable to the symbol table with the associated value. If there is already a variable with the given name, the value is updated.
    pub fn add_var(&mut self, name: String, kind: VariableKind, value: VariableValue) {
        let var_info = VariableInfo {
            var_value: value,
            var_kind: kind,
        };

        let var = &name.to_ascii_lowercase();

        // disallow naming more than one variable by the same name
        if self.symbols.contains_key(var) {
            panic!("A variable already exists with this name");
        }

        self.symbols.insert(name.to_ascii_lowercase(), var_info);
    }

    /// Update a variable's value in the symbol table if possible
    pub fn update_var(&mut self, name: &str, new_value: VariableValue) {
        // retrieve current value
        let current_var_info = self
            .symbols
            .remove(name)
            .expect("Attempted to update a variable that does not exist");

        // disallow updating to a different ST variable type
        if std::mem::discriminant(&current_var_info.var_value) != std::mem::discriminant(&new_value)
        {
            panic!("Cannot change the type of a variable");
        }

        let new_var_info = VariableInfo {
            var_value: new_value,
            ..current_var_info
        };
        self.symbols.insert(String::from(name), new_var_info);
    }

    /// Gets a variable from the symbol table with the given name
    pub fn get_var(&mut self, name: String) -> Option<&VariableInfo> {
        self.symbols.get(&name)
    }

    /// Gets all variables from the symbol table, returns an iterator
    pub fn get_all_vars(&mut self) -> Iter<'_, String, VariableInfo> {
        self.symbols.iter()
    }
}

#[derive(Debug)]
/// Main struct for controlling program execution
pub struct ProgHandle {
    statement_counter: u32,
    ast: Program,
    pub context: ProgContext,
}

/// Load in an ST file and set up a handle to execute it
pub fn st_program_load(filename: &str, context: ProgContext) -> ProgHandle {
    let program_ast = parser::ProgramParser::new()
        .parse(&read_file(filename))
        .unwrap();
    ProgHandle {
        statement_counter: 0,
        ast: program_ast,
        context,
    }
}

/// Run a ST file
/// ProgramHandle prog_handle = st_program_load(“testprogram.st”, context);
pub fn st_program_run(mut ProgramHandle: ProgHandle) {
    let context: &mut ProgContext = &mut ProgramHandle.context;

    let program = ProgramHandle.ast;

    program.execute(context);

    // Once I have the code above working, I was going to try below
    // loop {
    //     let mut ret_val = st_program_step(ProgHandle);
    //
    //     if ret_val {
    //         // For debugging
    //         println!("Program ran successfully");
    //         break;
    //     }
    // }
}

// function steps through one state from list stored in ast
// inputs: Program Handle
// outputs: Boolean used for determining when program is complete. True means you
//          have excecuted all statements in the list. can be expanded to use
//          with error detection
pub fn st_program_step(mut ProgramHandle: ProgHandle) -> bool {
    //for debugging
    //println!("step: {count}", count = ProgramHandle.statement_counter);

    // get context
    let context: &mut ProgContext = &mut ProgramHandle.context;

    //get statement counter
    let counter: u32 = ProgramHandle.statement_counter;
    let num_usize: usize = counter as usize;

    //get program node
    let program = ProgramHandle.ast;

    //use to get access to Vec<Assignments> as statements
    let Prog(_, all_dec_lists, statements) = program;

    //if first step then excecute declarations list
    if counter == 0 {
        if let Some(program_dec_lists) = all_dec_lists {
            for dec_list in program_dec_lists {
                dec_list.execute(context);
            }
        }
    }

    //execute current statement
    let statement: Assignment = statements[num_usize].clone();
    statement.execute(context);

    //check if program is complete
    if num_usize < statements.len() - 1 {
        //increment to the next statement
        ProgramHandle.statement_counter += 1;
    } else {
        return true;
    }

    false
}
