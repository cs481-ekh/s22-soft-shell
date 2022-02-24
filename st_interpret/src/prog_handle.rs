mod prog_handle {
    use chrono::naive::{NaiveDate, NaiveTime};
    use std::collections::HashMap;
    use std::time::Duration;

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

    enum VariableKind {
        INPUT,
        OUTPUT,
        NORMAL,
    }

    struct VariableInfo {
        var_value: VariableValue,
        var_kind: VariableKind,
    }

    struct ProgContext {
        symbols: HashMap<String, VariableInfo>,
    }
}
