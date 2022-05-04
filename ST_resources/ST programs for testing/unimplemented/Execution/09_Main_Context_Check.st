//Testing to insure seperation of context, Input2 and Sum variables being used in main and add function 
PROGRAM 09_Main_Context_Check
    VAR
	a : int;
        b : int;
        Input2 : int
        Sum : INT;
        result0 : int;
        result1 : int;
        ST_TESTING_RESULT : bool;
    END_VAR
        a := 3;
        b := 5;
        Input2 := 10;
        Sum := 2;

        result0 := SimpleAddFunction(a, b); // expects outputs of 8

        result1 := SimpleAddFunction(Input1 := a, Input2 := b); // expects outputs of 8

        ST_TESTING_RESULT := (result0 = 8 & result1 = 8) & (Input2 = 10 & Sum = 2);
        
END_PROGRAM