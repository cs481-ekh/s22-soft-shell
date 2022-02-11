#  Interpreter Grammar Structure
## Subset01
### expected to be able to hand simple variables assignments of constants


#### PROGRAM 
<pre>
Program				:'PROGRAM' ProgramName {VAR_list} StatementList 'END_PROGRAM'
</pre>


### VARS
<pre>
VAR_List			: VAR {VAR_List} 
VAR				: 'VAR' DeclarationList 'END_VAR'
								
DeclarationList			: variable ':' type ';'
</pre>	



### Statements
<pre>
StatementList			: Statement ';' {Statement ';'}
Statement			: NIL
				| Assignment Statement

</pre>


### Assignment Statements
<pre>
AssignmentStatement		: variable ':=' constant
</pre>

