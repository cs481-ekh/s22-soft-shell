/// Program state and current execution information.

mod prog_handle {
    use crate::ast::Program;
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
    struct ProgContext {
        symbols: HashMap<String, VariableInfo>,
    }

    struct ProgHandle {
        statement_counter: u32,
        ast: Program,
        context: ProgContext,
    }
}
