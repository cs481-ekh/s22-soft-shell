PROGRAM 07_While
    VAR
	a : Int;
	ST_TESTING_RESULT : bool;
    END_VAR
	a := 0;
        WHILE (a < 5) DO
            a := a + 1;
        END_WHILE;
        ST_TESTING_RESULT := (a = 5);
END_PROGRAM
