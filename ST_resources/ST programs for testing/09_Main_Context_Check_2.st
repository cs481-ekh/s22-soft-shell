//Testing for calls that use the return values of two functions in the same statement
PROGRAM 09_Main_Context_Check_2
    VAR
	a : int;
        b : int;
        result0 : int;
    END_VAR
        a := 3;
        b := 5;
        result0 := 0;

        IF (SimpleAddFunction(a, b) = SimpleAddFunction(Input1 := 10, Input2 := 2) ) THEN
            result0 := 5;
        ELSEIF (SimpleAddFunction(a, b) = SimpleAddFunction(Input1 := 3, Input2 := 5) )THEN
            result0 := 10; //expected result
        ELSE
            result0 := 15;
        END_IF;
        
END_PROGRAM