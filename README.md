YeAST
=====

Powerful next generation scripting !


### Modules :

`@zsh` : makes your script more portable.

Modules can have dependencies, Yeast handles resolved before the execution of the script.


### Dynamic shell call :

`#! @python` : equivalent of `#!/usr/bin/python`

Multiple calls : the link to the shell can be redefined within the script.


### Building on demand :

`#! @c++` : executable is hidden and signed for faster subsequent execution.

Compiled languages ​​can also be written as modules.
This method also allows the use of extern dynamic library.


###  Remote shell (testing) :

`#! https://user@server.org/shell`

Everything is in the title, yeast takes care of maintaining the connection with your remote shell.


### Statements :

`!#` : End of statement, return to previous shell rules.

Statements can be nested and allow meta-templating, the code is injecting from the highest execution layer.


### Flags (testing) :

`#! @bash : 1` : Statement marked `1`

`#! 1` : Go to the statement (`&` option will restore the environment of the state, it's like a back to the future).


### Export :

`./yeast script.ys -o android`
