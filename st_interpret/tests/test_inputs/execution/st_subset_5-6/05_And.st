PROGRAM 05_And
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := TRUE and False; // Result = False
		b := false & true; // Result = False
		c := False AND FALSE; // Result = False
		d := tRuE & True; // Result = True
        ST_TESTING_RESULT := (not a & not b) & (not c & d);
END_PROGRAM