YeAST [![Build Status](https://travis-ci.com/yvan-sraka/yeast.svg?token=r9S39DVzZNKVuhr9yRC6&branch=master)](https://travis-ci.com/yvan-sraka/yeast)
=====

Powerful next generation scripting !


You already know:

```Shell
#! /bin/sh
echo "Hello world"
```

Now with Yeast you can use any command:

```Shell
#! cat
Hello world!
```

Which is equivalent to:

```Shell
#! cat $^
Hello world!
```

`$^`: Temporary file containing the block

Let's try:

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
  	std::cout << "Hello World!" << std::endl;
  	return 0;
}
```

TO DO

```C++
#! clang++ -Wall -Werror -pedantic -std=c++11 \
   -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
  	std::cout << "Hello World!" << std::endl;
  	return 0;
}
```

TO DO

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "
	#! /usr/bin/python
		print("Hello world!")
	!#
	" << std::endl;
	return 0;
}
```


---- Special Cases ----

1.

TO DO

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "
	#! /usr/bin/python
		print("Hello world#!") #! foo
	!#
	" << std::endl;
	return 0;
}
```

TO DO

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "Hello world#!" << std::endl;
	return 0;
}
```


2.

TO DO

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "
	#! /usr/bin/python
		print("Hello world!")
		print("bar")
	!#
	" << std::endl;
	return 0;
}
```

TO DO

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "Hello world!
	bar" << std::endl;
	return 0;
}
```

--> ERROR

Solution: 

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "
	#! @inline
		#! /usr/bin/python
			print("Hello world!")
			print("bar")
		!#
	#!
	" << std::endl;
	return 0;
}
```

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "Hello world!\nbar" << std::endl;
	return 0;
}
```

Powerful next generation scripting !


### Modules : ### Dynamic shell call : ### Building on demand :

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
