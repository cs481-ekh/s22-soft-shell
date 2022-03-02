#  Interpreter Grammar Structure
## Subset07
### Expanded grammer to be able to use comparisions in expressions and use while loops


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
				| FunctionName ‘(‘ [ST_FunctionInputs] ‘)’
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
