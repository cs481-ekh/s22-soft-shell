PROGRAM 07_Test_A
    VAR
		a : Int;
		b : Int;
		c : bool;
		d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := 3;
		b := 6;
		c := 3 >= a = 6-20 < 26; // Result = True
		d := (16/4) = (b-4) * 2; // Result = True
        ST_TESTING_RESULT := (a = 3 & b = 6) & (c & d);
END_PROGRAM