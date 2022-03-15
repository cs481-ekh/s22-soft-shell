PROGRAM 01_NestedReal
    VAR
	a : real;
        b : real;
        c : real;
        ST_TESTING_RESULT : bool;
    END_VAR
	a := 1.5;
        b := a;
        c := b;
        ST_TESTING_RESULT := ((a = 1.5) & (b = 1.5)) & (c = 1.5);
END_PROGRAM
