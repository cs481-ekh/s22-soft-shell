// Tests that while and IF statements work inside eachother
PROGRAM 07_While_If
    VAR
	a : Int;
        b : Int;
        c : Int;
        d : Int;
    END_VAR
	a := 0;
        b := 0;
        c := 0;
        d := 0;
        WHILE (a < 5) DO
            a := a + 1;
            IF a > 3 THEN
                b := 3;
            END_IF;
        END_WHILE;

        IF (c < 5) THEN
            WHILE (d < 4) DO
                d := d + 1;
            END_WHILE;
            c := 2;
        END_IF;

END_PROGRAM
