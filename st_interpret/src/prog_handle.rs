/// Program state and current execution information.
use crate::ast::Program;
use crate::read_file;
lalrpop_mod!(pub parser);
use chrono::naive::{NaiveDate, NaiveTime};
use std::collections::HashMap;
use std::time::Duration;

/// ST variable types, each holding a corresponding value of that type.
enum VariableValue {
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

/// Different 'kinds' of ST variables, such as input, output, etc.
enum VariableKind {
    INPUT,
    OUTPUT,
    NORMAL,
}

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
