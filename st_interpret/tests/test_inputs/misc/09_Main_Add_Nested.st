// Function call to add two numbers together, which itself contains another function call
PROGRAM 09_Main_Add
    VAR
	a : int;
        b : int;
        result0 : int;
        result1 : int;
        result2 : int;
        result3 : int;
    END_VAR
        a := 3;
        b := 5;

        // four different ways of function input
        result0 := NestedAddFunction(10, 10+5); // expects outputs of 25

        result1 := NestedAddFunction(Input1 := 5, Input2 := 10); // expects outputs of 15

        result2 := NestedAddFunction(a, b); // expects outputs of 8

        result3 := NestedAddFunction(Input1 := a, Input2 := b); // expects outputs of 8
        
END_PROGRAM
