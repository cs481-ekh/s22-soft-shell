PROGRAM 01_invalid_bool // bool should accept 0 and 1 but not any other ints
    VAR
	a : bool;
        b : bool;
        c : bool;
    END_VAR
	a := 1; //true
        a := 0; //false
        a := 5; //error!
END_PROGRAM
