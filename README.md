# Ev-Interpreter
An interpreter for the Ev programming language, my own language!<br/>
Documentation:<br/>
if:<br/>
--if condition {<br/>
----stuff<br/>
--};<br/>
--if condition {<br/>
----stuff<br/>
--} else {<br/>
----other stuff<br/>
--};<br/>
  
while:<br/>
--while condition {<br/>
----stuff<br/>
--}<br/>

variables:<br/>
--let identifier = expression;<br/>
--e.g. let x = 5;<br/>
 
functions:<br/>
--fun identifier = (parameters) {<br/>
----stuff<br/>
----return expression;<br/>
--};<br/>
--e.g. fun add = (x, y) {<br/>
---------return x + y;<br/>
-------};

builtin functions:
  - random(origin, range) [generates a random number between the origin and the range]
  - scope() [Shows the variables and functions in that scope at that moment]
  - input("num" or "str") [takes input and converts it to a num or a string]
  - output(params) [prints the parameters]
  - free(param) [frees a variable or function from memory, useful for recursion]
  - abort(message) [stops the program and displays a message]
  
quirks about the language:
  - expressions are evaluated from right to left 
  - there is no operator precedence, you can use (grouping)
  - you can declare functions inside other functions and they will be in that scope
