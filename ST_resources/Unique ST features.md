#  Unique Structured Text features


### not case sensitive

#### Comments
Structured text has two types of comments
<pre>
line comment 	// commented text

block comment 	(* commented text *)
</pre>

### Program module and function module are separated
<pre>
programs are split up as follows

-program modules

-Functions

-Function blocks
</pre> 
#### Program module
A PLC program must be split up into many small program pieces in order to have a good and clear program structure. The small program pieces are called program modules, functions and function blocks, and they each contain a small piece of PLC code and is a building block, to be used or reused whenever needed.
To obtain a good structure, it is a good rule of thumb to only have only 20 - 25 lines of PLC code in each program module, function or function block.

The difference between functions and program modules is that functions often perform calculations or data processing on individual components, whilst program modules is the splitting up of the entire program. The program modules use relevant functions and function blocks to solve the specific tasks.



#### Functions
Functions are important building blocks in a PLC program. A function contains a limited number of code lines to be used (‘called’ and executed) again and again.

The advantage of using functions is that the PLC code can be reused. PLC code reuse reduces the size of the program, creates fewer syntax faults and is easier to work with for other programmers.

There are two function types in a PLC:

<pre>
	-Function (FC)
	-Function block (FB)
</pre>

Function (FC) PLC code excludes static data, which means that all local variables lose their value when the function ends. The variables are initialized again the next time the function is ‘called’. The function typically carries out a mathematical calculation and returns the calculated value.

Function block (FB) PLC code which includes static data. The local variables retain their values between each ‘call’ to the function. An example could be a function used as an hour counter (number of operation hours, also called TACHO HOURS) on a motor which requires that the local variables retain their values once the function has ended. The function could also count the number of motor starts per hour or time until the next motor service.

Template
<pre>
FUNCTION &ltName&gt : &ltRetDataType&gt
VAR_INPUT
   &ltVariables&gt
END_VAR
VAR_OUTPUT
   &ltVariables&gt
END_VAR
VAR_IN_OUT
   &ltVariables&gt
END_VAR
VAR
   &ltVariable&gt //local variables
END_VAR
   &ltImplementation&gt //write code here

   &ltName&gt := 123; //set return value
END_FUNCTION
</pre>



