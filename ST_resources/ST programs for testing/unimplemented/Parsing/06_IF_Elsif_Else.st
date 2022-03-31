//Testing simplest form of an IF ELSIF ELSE statement
PROGRAM 07_IF_Elsif_Else
    VAR
	a : Int;
        b : Int;
        c : Int;
    END_VAR
	a := 0;
        b := 0;
        c := 0;
        IF a < 5 THEN
            a :=  5; //correct result
        ELSIF a < 4 THEN
            a := 4;
        ELSE
            a := 3;
        END_IF;

        IF b > 5 THEN
            b :=  5;
        ELSIF b < 4 THEN
            b := 4; //correct result
        ELSE
            b := 3;
        END_IF;

        IF c > 5 THEN
            c :=  5; 
        ELSIF c > 4 THEN
            c := 4; 
        ELSE
            c := 3; //correct result
        END_IF;

END_PROGRAM
