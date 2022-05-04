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
				| 'VAR_INPUT' DeclarationList 'END_VAR'
				| 'VAR_OUTPUT' DeclarationList 'END_VAR'
				| 'VAR_IN_OUT' DeclarationList 'END_VAR'
				| 'VAR_EXTERNAL' DeclarationList 'END_VAR'
				| 'VAR_GLOBAL' DeclarationList 'END_VAR'
				
DeclarationList			: variable ':' type ';' {DeclarationList}
</pre>	



### Expressions
<pre>
Expression			: XOR_Expression {'OR' XOR_Expression}
XOR_Expression			: AND_Expression {‘XOR’ AND_Expression}
AND_Expression			: Comparison { (‘&’ | ‘AND’) Comparison}
Comparision			: EquExpression { ( ‘=’ | ‘<>’) EquExpression}
EquExpression			: AddExpression {ComparisonOperator AddExpression}
ComparisionOperator		: ‘<’
				| ‘>’
				| ‘<=’
				| ‘>=’
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
</pre>



### Statements
<pre>
StatementList			: Statement ';' {StatementList}
				| NIL
Statement			: NIL
				| Assignment Statement
				| SelectionStatement
				| IterationStatement
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



### Iteration Statements
<pre>
IterationStatement		: WhileStatement

WhileStatement			: ‘WHILE’ Expression ‘DO’
				   StatementList
				   ‘END_WHILE’

</pre>
