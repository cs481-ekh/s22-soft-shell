mod prog_handle {
    use std::collections::HashMap;

    enum VariableValue {
        INT(i16),
        BOOL(bool),
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
