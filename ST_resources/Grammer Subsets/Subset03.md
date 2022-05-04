#  Expected Interpreter Grammar Structure
## "Not case sensitive"


#### PROGRAM 
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
Term				: PrimaryExpression
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
