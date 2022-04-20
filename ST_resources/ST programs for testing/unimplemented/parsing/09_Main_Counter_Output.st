//Testing counter, increments input by one by returning value from Output variable
PROGRAM 09_Main_Counter_Output
    VAR
	a : int;
        b : int;
        result0 : int;
        result1 : int;

    END_VAR
        a := 0;
        b := 0;

        // Output variables in a function call use => instead of := because Output variable is not being
        // assinged an initial value but rather it will only be outputing a value to that variable.
        result0 := CounterOutputFunction( Input1 := 4, Output1 => a); // expects result of true and a as 5

        result1 := CounterOutputFunction(Input1 := 2, Output1 => b); // expects result of true and b as 3
        
END_PROGRAM
