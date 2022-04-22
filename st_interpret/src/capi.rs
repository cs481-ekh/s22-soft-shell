use crate::ast::{VariableKind, VariableValue};
use crate::lib_function_example_add;
use crate::prog_handle;
use crate::prog_handle::ProgHandle;

use chrono::naive::{NaiveDate, NaiveTime};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn lib_function_example_add_clib(num_one: usize, num_two: usize) -> usize {
    lib_function_example_add(num_one, num_two)
}

#[no_mangle]
pub extern "C" fn st_program_load(filename: *const c_char) -> ProgHandlePointer {
    unsafe {
        let handle =
            prog_handle::st_program_load(CStr::from_ptr(filename).to_str().unwrap()).unwrap();
        ProgHandlePointer {
            prog_handle: Box::new(handle),
        }
    }
}

#[no_mangle]
pub extern "C" fn st_program_step(program_handle: &mut ProgHandlePointer) -> bool {
    prog_handle::st_program_step(&mut program_handle.prog_handle).unwrap()
}

#[no_mangle]
pub extern "C" fn st_program_run(program_handle: &mut ProgHandlePointer) {
    let _result = prog_handle::st_program_run(&mut program_handle.prog_handle);
}

#[no_mangle]
/// Returns a linked list of all the variables
pub extern "C" fn get_all_vars(
    program_handle: &ProgHandlePointer,
) -> Option<Box<VariableNameInfo>> {
    let vars = program_handle.prog_handle.context.get_all_vars();
    let mut current: Option<Box<VariableNameInfo>> = None;
    let mut i = 0;
    for (name, var) in vars {
        let mut last: Option<Box<VariableNameInfo>> = None;
        if i != 0 {
            last = current;
        }
        current = Some(Box::new(VariableNameInfo {
            name: CString::new(name.clone()).unwrap().into_raw(),
            value: CString::new(var.var_value.to_string()).unwrap().into_raw(),
            kind: CString::new(var.var_kind.to_string()).unwrap().into_raw(),
            next: last,
        }));
        i += 1;
    }
    current
}

#[no_mangle]
pub extern "C" fn get_var(
    program_handle: &ProgHandlePointer,
    name: *mut c_char,
) -> Option<Box<VariableNameInfo>> {
    unsafe {
        let var = program_handle
            .prog_handle
            .context
            .get_var(String::from(CStr::from_ptr(name).to_str().unwrap()))
            .unwrap();
        Some(Box::new(VariableNameInfo {
            name: name,
            value: CString::new(var.var_value.to_string()).unwrap().into_raw(),
            kind: CString::new(var.var_kind.to_string()).unwrap().into_raw(),
            next: None,
        }))
    }
}

#[no_mangle]
pub extern "C" fn update_var(
    program_handle: &mut ProgHandlePointer,
    name: *const c_char,
    value_type: *const c_char,
    value: *const c_char,
) {
    unsafe {
        let value_string = String::from(CStr::from_ptr(value).to_str().unwrap());
        let var_value = match String::from(CStr::from_ptr(value_type).to_str().unwrap()).as_str() {
            "int" => VariableValue::INT(value_string.parse::<i16>().unwrap()),
            "bool" => VariableValue::BOOL(value_string.parse::<bool>().unwrap()),
            "byte" => VariableValue::BYTE(value_string.parse::<u8>().unwrap()),
            "word" => VariableValue::WORD(value_string.parse::<u16>().unwrap()),
            "uint" => VariableValue::UINT(value_string.parse::<u16>().unwrap()),
            "dword" => VariableValue::DWORD(value_string.parse::<u32>().unwrap()),
            "dint" => VariableValue::DINT(value_string.parse::<i32>().unwrap()),
            "real" => VariableValue::REAL(value_string.parse::<f32>().unwrap()),
            "lreal" => VariableValue::LREAL(value_string.parse::<f64>().unwrap()),
            "char" => VariableValue::CHAR(value_string.parse::<u8>().unwrap()),
            "wchar" => VariableValue::WCHAR(value_string.parse::<u16>().unwrap()),
            "string" => VariableValue::STRING(value_string),
            // "time" => VariableValue::TIME(),
            // "ltime" => VariableValue::LTIME(),
            "date" => {
                VariableValue::DATE(NaiveDate::parse_from_str(value_string.as_str(), "%F").unwrap())
            }
            "time_of_day" => VariableValue::TimeOfDay(
                NaiveTime::parse_from_str(value_string.as_str(), "%T").unwrap(),
            ),
            _ => return,
        };
        let _result = program_handle
            .prog_handle
            .context
            .update_var(CStr::from_ptr(name).to_str().unwrap(), var_value);
    }
}

