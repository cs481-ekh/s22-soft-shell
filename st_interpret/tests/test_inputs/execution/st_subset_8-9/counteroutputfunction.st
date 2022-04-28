// This function is designed to test Output variable. 
// Takes input integer and adds 1 to it "counting up by 1"
// Then returns new count using Output variable instead of function name
// function name returns true to indicate success
FUNCTION CounterOutputFunction : BOOL //BOOL is the return parameter data type
VAR_INPUT
  Input1 :	INT; //Input parameter
END_VAR	
VAR_OUTPUT
  Output1 :	INT; //Output parameter
END_VAR

  Output1 := Input1 + 1;

  CounterFunction:= 'TRUE'; //Set the return parameter

END_FUNCTION
