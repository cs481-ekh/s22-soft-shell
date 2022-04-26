use st_interpret::prog_handle::{
    st_program_load, st_program_run, st_program_step, InterpreterResult, ProgContext,
};
use st_interpret::VariableValue;

fn main() -> InterpreterResult<()> {
    println!("Example 1 - Execute a program");
    let mut context = ProgContext::new();
    let mut prog1 = st_program_load("st_testfiles/Calc_Test.st", context)?;

    st_program_run(&mut prog1)?;
    println!("d: {:?}", prog1.context.get_var("d".to_string()).unwrap());

    println!("\nExample 2 - Step through a program");
    let mut context2 = ProgContext::new();
    let mut prog2 = st_program_load("st_testfiles/Times2.st", context2)?;
    let mut is_finished = false;

    while !is_finished {
        is_finished = st_program_step(&mut prog2)?;
        println!("{:?}", prog2.context.get_all_vars());
    }

    println!("\nExample 3 - Modify a program mid execution");
    let mut context3 = ProgContext::new();
    let mut prog3 = st_program_load("st_testfiles/Times2.st", context3)?;

    st_program_step(&mut prog3)?;
    st_program_step(&mut prog3)?;

    prog3.context.update_var("b", VariableValue::INT(6))?;

    st_program_step(&mut prog3)?;
    println!("{:?}", prog3.context.get_all_vars());

    InterpreterResult::Ok(())
}
