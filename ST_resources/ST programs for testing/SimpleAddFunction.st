FUNCTION SimpleAddFunction : INT //INT is the return parameter data type
VAR_INPUT
  Input1 :	INT; //Input parameter 1 to the function
  Input2 :	INT; //Input parameter 2 to the function
END_VAR	

VAR	
  Sum :	INT; //Local variable for temporary calculation
END_VAR

  Sum:= Input1 + Input2; //Total sum

  SimpleAddFunction:= Sum; //Set the return parameter

END_FUNCTION
