PROGRAM 07_Equality
    VAR
		a : bool;
		b : bool;
		c : bool;
		d : bool;
    END_VAR
		a := TRUE = False; // Result = False
		b := false <> true; // Result = False
		c := False = FALSE; // Result = True
		d := tRuE <> True; // Result = True
END_PROGRAM