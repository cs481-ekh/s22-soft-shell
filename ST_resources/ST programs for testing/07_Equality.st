PROGRAM 07_Equality
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
    END_VAR
		a := TRUE = False; // Result = False
		b := false <> true; // Result = True
		c := False = FALSE; // Result = True
		d := tRuE <> True; // Result = False
END_PROGRAM