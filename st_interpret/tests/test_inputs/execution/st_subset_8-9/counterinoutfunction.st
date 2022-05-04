// This function is designed to test IN_OUT variable. 
// Takes in_out integer and adds 1 to it "counting up by 1"
// Then returns new count  with variable instead of function name
// function name returns true to indicate success
FUNCTION CounterInOutFunction : BOOL //BOOL is the return parameter data type
VAR_IN_OUT
  InOut1 :	INT; //IN_OUT parameter
END_VAR

  InOut1 := InOut1 + 1;

  CounterFunction:= 'TRUE'; //Set the return parameter

END_FUNCTION
