// Rust integration tests here

use st_interpret::prog_handle::{add_var, get_var, st_program_load, st_program_run, update_var};
use st_interpret::{
    interpreter_batch_test_st_folder, parser_batch_test_st_folder, parser_test_st_function,
    read_file, VariableKind, VariableValue,
};

/// Test lalrpop functionality
#[test]
fn example_parser() {
    assert!(st_interpret::parser_test());
}

#[test]
/// Test the ability to read in a file
fn test_open_file() {
    let a = read_file("tests/test_inputs/st_subset_1/01_Bool.st").unwrap();
    assert_eq!(a.is_empty(), false);
}

#[test]
/// Test loading in a valid program
fn test_load_program() {
    st_program_load("tests/test_inputs/st_subset_1/01_Bool.st").unwrap();
}

#[test]
/// Test loading in a valid program with functions
fn test_load_program_with_functions() {
    let mut handle = st_program_load("tests/test_inputs/misc/09_Main_Add.st").unwrap();
    handle
        .context
        .get_function(String::from("SimpleAddFunction"))
        .unwrap();
}

#[test]
/// Test loading in a valid program with functions that call other functions
fn test_load_program_with_nested_functions() {
    let mut handle = st_program_load("tests/test_inputs/misc/09_Main_Add_Nested.st").unwrap();
    handle
        .context
        .get_function(String::from("NestedAddFunction"))
        .unwrap();
    handle
        .context
        .get_function(String::from("IntIdentityFunction"))
        .unwrap();
}

#[test]
#[should_panic(expected = "Unrecognized token `Hello`")]
/// Test that loading an invalid program fails
fn test_load_invalid_program() {
    st_program_load("tests/test_inputs/misc/read_test.txt").unwrap();
}

#[test]
/// Example test that does addition
fn it_adds_two_integration() {
    assert_eq!(4, st_interpret::lib_function_example_add(2, 2));
}

#[test]
/// Example test that does addition
fn it_adds_15_integration() {
    assert_eq!(17, st_interpret::lib_function_example_add(15, 2));
}

#[test]
/// Check parser succeeds over subset 1 ST programs.
fn subset1_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_subset_1");
}

#[test]
/// Check parser succeeds over subset 3 and 4 ST programs.
fn subset3_4_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_subset_3-4");
}

#[test]
/// Check parser succeeds over subset 5 and 6 ST programs.
fn subset5_6_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_subset_5-6");
}

#[test]
/// Check parser succeeds over subset 7 ST programs.
fn subset7_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_subset_7");
}

#[test]
/// Check parser succeeds over subset 8 and 9 ST programs.
fn subset8_9_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_subset_8-9");
}

#[test]
/// Check parser succeeds over subset 9 ST functions
fn subset9_func_parse() {
    parser_test_st_function("tests/test_inputs/st_subset_9_funcs/SimpleAddFunction.st");
    parser_test_st_function("tests/test_inputs/st_subset_9_funcs/ValueAverageFunction.st");
}

#[test]
/// Check parser succeeds over parts of the ST language not currently included in a target subset.
fn stretch_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_stretch");
}

#[test]
#[should_panic(expected = "assertion failed: parse_result.is_ok()")]
/// Check parser correctly finds incorrect st files
fn fail_parse() {
    parser_batch_test_st_folder("tests/test_inputs/st_failure/parse_fail");
}

#[test]
/// Check subset 1 programs execute correctly
fn execute_subset_1() {
    interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_1");
}

#[test]
/// Check subset 3-4 programs execute correctly
fn execute_subset_3_4() {
    interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_3-4");
}

#[test]
/// Check subset 5-6 programs execute correctly
fn execute_subset_5_6() {
    interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_5-6");
}

#[test]
/// Check subset 7 programs execute correctly
fn execute_subset_7() {
    interpreter_batch_test_st_folder("tests/test_inputs/execution/st_subset_7");
}

#[test]
/// Check stretch (no assigned subset) programs execute correctly
fn execute_stretch() {
    interpreter_batch_test_st_folder("tests/test_inputs/execution/st_stretch");
}

#[test]
/// Example of a basic interpretation test
fn execute_basic() {
    let mut prog_handle = st_program_load("tests/test_inputs/st_subset_1/01_mixed.st").unwrap();
    st_program_run(&mut prog_handle).unwrap();
    assert_eq!(
        get_var(&prog_handle, String::from("a")).unwrap().var_value,
        VariableValue::REAL(1.2)
    );
    assert_eq!(
        get_var(&prog_handle, String::from("b")).unwrap().var_value,
        VariableValue::INT(5)
    );
    assert_eq!(
        get_var(&prog_handle, String::from("c")).unwrap().var_value,
        VariableValue::BOOL(true)
    );
}

#[test]
/// Test using context-related API wrapper functions on a program
fn test_context_api_wrappers() {
    let mut prog_handle = st_program_load("tests/test_inputs/st_subset_1/01_Int.st").unwrap();
    add_var(
        &mut prog_handle,
        String::from("testvar"),
        VariableKind::NORMAL,
        VariableValue::INT(5),
    )
    .unwrap();
    let result = get_var(&mut prog_handle, String::from("testvar")).unwrap();
    assert_eq!(VariableValue::INT(5), result.var_value);
    assert_eq!(VariableKind::NORMAL, result.var_kind);
    update_var(&mut prog_handle, "testvar", VariableValue::INT(4)).unwrap();
    let result = get_var(&mut prog_handle, String::from("testvar")).unwrap();
    assert_eq!(VariableValue::INT(4), result.var_value);
    assert_eq!(VariableKind::NORMAL, result.var_kind);
}
