/// Program state and current execution information.
use crate::ast::{Program, VariableKind, VariableValue};
use crate::read_file;
lalrpop_mod!(pub parser);
use std::collections::HashMap;

/// All information stored about a variable.
struct VariableInfo {
    var_value: VariableValue,
    var_kind: VariableKind,
}

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

        self.symbols.insert(name, var_info);
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
}

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