#[no_mangle]
pub extern "C" fn add_var(
    program_handle: &mut ProgHandlePointer,
    name: *const c_char,
    kind: *const c_char,
    value_type: *const c_char,
    value: *const c_char,
) {
    unsafe {
        let value_string = String::from(CStr::from_ptr(value).to_str().unwrap());
        let var_value = match String::from(CStr::from_ptr(value_type).to_str().unwrap()).as_str() {
            "int" => VariableValue::INT(value_string.parse::<i16>().unwrap()),
            "bool" => VariableValue::BOOL(value_string.parse::<bool>().unwrap()),
            "byte" => VariableValue::BYTE(value_string.parse::<u8>().unwrap()),
            "word" => VariableValue::WORD(value_string.parse::<u16>().unwrap()),
            "uint" => VariableValue::UINT(value_string.parse::<u16>().unwrap()),
            "dword" => VariableValue::DWORD(value_string.parse::<u32>().unwrap()),
            "dint" => VariableValue::DINT(value_string.parse::<i32>().unwrap()),
            "real" => VariableValue::REAL(value_string.parse::<f32>().unwrap()),
            "lreal" => VariableValue::LREAL(value_string.parse::<f64>().unwrap()),
            "char" => VariableValue::CHAR(value_string.parse::<u8>().unwrap()),
            "wchar" => VariableValue::WCHAR(value_string.parse::<u16>().unwrap()),
            "string" => VariableValue::STRING(value_string),
            // "time" => VariableValue::TIME(),
            // "ltime" => VariableValue::LTIME(),
            "date" => {
                VariableValue::DATE(NaiveDate::parse_from_str(value_string.as_str(), "%F").unwrap())
            }
            "time_of_day" => VariableValue::TimeOfDay(
                NaiveTime::parse_from_str(value_string.as_str(), "%T").unwrap(),
            ),
            _ => return,
        };
        let var_kind = match String::from(CStr::from_ptr(kind).to_str().unwrap()).as_str() {
            "NORMAL" => VariableKind::NORMAL,
            "INPUT" => VariableKind::INPUT,
            "OUTPUT" => VariableKind::OUTPUT,
            "IN_OUT" => VariableKind::InOut,
            "EXTERNAL" => VariableKind::EXTERNAL,
            "GLOBAL" => VariableKind::GLOBAL,
            _ => return,
        };
        let _result = program_handle.prog_handle.context.add_var(
            String::from(CStr::from_ptr(name).to_str().unwrap()),
            var_kind,
            var_value,
        );
    }
}

#[no_mangle]
pub extern "C" fn destroy_variable_name_info(var_name_info: Box<VariableNameInfo>) {
    unsafe {
        let _name = CString::from_raw(var_name_info.name);
        let _value = CString::from_raw(var_name_info.value);
        let _kind = CString::from_raw(var_name_info.kind);
    }
}

#[repr(C)]
pub struct VariableNameInfo {
    name: *mut c_char,
    value: *mut c_char,
    kind: *mut c_char,
    next: Option<Box<VariableNameInfo>>,
}

#[repr(C)]
pub struct ProgHandlePointer {
    prog_handle: Box<ProgHandle>,
}
