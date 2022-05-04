//Test used to confirm nested while loops
PROGRAM 07_While_Nested
    VAR
	a : Int;
        b : Int;
        c : Int;
    END_VAR
	a := 0;
        b := 0;
        c := 0;
        WHILE (a < 5) DO
            a := a + 1;
            WHILE (b < 4) DO
                b := b + 1;
                WHILE (c < 3) DO
                    c := c + 1;
                END_WHILE;
            END_WHILE;
        END_WHILE;
END_PROGRAM
