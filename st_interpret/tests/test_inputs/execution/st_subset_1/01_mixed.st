PROGRAM 01_real //accepts all types in same program
    VAR
	a : real;
        b : int;
        c : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
	a := 1.2;
        b := 5;
        c := True;
        ST_TESTING_RESULT := (a = 1.2) & ((b = 5) & c);
END_PROGRAM
