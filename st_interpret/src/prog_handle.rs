//! Module containing the program context and program handle that control the interpretation of
//! the Structured Text files.
//!
//! Program context information goes here.
//!
//! Program handle controls the execution of the Structured Text program. A Structured Text program
//! is first loaded using the method [st_program_load] which load the program context and creates a
//! handle for the program. Once the program is loaded, [st_program_run] is called and executes the
//! program while updating the context as it runs. When the programming is being executed, the
//! method [st_program_step] is called to step through the code line-by-line while updating the
//! context.
//!
use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
use std::path::{Path, MAIN_SEPARATOR};

/// Program state and current execution information.
use crate::ast::{
    IfStatement, IterationStatement, Program, SelectionStatement, Statement, VariableKind,
    VariableValue, WhileStatement,
};
//use crate::ast::AssignmentStatement;
use crate::ast::ExecutableAstNode;
use crate::ast::Function;
use crate::ast::Program::Prog;
use crate::read_file;
use crate::ST_FILE_EXTENSION;

lalrpop_mod!(pub parser);

#[derive(Debug, PartialEq, Clone)]
#[repr(C)]
/// All information stored about a variable.
pub struct VariableInfo {
    pub var_value: VariableValue,
    pub var_kind: VariableKind,
}

#[derive(Debug, Clone)]
// #[repr(C)]
/// Program context struct which stores the symbol table and other long-lived state information of
/// the ST program.
pub struct ProgContext {
    symbols: HashMap<String, VariableInfo>,
    function_context: Option<Box<ProgContext>>,
    function_blocks: HashMap<String, Box<ProgContext>>,
    functions: HashMap<String, Function>,
}

impl ProgContext {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            function_context: None,
            function_blocks: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Adds a variable to the symbol table with the associated value
    pub fn add_var(
        &mut self,
        name: String,
        kind: VariableKind,
        value: VariableValue,
    ) -> InterpreterResult<()> {
        if let Some(f) = &mut self.function_context {
            return f.add_var(name, kind, value);
        }
        let var_info = VariableInfo {
            var_value: value,
            var_kind: kind,
        };

        let var = &name.to_ascii_lowercase();

        // disallow naming more than one variable by the same name
        if self.symbols.contains_key(var) {
            return Err(String::from("A variable already exists with this name"));
        }

        self.symbols.insert(name.to_ascii_lowercase(), var_info);
        InterpreterResult::Ok(())
    }

