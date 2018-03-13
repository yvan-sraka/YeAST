YeAST [![Build Status](https://travis-ci.com/yvan-sraka/yeast.svg?token=r9S39DVzZNKVuhr9yRC6&branch=master)](https://travis-ci.com/yvan-sraka/yeast)
=====

> Yet another shell trick

You already know:

```shell
#! /bin/sh
echo "Hello world"
```

Now with Yeast you can use any command:

```shell
#! cat
Hello world!
```

Which is equivalent to:

```shell
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

... and with few arguments:

```C++
#! clang++ -Wall -Werror -pedantic -std=c++11 \
   -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
  	std::cout << "Hello World!" << std::endl;
  	return 0;
}
```

... and what if it could be possible to ... WOW:

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


## Purposes

- Can be a good TDD project
- Should bootstrap itself
- Should think about extending the project of keep the most POSIX and KISS

Yeast must:

- be a high level language, very easy to use, well documented,
etc ...
- to make the sauce that people make today in Python
or Perl, while being more extensible than these last two!
- provided with a compiler, and thus allow the many
non-existent optimizations in only interpreted languages.
- separate in compartments: the dirty parts of the code, such as the
flow management, must be separated from the clean part, the code
functional for example, cf. Haskell / Rust!


## Few Special Cases to Understand more deeply what's happen here:

### 1. Bunch of #!

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

... is yeasted into:

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "Hello world#!" << std::endl;
	return 0;
}
```

### 2. Inlining issue

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

... is yeasted into:

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "Hello world!
	bar" << std::endl;
	return 0;
}
```

... and will make a pretty ERROR in C++!

So the solution is:

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

... that is yeasted into:

```C++
#! clang++ -o helloWorld $^ & ./helloWorld
#include <iostream>
int main() {
	std::cout << "Hello world!\nbar" << std::endl;
	return 0;
}
```

Execution of script by stacking / unstacking context, this is the
currently coded part.

### Compiler:

TO DO: construction of an AST, then execution from the sheets of the
context.

> "Powerful next generation scripting!"
> Should be asynchronous

### Modules:

Writing modules, according to the different languages / technologies
and standardization of environment variables as needed
of these modules.

- Dynamic shell call:
- Building on demand:

`@zsh` : makes your script more portable.

Modules can have dependencies, Yeast handles resolved before the execution of the script.

`#! @python` : equivalent of `#!/usr/bin/python`

Multiple calls : the link to the shell can be redefined within the script.

`#! @c++` : executable is hidden and signed for faster subsequent execution.

Compiled languages ​​can also be written as modules.
This method also allows the use of extern dynamic library.

### Statements:

`!#` : End of statement, return to previous shell rules.

Statements can be nested and allow meta-templating, the code is injecting from the highest execution layer.

## Dynamic compilation example:

Haskell | GHC -> C
C | Clang -> LLVM
LLVM | Emscripten -> JavaScript

### Export:

`./yeast script.ys -o android`