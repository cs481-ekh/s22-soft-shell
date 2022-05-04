//Testing counter, increments input by one by returning value from Output variable
// 
PROGRAM 09_Main_Counter_InOut
    VAR
	a : int;
        b : int;
        result0 : int;
        result1 : int;

    END_VAR
        a := 4;
        b := 2;

        // IN_OUT variable get copied into function and copied out changing the original variable when done
        result0 := CounterInOutFunction(InOut1 := a); // expects result of true and a as 5

        result1 := CounterInOutFunction(InOut1 := b); // expects result of true and b as 3

        
END_PROGRAM
