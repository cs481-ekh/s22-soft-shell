PROGRAM 01_NestedInt
    VAR
	a : INT;
        b : INT;
        c : INT;
        ST_TESTING_RESULT : bool;
    END_VAR
 	a := 1;
        b := a;
        c := b;
        ST_TESTING_RESULT := ((a = 1) & (b = 1)) & (c = 1);
END_PROGRAM
