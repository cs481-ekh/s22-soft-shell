//Testing for INPUT, OUTPUT, IN_OUT type variables in same function call
PROGRAM 09_Main_ThreeVar
    VAR
	a : int;
        b : int;
        result0 : int;
        ST_TESTING_RESULT : bool;
    END_VAR
        a := 0;
        b := 0;

        // Output variables in a function call use => instead of := because Output variable is not being
        // assinged an initial value but rather it will only be outputing a value to that variable.
        // expects result of true, a as 5 and b as 2
        result0 := CounterOutputFunction( Input1 := 4, Output1 => a, InOut1 := b); 

        ST_TESTING_RESULT := (result0) & (a = 5 & b = 2);
        
END_PROGRAM
