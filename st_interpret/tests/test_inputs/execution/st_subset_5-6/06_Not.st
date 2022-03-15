PROGRAM 05_Not
    VAR
		a : bool;
		b : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := not false; // Result = True
		b := NOT true; // Result = False
        ST_TESTING_RESULT := (a & not b);
END_PROGRAM