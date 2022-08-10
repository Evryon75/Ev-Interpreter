# Ev-Interpreter
An interpreter for the Ev programming language, my own language!<br/>
Documentation:<br/>
> If statement<br/>
<pre>
if condition {<br/>
    stuff<br/>
};<br/>
if condition {<br/>
    stuff<br/>
} else {<br/>
    other stuff<br/>
};<br/>
</pre>

> While statement<br/>
<pre>
while condition {<br/>
    stuff<br/>
};<br/>
</pre>

> Variables<br/>
<pre>
let identifier = expression;<br/>
e.g. let x = 5;<br/>
</pre>

> Functions<br/>
<pre>
fun identifier = (parameters) {<br/>
    stuff<br/>
    return expression;<br/>
};<br/>
e.g. fun add = (x, y) {<br/>
    return x + y;<br/>
};
</pre>

> Builtin functions
<pre>
  ⨠ random(origin, range) generates a random number between the origin and the range
<pre>random(2, 15)</pre>
  ⨠ scope() Shows the variables and functions in that scope at that moment
<pre>scope()</pre>
  ⨠ input("num" or "str") takes input and converts it to a num or a string
<pre>input("num"), input("str")</pre>
  ⨠ output(params) prints the parameters
<pre>output("Hello world!")</pre>
  ⨠ free(param) frees a variable or function from memory, useful for recursion
<pre>free(x)</pre>
  ⨠ abort(message) stops the program and displays a message
<pre>abort("Something went wrong")</pre>
  ⨠ concat(params) concatenates strings
<pre>concat("Hello ", "world", "!")</pre>
</pre>

> Quirks about the language
  - expressions are evaluated from right to left 
  - there is no operator precedence, you can use (grouping)
  - you can declare functions inside other functions, and they will be in that scope
