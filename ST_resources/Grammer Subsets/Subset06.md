#  Interpreter Grammar Structure
## Subset06
### Expanded grammer to use negative values, NOT, and If statements

#### PROGRAM 
<pre>
Program				:'PROGRAM' ProgramName {VAR_list} StatementList 'END_PROGRAM'
</pre>


### VARS
<pre>
VAR_List			: VAR {VAR_List} 
VAR				: 'VAR'{RETAIN} DeclarationList 'END_VAR'
							
DeclarationList			: variable ':' type ';' {DeclarationList}
</pre>	



### Expressions
<pre>
Expression			: XOR_Expression {'OR' XOR_Expression}
XOR_Expression			: AND_Expression {‘XOR’ AND_Expression}
AND_Expression			: AddExpression { (‘&’ | ‘AND’) AddExpression}
AddExpression			: Term {AddOperator Term}
AddOperator			: ‘+’
				| ‘-’
Term				: UnaryExpression {MultiplyOperator UnaryExpression}
MultiplyOperator		: ‘*’
				| ’/’
				| ‘MOD’
UnaryExpression			: [UnaryOperator] PrimaryExpression
UnaryOperator			: ‘-‘
				| ‘NOT’
PrimaryExpression		: constant
				| variable
				| ‘(‘ Expression ‘)’
				| FunctionName ‘(‘ [ST_FunctionInputs] ‘)’
ST_FunctionInputs		: ST_FunctionInput { ‘,’ ST_FunctionInput}
ST_FunctionInput		: [VariableName ‘:=’] Expression
</pre>

### Statements
<pre>
StatementList			: Statement ';' {Statement ';'}
Statement			: NIL
				| Assignment Statement
				| SelectionStatement
</pre>


### Assignment Statements
<pre>
AssignmentStatement		: variable ':=' Expression
</pre>


### Selection statements
<pre>
SelectionStatement		: IfStatement
IfStatement			: ‘IF’ Expression ‘THEN’ StatementList 
				   { ‘ElSIF’ Expression ‘THEN' StatementList } 
				   [‘Else’ StatementList] ‘END_IF’
				   
</pre>
