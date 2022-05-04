PROGRAM 07_Equality
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := TRUE = False; // Result = False
		b := false <> true; // Result = True
		c := False = FALSE; // Result = True
		d := tRuE <> True; // Result = False
        ST_TESTING_RESULT := (not a & b) & (c & not d);
END_PROGRAM