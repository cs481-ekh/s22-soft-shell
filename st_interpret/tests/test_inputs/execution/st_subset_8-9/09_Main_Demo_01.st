// Testing for GO West Example use case example
// program doesnt go into while loop without C API
// is used to set INPUT1 or INPUT2 to TRUE
// x represents number of loops INPUT1 and INPUT2 are both true during
// OUTPUT1 represents number of loops
PROGRAM 09_Main_Demo_01
    VAR
	x : int;
        INPUT1 : bool;
        INPUT2 : bool;
        OUTPUT1 : int;
        ST_TESTING_RESULT : bool;
    END_VAR
        x := 0;
        INPUT1 := False;
        INPUT2 := False ;
        OUTPUT1 := 0 ;
//API would control INPUTs after here HERE here

        WHILE (INPUT1) OR (INPUT2) DO
            If (INPUT1) AND (INPUT2) THEN
                x:= x +1;
            END_IF;
            OUTPUT1 := SimpleAddFunction(Input1 := OUTPUT1, Input2 := 1);
        END_WHILE;
        ST_TESTING_RESULT := (x = 0 );
END_PROGRAM
