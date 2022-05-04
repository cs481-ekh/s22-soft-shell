//Testing simplest form of an IF ELSIF ELSE statement
PROGRAM 06_IF_Elsif_Else
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
            a :=  5; //correct result
        ELSEIF a < 4 THEN
            a := 4;
        ELSE
            a := 3;
        END_IF;

        IF b > 5 THEN
            b :=  5;
        ELSEIF b < 4 THEN
            b := 4; //correct result
        ELSE
            b := 3;
        END_IF;

        IF c > 5 THEN
            c :=  5; 
        ELSEIF c > 4 THEN
            c := 4; 
        ELSE
            c := 3; //correct result
        END_IF;

        ST_TESTING_RESULT := (a = 5 & b =4) & (c = 3);
END_PROGRAM
