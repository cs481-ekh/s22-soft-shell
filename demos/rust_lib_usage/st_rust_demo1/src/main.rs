use st_interpret::{
    get_all_vars, get_var, st_program_load, st_program_run, st_program_step, update_var,
    InterpreterResult, VariableValue,
};

fn main() -> InterpreterResult<()> {
    println!("Example 1 - Execute a program");
    let mut prog1 = st_program_load("st_testfiles/Calc_Test.st")?;

    st_program_run(&mut prog1)?;
    println!("d: {:?}", get_var(&prog1, "d".to_string()).unwrap());

    println!("\nExample 2 - Step through a program");
    let mut prog2 = st_program_load("st_testfiles/Times2.st")?;
    let mut is_finished = false;

    while !is_finished {
        is_finished = st_program_step(&mut prog2)?;
        println!("{:?}", get_all_vars(&prog2));
    }

    println!("\nExample 3 - Modify a program mid execution");
    let mut prog3 = st_program_load("st_testfiles/Times2.st")?;

    st_program_step(&mut prog3)?;
    st_program_step(&mut prog3)?;

    update_var(&mut prog3, "b", VariableValue::INT(6))?;

    st_program_step(&mut prog3)?;
    println!("{:?}", get_all_vars(&prog3));

    InterpreterResult::Ok(())
}
