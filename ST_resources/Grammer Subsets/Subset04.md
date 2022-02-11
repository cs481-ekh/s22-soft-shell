# Interpreter Grammar Structure
## Subset04
### Expanded grammer to handle variables assignments using +, -, *, / expressions


<pre>
Program				:'PROGRAM' ProgramName {VAR_list} StatementList 'END_PROGRAM'
</pre>


### VARS
<pre>
VAR_List			: VAR {VAR_List} 
VAR				: 'VAR' DeclarationList 'END_VAR'
								
DeclarationList			: variable ':' type ';' {DeclarationList}
</pre>	


### Expressions
<pre>
Expression			: AddExpression
AddExpression			: Term {AddOperator Term}
AddOperator			: ‘+’
				| ‘-’
Term					: PrimaryExpression {MultiplyOperator PrimaryExpression}
MultiplyOperator		: ‘*’
				| ’/’
PrimaryExpression		: constant
				| variable
				| ‘(‘ Expression ‘)’
</pre>

### Statements
<pre>
StatementList			: Statement ';' {Statement ';'}
Statement			: NIL
				| Assignment Statement
</pre>


### Assignment Statements
<pre>
AssignmentStatement		: variable ':=' Expression
</pre>

