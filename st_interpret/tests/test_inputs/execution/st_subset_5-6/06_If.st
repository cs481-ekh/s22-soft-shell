//Testing simplest form of an If statement
PROGRAM 06_If
    VAR
	a : Int;
	ST_TESTING_RESULT : bool;
    END_VAR
	a := 0;
        IF a < 5 THEN
            a :=  5;
        END_IF;
        ST_TESTING_RESULT := (a = 5);
END_PROGRAM
