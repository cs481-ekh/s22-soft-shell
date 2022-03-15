PROGRAM 07_Less_Eq
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := 8 <= -5; // Result = False
		b := 44 <= 44; // Result = True
		c := 0 <= 5555; // Result = True
		d := -34 <= -55; // Result = False
        ST_TESTING_RESULT := (not a & b) & (c & not d);
END_PROGRAM