PROGRAM 07_While
    VAR
		a : Int;
		b : Int;
    END_VAR
		a := 1;
        b := 2;

        WHILE (a < 8) DO
            a := a + 1;
            a := a + 1;

            WHILE (b < 3) DO
                b := b + 1;
            END_WHILE;
        END_WHILE;
END_PROGRAM