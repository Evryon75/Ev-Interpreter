# Ev-Interpreter
An interpreter for the Ev programming language, my own language!
Documentation:
if:
  if condition { <br/>
    stuff<br/>
  };<br/>
  if condition {<br/>
    stuff<br/>
  } else {<br/>
    other stuff<br/>
  };<br/>
  
while:<br/>
  while condition {
    stuff
  }

variables:
  let identifier = expression;
  e.g. let x = 5;
 
functions:
  fun identifier = (parameters) {
    stuff
    return expression;
  }
  e.g. fun add = (x, y) {
          return x + y;

builtin functions:
  - random(origin, range) [generates a random number between the origin and the range]
  - scope() [Shows the variables and functions in that scope at that moment]
  - input("num" or "str") [takes input and converts it to a num or a string]
  - output(params) [prints the parameters]
  - free(param) [frees a variable from memory]
  
quirks about the language:
  - expressions are evaluated from left to right 
  - there is no operator precedence, you can use (grouping)
  - you can declare functions inside other functions and they will be in that scope
