/// Program state and current execution information.
use crate::ast::{Program, VariableKind, VariableValue};
use crate::read_file;
lalrpop_mod!(pub parser);

use crate::ast::Assignment;
use crate::ast::AstNode;
use crate::ast::Program::Prog;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
/// All information stored about a variable.
pub struct VariableInfo {
    pub var_value: VariableValue,
    pub var_kind: VariableKind,
}

#[derive(Debug, Clone)]
/// Program context struct which stores the symbol table and other long-lived state information of
/// the ST program.
pub struct ProgContext {
    symbols: HashMap<String, VariableInfo>,
    function_context: Option<Box<ProgContext>>,
    function_blocks: HashMap<String, Box<ProgContext>>,
}

impl ProgContext {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            function_context: None,
            function_blocks: HashMap::new(),
        }
    }

    /// Adds a variable to the symbol table with the associated value
    pub fn add_var(&mut self, name: String, kind: VariableKind, value: VariableValue) {
        if let Some(f) = &mut self.function_context {
            f.add_var(name, kind, value);
            return;
        }
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
        let name = name.to_ascii_lowercase();
        if let Some(f) = &mut self.function_context {
            f.update_var(&name, new_value);
            return;
        }
        // retrieve current value
        let current_var_info = self.symbols.remove(&name).expect(&format!(
            "Attempted to update a variable '{}' that does not exist",
            name
        ));

        // disallow updating to a different ST variable type
        if std::mem::discriminant(&current_var_info.var_value) != std::mem::discriminant(&new_value)
        {
            panic!(
                "Cannot change the type of a variable (previous '{:?}', new '{:?})",
                current_var_info.var_value, &new_value
            );
        }

        let new_var_info = VariableInfo {
            var_value: new_value,
            ..current_var_info
        };
        self.symbols.insert(name, new_var_info);
    }

    /// Gets a variable from the symbol table with the given name
    pub fn get_var(&self, name: String) -> Option<&VariableInfo> {
        let name = name.to_ascii_lowercase();
        if let Some(f) = &self.function_context {
            let function_result = f.get_var(name.clone());
            return match function_result {
                Some(_) => function_result,
                _ => {
                    let result = self.symbols.get(&name);
                    match result {
                        Some(var) => {
                            if var.var_kind == VariableKind::GLOBAL {
                                result
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
            };
        }
        self.symbols.get(&name)
    }

    /// Gets all variables from the symbol table, returns an iterator
    pub fn get_all_vars(&self) -> Iter<'_, String, VariableInfo> {
        return match &self.function_context {
            Some(f) => f.get_all_vars(),
            _ => self.symbols.iter(),
        };
    }

    /// Initializes the function context
    pub fn start_function(&mut self) {
        self.function_context = Some(Box::new(ProgContext::new()));
    }

    /// Sets the function context to None
    pub fn end_function(&mut self) {
        self.function_context = None;
    }

    /// Initializes the function block context. If a function block with the same name has
    /// already been called, the context from before is loaded. Otherwise, a new context is created.
    pub fn start_function_block(&mut self, function_name: String) {
        self.function_context = match self
            .function_blocks
            .remove(&function_name.to_ascii_lowercase())
        {
            Some(context) => Some(context),
            _ => Some(Box::new(ProgContext::new())),
        }
    }

    /// Saves the function block context. The function name must be provided.
    pub fn end_function_block(&mut self, function_name: String) {
        if let Some(context) = &self.function_context {
            self.function_blocks
                .insert(function_name.to_ascii_lowercase(), (*context).clone());
        }
        self.function_context = None;
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
pub fn st_program_run(ProgramHandle: &mut ProgHandle) {
    loop {
        let mut ret_val = st_program_step(ProgramHandle);

        if ret_val {
            break;
        }
    }
}

// function steps through one state from list stored in ast
// inputs: Program Handle
// outputs: Boolean used for determining when program is complete. True means you
//          have excecuted all statements in the list. can be expanded to use
//          with error detection
pub fn st_program_step(ProgramHandle: &mut ProgHandle) -> bool {
    //for debugging
    //println!("step: {count}", count = ProgramHandle.statement_counter);

    // get context
    let context: &mut ProgContext = &mut ProgramHandle.context;

    //get statement counter
    let counter: u32 = ProgramHandle.statement_counter;
    let num_usize: usize = counter as usize;

    //get program node
    let program = &ProgramHandle.ast;

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
