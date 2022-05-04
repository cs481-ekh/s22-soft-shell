
(* Here is a simple test program taht assigns a with a real number and then tries 
to assign that value to b which is an INT, this should truncate and leave b equal to 1 *)

PROGRAM 02_testA
    VAR //variable declarations
	a : real;
      	b : int;
    END_VAR
	a := 1.2;
    	b := a; //attempting to assign a real to an int
END_PROGRAM
