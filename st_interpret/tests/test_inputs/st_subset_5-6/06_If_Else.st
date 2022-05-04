//Testing simplest form of an If Else statement
PROGRAM 06_If_Else
    VAR
	a : Int;
        b : Int;
    END_VAR
	a := 0;
        b := 0;
        IF a < 5 THEN
            a :=  5;
        ELSE
            a := 3;
        END_IF;

        IF b > 5 THEN
            b :=  5;
        ELSE
            b := 3;
        END_IF;

END_PROGRAM
