<!-- cargo-sync-readme start -->

YeAST [![Build Status](https://travis-ci.org/yvan-sraka/YeAST.svg?branch=master)](https://travis-ci.org/yvan-sraka/YeAST)
=========================================================================================================================

*Yet Another Shell Trick*

What's that?
------------

YeAST is an extension of the bourne shell [shebangsyntax](https://en.wikipedia.org/wiki/Shebang_\(Unix\)).

YeAST aims to solve issues of interoperability, parallel programming,
quick prototyping and progressive refactoring by invite users to use
in their scripts and software features of UNIX systems and build things
more [KISS](https://en.wikipedia.org/wiki/KISS_principle).

You can read more about it through:

-   [a blog post](https://yvan-sraka.github.io/1970/01/05/yeast.html) on
    my web page
-   [a lightning talkpresentation](https://raw.githubusercontent.com/yvan-sraka/yvan-sraka.github.io/master/YeAST.pdf)
    of the core idea, implementation, and real-world applications

Quick Running
-------------

These instructions will get you the last stable binaries of yeast on
your local machine for standard usage purposes.

### Binaries installation

#### ..on macOS using `brew`:

You can install YeAST on macOS using our [Homebrew](https://brew.sh/)
custom Tap:

``` {.shell}
brew install yvan-sraka/YeAST/yeast
```

#### ..on others UNIX-like using the install script:

Run the following line in your terminal, then follow the onscreen
instructions:

``` {.shell}
curl https://raw.githubusercontent.com/yvan-sraka/YeAST/master/install.sh -sSf | sh
```

> If you wonder what this previous command do, don't hesitate to inspect
> the script: it runs all instructions of the **Getting Started**
> section.

### Bootstraping

To make scripts easily work out of the box, you can add this little hack
at the beggining of your files:

``` {.shell}
#! /bin/sh
#! @ignore
if ! [ -x "$(command -v yeast)" ]; then
  curl "https://raw.githubusercontent.com/yvan-sraka/YeAST/master/install.sh" -sSf | sh
fi
yeast "$0"
exit
```

### Editors integration

You now have YeAST installed on your machine! Cool next thing to do
could be to get a syntax support extension for your favorite code
editor:

-   [VisualStudioCode](https://marketplace.visualstudio.com/items?itemName=yvan-sraka.yeast)

Getting Started
---------------

These instructions will get you a copy of the project up and running on
your local machine for development and testing purposes.

### Prerequisites

Get a stable [Rust](https://www.rust-lang.org/) toolchain:

``` {.shell}
curl https://sh.rustup.rs -sSf | sh
```

### Building and running

``` {.shell}
git clone git@github.com:yvan-sraka/YeAST.git
cd YeAST
cargo run basic_example.yst
```

Ecosystem
---------

-   [Kombucha](https://github.com/yvan-sraka/Kombucha): a simple aliases
    manager for YeAST
-   [Palombe](https://github.com/yvan-sraka/Palombe): which lets you
    send and receive messages synchronously through different processes
-   [Cleopatra](https://github.com/yvan-sraka/Cleopatra): make YeAST be
    called in place of your standard interpreter in the current
    environment

Real-World Examples
-------------------

The
[basic_example](https://github.com/yvan-sraka/YeAST/blob/master/basic_example.yst)
available in this repository is fun to get a general idea behind
YeAST! But to understand the purpose of this tool, we will go
through examples inspired from *REAL WORLD*, mainly by my scientific
domain: "Bioinformatics and Modeling".

### YeAST works like `cat` outside interpreters

``` {.yeast}
#! /usr/bin/env yeast

This text will be printed on standard output!

Besides we experiment with a Python code we expect to say "Hello World":

#! python3
print("Hello, World!")
#!

Enjoy this nice way of writing code in notebook style.
```

### It provides multithreading by default

``` {.yeast}
#! /usr/bin/env yeast

Here we try to fold 3 different sequences, so we do it in parallel by
allocating 3 threads, there is no additional syntax needed:

#! python3 folding.py
;LCBO - Prolactin precursor - Bovine
; a sample sequence in FASTA format
MDSKGSSQKGSRLLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVSLRDLFDRAVMVSHYIHDLSS
EMFNEFDKRYAQGKGFITMALNSCHTSSLPTPEDKEQAQQTHHEVLMSLILGLLRSWNDPLYHL
VTEVRGMKGAPDAILSRAIEIEEENKRLLEGMEMIFGQVIPGAKETEPYPVWSGLPSLQTKDED
ARYSAFYNLLHCLRRDSSKIDTYLKLLNCRIIYNNNC*
!#

Shebang in YeAST could be any shell command that's accepting as last argument a
given file, here we use the syntax to call our external script `folding.py`.

#! python3 folding.py
>MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
ADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTID
FPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIREA
DIDGDGQVNYEEFVQMMTAK*
!#

Bonus: the output of this multithreading computation will be displayed the right
order as soon that the information is available!

#! python3 folding.py
>gi|5524211|gb|AAD44166.1| cytochrome b [Elephas maximus maximus]
LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV
EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG
LLILILLLLLLALLSPDMLGDPDNHMPADPLNTPLHIKPEWYFLFAYAILRSVPNKLGGVLALFLSIVIL
GLMPFLHTSKHRSMMLRPLSQALFWTLTMDLLTLTWIGSQPVEYPYTIIGQMASILYFSIILAFLPIAGX
IENY
!#
```

Roadmap
-------

-   [x] Create a Makefile to handle `make`, `make test` & `make install`
    intructions.
-   [x] Validate builds using Travis CI.
-   [x] Read input file line by line using a Buffer.
-   [x] Use [clap.rs](https://github.com/kbknapp/clap-rs) to enhance
    command line interactions.
-   [x] Allow sub-scripts to read command-line arguments, using `$0`,
    `$1`, etc ...
-   [x] ... and `$*` for all input args.
-   [x] Handle scripts errors correctly and display a `BACKTRACE` in
    `=1|full` mode
-   [x] Let the use of `#! /usr/bin/env yeast` not failed in stack
    overflow due to an infinite loop of self-calls.
-   [x] Fix rules for inlining shell commands in scripts.
-   [x] Let the user write multiline commands as he did in an interactive
    shell.
-   [x] Add bunch of input / output tests scripts.
-   [x] Provide true multithreading by getting ride of the waiting lock of
    shell command output!
-   [ ] Write a beautiful and comprehensive `man` page.
-   [x] Create an install script for handling one-instruction
    installation.
-   [ ] Create packages for most UNIX distributions.
-   [ ] Host a beautiful landing page on the `gh-page` branch of this
    repository.
-   [ ] Add Windows support (use `cmd` instead of `sh` and [Windowsnamedpipes](https://msdn.microsoft.com/en-us/library/windows/desktop/aa365590\(v=vs.85\).aspx)
    instead of `mkfifo`).
-   [x] Create a small lib in each language to uniform the use of named
    pipes in sharing serialized data between processes.
-   [x] Provide a collection of alias by adding a special environment to
    the `PATH` of command launch by YeAST.
-   [x] Add a simple script in the path to update aliases by running a
    `git pull`.
-   [x] Provide plugins for syntactic coloration in VSCode.
-   [x] Let user define a top level interpreter by set `YEAST_CONTEXT`
    env variable

Contributing
------------

Please read
[CONTRIBUTING.md](https://github.com/yvan-sraka/YeAST/blob/master/CONTRIBUTING.md)
for details on our code of conduct, and the process for submitting a pull
requests to us.

Authors
-------

-   [Yvan Sraka](https://github.com/yvan-sraka) - for the original idea
-   [Mattia Primavera](https://github.com/MattiaPrimavera/) - who
    provide constant positive feedback and was the very first
    beta-tester

See also the list of
[contributors](https://github.com/yvan-sraka/YeAST/graphs/contributors)
who participated in this project.

License
-------

This project is licensed under the 3rd version of the GPL License - see the
[LICENSE](https://github.com/yvan-sraka/YeAST/blob/master/LICENSE) file
for details.

<!-- cargo-sync-readme end -->
