//Testing simple function call to get average of two numbers
PROGRAM 09_Main_Avg
    VAR
	a : real;
        b : real;
        avg0 : real;
        avg1 : real;
        avg2 : real;
        avg3 : real;
        ST_TESTING_RESULT : bool;
    END_VAR
        a := 4;
        b := 6;

        // four different ways of function input
        avg1 := ValueAverageFunction(10, 10+5); // expects 12.5

        avg2 := ValueAverageFunction(Value1 := 5, Value2 := 10); // expects 7.5

        avg3 := ValueAverageFunction(a, b); // expects 5

        avg4 := ValueAverageFunction(Value1 := a, Value2 := b); // expects 5

        ST_TESTING_RESULT := (avg1 = 12.5 & avg2 = 7.5) & (avg3 = 5 & avg4 = 5);
        
END_PROGRAM
