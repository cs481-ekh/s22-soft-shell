//Testing nested form of an If statement
PROGRAM 06_If_Nested
    VAR
	a : Int;
        b : Int;
        c : Int;
        ST_TESTING_RESULT : bool;
    END_VAR
	a := 0;
        b := 0;
        c := 0;
        IF a < 5 THEN
            a :=  5;
            IF b < 5 THEN
                b :=  4;
                IF c < 5 THEN
                    c :=  3;
                END_IF;
            END_IF;
            IF c < 1 THEN
                c :=  5;// should not be reached
            END_IF;
        END_IF;
        ST_TESTING_RESULT := (a = 5 & b = 4) & (c = 3) ;
END_PROGRAM
