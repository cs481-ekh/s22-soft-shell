PROGRAM 01_real //boolean accepts integers 0 and 1
    VAR
	a : bool;
        b : bool;
        c : bool;
        d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
	a := TRUE;
        b := FALSE;
        c := 1; //true
        d := 0; //false

        ST_TESTING_RESULT := (a & not b) & (c & not d);
END_PROGRAM
