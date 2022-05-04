FUNCTION ValueAverageFunction : REAL //REAL is the return parameter data type
VAR_INPUT
  Value1 :	REAL; //Input parameter 1 to the function
  Value2 :	REAL; //Input parameter 2 to the function
END_VAR	

VAR	
  Sum :	REAL; //Local variable for temporary calculation
END_VAR

  Sum:= Value1 + Value2; //Total sum
  Sum:= Sum/2; //Average

  ValueAverageFunction := Sum; //Set the return parameter

END_FUNCTION
