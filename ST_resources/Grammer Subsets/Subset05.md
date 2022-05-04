#  Interpreter Grammar Structure
## Subset05
### Expanded grammer to use AND and OR in expressions


#### PROGRAM 
<pre>
Program				:'PROGRAM' ProgramName {VAR_list} StatementList 'END_PROGRAM'
</pre>



### VARS
<pre>
VAR_List			: VAR {VAR_List} 
VAR				: 'VAR'DeclarationList 'END_VAR'
								
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
Term				: PrimaryExpression {MultiplyOperator PrimaryExpression}
MultiplyOperator		: ‘*’
				| ’/’
				| ‘MOD’
PrimaryExpression		: constant
				| variable
				| ‘(‘ Expression ‘)’
</pre>

### Statements
<pre>
StatementList			: Statement ';' {StatementList}
				| NIL
Statement			: NIL
				| Assignment Statement
</pre>


### Assignment Statements
<pre>
AssignmentStatement		: variable ':=' Expression
</pre>

