PROGRAM 07_While
    VAR
		a : Int;
		b : Int;
		ST_TESTING_RESULT : bool;
    END_VAR
		a := 1;
        b := 1;

        WHILE (a < 8) DO
            a := a + 1;

            WHILE (b < 3) DO
                b:= b+1;
            END_WHILE;
        END_WHILE;

        ST_TESTING_RESULT := (a = 8) & (b = 3);
END_PROGRAM