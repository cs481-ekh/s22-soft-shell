PROGRAM 01_truncate //should not error but rather truncate 1.5 to 1
    VAR
	a : INT;
        ST_TESTING_RESULT : bool;
    END_VAR
	a := 1.5;
        ST_TESTING_RESULT := (a = 1);
END_PROGRAM