    /// Update a variable's value in the symbol table if possible
    pub fn update_var(&mut self, name: &str, new_value: VariableValue) -> InterpreterResult<()> {
        let name = name.to_ascii_lowercase();
        if let Some(f) = &mut self.function_context {
            f.update_var(&name, new_value)?;
            return InterpreterResult::Ok(());
        }
        // retrieve current value
        let current_var_info = self.symbols.remove(&name).ok_or(format!(
            "Attempted to update a variable '{}' that does not exist",
            name
        ))?;

        let mut up_new_value = new_value;

        // disallow updating to a different ST variable type
        if std::mem::discriminant(&current_var_info.var_value)
            != std::mem::discriminant(&up_new_value)
        {
            // implicit cast from int to real
            if matches!(current_var_info.var_value, VariableValue::INT(_))
                && matches!(up_new_value, VariableValue::REAL(_))
            {
                if let VariableValue::REAL(val) = up_new_value {
                    up_new_value = VariableValue::INT(val.trunc() as i16);
                }
            } else if matches!(current_var_info.var_value, VariableValue::REAL(_))
                && matches!(up_new_value, VariableValue::INT(_))
            {
                if let VariableValue::INT(val) = up_new_value {
                    up_new_value = VariableValue::REAL(val as f32);
                }
            } else if matches!(current_var_info.var_value, VariableValue::BOOL(_))
                && matches!(up_new_value, VariableValue::INT(_))
            {
                if let VariableValue::INT(val) = up_new_value {
                    if val == 1 {
                        up_new_value = VariableValue::BOOL(true);
                    } else if val == 0 {
                        up_new_value = VariableValue::BOOL(false);
                    } else {
                        return InterpreterResult::Err(format!(
                            "Cannot change the type of a variable (previous '{:?}', new '{:?})",
                            current_var_info.var_value, &up_new_value
                        ));
                    }
                }
            } else if matches!(current_var_info.var_value, VariableValue::INT(_))
                && matches!(up_new_value, VariableValue::BOOL(_))
            {
                if let VariableValue::BOOL(val) = up_new_value {
                    up_new_value = VariableValue::INT(val as i16);
                }
            } else {
                return InterpreterResult::Err(format!(
                    "Cannot change the type of a variable (previous '{:?}', new '{:?})",
                    current_var_info.var_value, &up_new_value
                ));
            }
        }

        let new_var_info = VariableInfo {
            var_value: up_new_value.clone(),
            ..current_var_info
        };
        self.symbols.insert(name, new_var_info);
        InterpreterResult::Ok(())
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

    /// Saves the AST for a function in the program context
    pub fn add_function(
        &mut self,
        function_name: String,
        function_ast: Function,
    ) -> InterpreterResult<()> {
        let name = function_name.to_ascii_lowercase();
        if self.symbols.contains_key(&name) {
            return Err(String::from("A function already exists with this name"));
        }
        self.functions.insert(name, function_ast);
        InterpreterResult::Ok(())
    }

    /// Returns the AST for the function with the given name, or None if there is no function with that name
    pub fn get_function(&mut self, function_name: String) -> Option<&Function> {
        let name = function_name.to_ascii_lowercase();
        self.functions.get(&name)
    }
}

#[derive(Debug)]
// #[repr(C)]
/// Main struct for controlling program execution
pub struct ProgHandle {
    statement_counter: Vec<(usize, bool)>,
    // bool represents if that level's statement should be skipped when returning
    ast: Program,
    pub context: ProgContext,
    global_index: usize,
}

/// Load in an ST file and set up a handle to execute it
pub fn st_program_load(filename: &str) -> InterpreterResult<ProgHandle> {
    st_program_load_actual(filename, ProgContext::new())
}

/// Load in an ST file and set up a handle to execute it, using the given context
fn st_program_load_actual(
    filename: &str,
    mut context: ProgContext,
) -> InterpreterResult<ProgHandle> {
    let file_contents = &read_file(filename)?;
    let mut function_names = HashSet::new();
    let parsed_program_ast = parser::ProgramParser::new().parse(&mut function_names, file_contents);
    let directory_path = Path::new(filename)
        .parent()
        .ok_or(format!("File path has no parent: '{}'", filename))?;
    let directory_path = directory_path.to_str().ok_or(format!(
        "Could not convert pathname '{:#?}' to string",
        directory_path
    ))?;
    let function_map = load_functions(function_names, directory_path)?;
    for function_def in function_map {
        context.add_function(function_def.0, function_def.1)?;
    }
    match parsed_program_ast {
        Ok(program_ast) => InterpreterResult::Ok(ProgHandle {
            statement_counter: vec![(0, true)],
            ast: program_ast,
            context,
            global_index: 0,
        }),
        Err(parse_error) => InterpreterResult::Err(format!("parse error: {}", parse_error)),
    }
}

// Initiate the recursive process of loading functions used by a program
fn load_functions(
    function_names: HashSet<String>,
    directory_path: &str,
) -> InterpreterResult<HashMap<String, Function>> {
    let mut function_list = HashMap::new();
    do_load_functions(function_names, directory_path, &mut function_list)?;
    InterpreterResult::Ok(function_list)
}

// Recursive work step of loading functions from a list of called function names
fn do_load_functions(
    function_names: HashSet<String>,
    directory_path: &str,
    function_list: &mut HashMap<String, Function>,
) -> InterpreterResult<()> {
    for function_name in function_names {
        // load function file
        let function_filename = String::from(directory_path)
            + &MAIN_SEPARATOR.to_string()
            + &function_name
            + ST_FILE_EXTENSION;
        let function_file_contents = &read_file(&function_filename)?;

        let mut additional_function_list: HashSet<String> = HashSet::new();
        let parsed_function_ast = parser::FunctionParser::new()
            .parse(&mut additional_function_list, function_file_contents);

        // find functions called from this one that we haven't seen before
        let mut new_functions = HashSet::new();
        for potential_new_function in additional_function_list {
            if !function_list.contains_key(&potential_new_function) {
                new_functions.insert(potential_new_function);
            }
        }
        // recurse on any newly-discovered functions
        do_load_functions(new_functions, directory_path, function_list)?;

        // insert this actual function
        let function_ast =
            parsed_function_ast.map_err(|parse_error| format!("parse error: {}", parse_error))?;
        function_list.insert(String::from(function_name), function_ast);
    }
    InterpreterResult::Ok(())
}

/// Function executes the Structured Text code using the program handle. Execution is done by calling
/// the [st_program_step] function until the function return true. When the [st_program_step] returns
/// true, the program will end.
pub fn st_program_run(program_handle: &mut ProgHandle) -> InterpreterResult<()> {
    loop {
        let ret_val = st_program_step(program_handle)?;

        if ret_val {
            break;
        }
    }

    InterpreterResult::Ok(())
}

/// Function that steps through the Structured Text program line-by-line with the program handle
/// while updating the context after each line of code. Returns the an InterpreterResult to determine
/// if the program is done executing the step function. If the InterpreterResult returns True than
/// the program has executed all statements in the list.
pub fn st_program_step(program_handle: &mut ProgHandle) -> InterpreterResult<bool> {
    //for debugging
    //println!("step: {count}", count = ProgramHandle.statement_counter);

    // get context
    let context: &mut ProgContext = &mut program_handle.context;

    //get statement counter, first postion is top level statements, last position is deepest level
    let counter_stack: &mut Vec<(usize, bool)> = &mut program_handle.statement_counter;
    let mut global_index: usize = program_handle.global_index;

    //get program node
    let program = &program_handle.ast;

    //use to get access to Vec<Assignments> as statements
    let Prog(_, all_dec_lists, statements, _) = program;

    println!("{:?}", &counter_stack);
    //check if program is complete, only have to check top level here
    if counter_stack[0].0 == statements.len() {
        return InterpreterResult::Ok(true);
    }

    //if first step then excecute declarations list
    if counter_stack[0].0 == 0 && counter_stack.len() == 1 {
        if let Some(program_dec_lists) = all_dec_lists {
            for dec_list in program_dec_lists {
                dec_list.execute(context)?;
            }
        }
    }

    //get the current statement
    let mut statement_list = statements.clone();
    let mut cur_level = 0;
    let mut statement_num = counter_stack[cur_level].0;
    let mut statement: Statement = statement_list[statement_num].clone();
    // let mut global_index = 0;

    while cur_level < counter_stack.len() {
        match statement {
            Statement::Asgn(_asgn) => {
                // Do nothing as it must be done nesting
                cur_level += 1; // must be iterated here
            }
            Statement::Iter(iter) => {
                match iter {
                    IterationStatement::While(w) => {
                        // If while loop evaluation isn't the deepest level of program counter:
                        // update statement_list so it's one level deeper

                        if cur_level != counter_stack.len() - 1 {
                            let WhileStatement::While(_w_expr, w_statment_list) = w;

                            statement_list = w_statment_list.clone();
                            cur_level += 1;

                            // Copy statement
                            statement_num = counter_stack[cur_level].0;
                        } else {
                            cur_level += 1;
                        }
                    }
                }
            }
            Statement::Select(select) => {
                match select {
                    SelectionStatement::If(i) => {
                        // If "if" loop evaluation isn't the deepest level of program counter:
                        // update statement_list so it's one level deeper

                        if cur_level != counter_stack.len() - 1 {
                            let IfStatement::If(i_vec, _i_sec_statement_list) = i;
                            let check = _i_sec_statement_list;
                            match check {
                                None => {
                                    if i_vec.len() == global_index {
                                        cur_level += 1;
                                    } else {
                                        statement_list = i_vec[global_index].1.clone();
                                        cur_level += 1;
                                    }
                                }
                                Some(x) => {
                                    if i_vec.len() == global_index {
                                        statement_list = x.clone();
                                        cur_level += 1;
                                    } else {
                                        statement_list = i_vec[global_index].1.clone();
                                        cur_level += 1;
                                    }
                                }
                            }

                            // Copy statement
                            statement_num = counter_stack[cur_level].0;
                        } else {
                            cur_level += 1;
                        }
                    }
                }
            }
        }

        statement = statement_list[statement_num].clone(); // Satisfy rust's compiler
    }
    // statement is now the current statement to execute

    //execute current statement
    match statement {
        Statement::Asgn(asgn) => {
            asgn.execute(context)?;

            // If statement is last statement of loop, go up one level in counter stack
            if statement_num == statement_list.len() - 1 {
                if counter_stack.len() > 1 {
                    // Isn't root level
                    counter_stack.pop();

                    // if skip over flag is true
                    if counter_stack.last().unwrap().1 {
                        (*counter_stack.last_mut().unwrap()).0 += 1; // Move to next statement
                    }
                    // just pop if false
                } else {
                    (*counter_stack.last_mut().unwrap()).0 += 1; // Move to next statement
                }
            } else {
                (*counter_stack.last_mut().unwrap()).0 += 1; // Move to next statement
            }
        }
        Statement::Iter(iter) => {
            match iter {
                IterationStatement::While(w) => {
                    // if statement is while it's boolean needs to be evaluated
                    // unpack while tuple into expression and statement_list
                    let WhileStatement::While(w_expr, _w_statement_list) = w;

                    let w_expr_result = w_expr.execute(context)?.unwrap();

                    match w_expr_result {
                        VariableValue::BOOL(b) => {
                            if b {
                                // set skip over flag to false as while needs to re-evaluated each loop
                                counter_stack.last_mut().unwrap().1 = false;
                                // Add new counter to counter stack to begin execution of codeblock on next step
                                counter_stack.push((0, true));
                            } else {
                                // TODO: Consolidate duplicate code
                                // If statement is last statement of loop, go up one level in counter stack

                                // Resets the recheck condition. Without this Whiles with if statements fail.
                                counter_stack.last_mut().unwrap().1 = true;

                                if statement_num == statement_list.len() - 1 {
                                    if counter_stack.len() > 1 {
                                        // Isn't root level
                                        counter_stack.pop();

                                        // if skip over flag is true
                                        if counter_stack.last().unwrap().1 {
                                            (*counter_stack.last_mut().unwrap()).0 += 1;
                                            // Move to next statement
                                        }
                                        // just pop if false
                                    } else {
                                        (*counter_stack.last_mut().unwrap()).0 += 1;
                                        // Move to next statement
                                    }
                                } else {
                                    (*counter_stack.last_mut().unwrap()).0 += 1;
                                    // Move to next statement
                                }
                            }
                        }
                        _ => {
                            return InterpreterResult::Err(String::from(
                                "While expression must resolve to boolean",
                            ));
                        }
                    }
                }
            }
        }
        Statement::Select(select) => {
            match select {
                SelectionStatement::If(i) => {
                    // if statement is while it's boolean needs to be evaluated
                    // unpack while tuple into expression and statement_list
                    let IfStatement::If(i_vec, _i_sec_statement_list) = i;
                    global_index = 0;

                    // check to see if this is the first usage of this if
                    if counter_stack.last_mut().unwrap().1 == true {
                        while i_vec.len() != global_index {
                            let i_expr_result = i_vec[global_index].0.execute(context)?.unwrap();

                            match i_expr_result {
                                VariableValue::BOOL(b) => {
                                    if b {
                                        // Don't need to re-evaluate each loop for an if
                                        // set skip over flag to false as while needs to re-evaluated each loop
                                        counter_stack.last_mut().unwrap().1 = false;
                                        // Add new counter to counter stack to begin execution of codeblock on next step
                                        counter_stack.push((0, true));
                                        break;
                                    } else {
                                        //i_vec.remove(0);
                                        global_index = global_index + 1;
                                        continue;
                                        // might have an issue not pushing to the counter stack when we have an else
                                    }
                                }
                                _ => {
                                    panic!("If expression must resolve to boolean")
                                }
                            }
                        }
                        program_handle.global_index = global_index;
                        // checking the else condition
                        let check = _i_sec_statement_list;
                        match check {
                            None => {
                                if i_vec.len() == global_index {
                                    if statement_num == statement_list.len() - 1 {
                                        if counter_stack.len() > 1 {
                                            // Isn't root level
                                            counter_stack.pop();

                                            // if skip over flag is true
                                            if counter_stack.last().unwrap().1 {
                                                (*counter_stack.last_mut().unwrap()).0 += 1;
                                                // Move to next statement
                                            }
                                            // just pop if false
                                        } else {
                                            (*counter_stack.last_mut().unwrap()).0 += 1;
                                            // Move to next statement
                                        }
                                    } else {
                                        (*counter_stack.last_mut().unwrap()).0 += 1;
                                        // Move to next statement
                                    }
                                }
                            }
                            Some(_x) => {
                                if i_vec.len() == global_index {
                                    counter_stack.push((0, true));
                                }
                            }
                        }
                    } else {
                        // move on to the next level
                        counter_stack.last_mut().unwrap().1 = true;
                        if statement_num == statement_list.len() - 1 {
                            if counter_stack.len() > 1 {
                                // Isn't root level
                                counter_stack.pop();

                                // if skip over flag is true
                                if counter_stack.last().unwrap().1 {
                                    (*counter_stack.last_mut().unwrap()).0 += 1;
                                    // Move to next statement
                                }
                                // just pop if false
                            } else {
                                (*counter_stack.last_mut().unwrap()).0 += 1;
                                // Move to next statement
                            }
                        } else {
                            (*counter_stack.last_mut().unwrap()).0 += 1;
                            // Move to next statement
                        }
                    }
                }
            }
        }
    }
    InterpreterResult::Ok(false)
}

/// Add a variable to the context of the specified [ProgHandle]
pub fn add_var(
    program_handle: &mut ProgHandle,
    name: String,
    kind: VariableKind,
    value: VariableValue,
) -> InterpreterResult<()> {
    program_handle.context.add_var(name, kind, value)
}

/// Modify a variable in the context of the specified [ProgHandle]
pub fn update_var(
    program_handle: &mut ProgHandle,
    name: &str,
    new_value: VariableValue,
) -> InterpreterResult<()> {
    program_handle.context.update_var(name, new_value)
}

/// Get information about a variable from the symbol table within the specified [ProgHandle]
pub fn get_var(program_handle: &ProgHandle, name: String) -> Option<&VariableInfo> {
    program_handle.context.get_var(name)
}

/// Gets all variables from the symbol table of the specified [ProgHandle], returning an iterator
pub fn get_all_vars(program_handle: &ProgHandle) -> Iter<'_, String, VariableInfo> {
    program_handle.context.get_all_vars()
}

pub type InterpreterResult<T> = std::result::Result<T, String>;

// struct InterpreterError(String);
