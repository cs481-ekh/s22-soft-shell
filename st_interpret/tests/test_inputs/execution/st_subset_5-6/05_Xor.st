PROGRAM 05_Xor
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
        ST_TESTING_RESULT : bool;
    END_VAR
		a := TRUE XOR False; // Result = True
		b := false xOR true; // Result = True
		c := False xOr FALSE; // Result = False
		d := tRuE xor True; // Result = False
        ST_TESTING_RESULT := (a & b) & (not c & not d);
END_PROGRAM