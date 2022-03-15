PROGRAM 07_Greater_Eq
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := 8 >= -5; // Result = True
		b := -4 >= -4; // Result = True
		c := 0 >= 5555; // Result = False
		d := -34 >= -55; // Result = True
        ST_TESTING_RESULT := (a & b) & (not c & d);
END_PROGRAM