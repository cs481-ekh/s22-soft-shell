PROGRAM 01_NestedBool
    VAR
	a : bool;
        b : bool;
        c : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
	a := true;
        b := a;
        c := b;
        ST_TESTING_RESULT := a & (b & c);
END_PROGRAM
