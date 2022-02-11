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
Expression			: PrimaryExpression
PrimaryExpression		: constant
				| variable
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