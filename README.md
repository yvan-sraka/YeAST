# YeAST [![Build Status](https://travis-ci.org/yvan-sraka/YeAST.svg?branch=master)](https://travis-ci.org/yvan-sraka/YeAST)

Yet Another Shell Trick

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Get a stable [Rust](https://www.rust-lang.org/) toolchain:

```shell
curl https://sh.rustup.rs -sSf | sh
```

### Building and running

```shell
git clone git@github.com:yvan-sraka/YeAST.git
cd YeAST
cargo run basic_example
```

## Roadmap

- [x] Create a Makefile to handle `make`, `make test` & `sudo make install` intructions.
- [x] Validate builds using Travis CI.
- [x] Read input file line by line using a Buffer.
- [ ] Use [clap.rs](https://github.com/kbknapp/clap-rs) to enhance command line interactions?
- [ ] Allow sub-scripts to read command line arguments.
- [ ] Handle scripts errors correctly and display a `BACKTRACE` in `--debug` mode
- [x] Let the use of `#! /usr/bin/env yeast` not failed in stack overflow due of an infinite loop of self calls.
- [x] Fix rules for inlining shell commands in scripts.
- [ ] Let user write multiline commands as he did in an interactive shell.
- [ ] Add bunch of input / output tests scripts.
- [ ] Provide true multithreading by get ride of the waiting lock of shell command output!
- [ ] Write a beautiful and comprehensive `man` page.
- [ ] Create an install script and packages for most of UNIX distributions.
- [ ] Host a beautiful landing page on the `gh-page` branch of this repository.
- [ ] Add Windows support (use `cmd` instead of `sh` and [Windows named pipes](https://msdn.microsoft.com/en-us/library/windows/desktop/aa365590(v=vs.85).aspx) instead of `mkfifo`).
- [ ] Create a small lib in each language to uniform the use a named pipes in sharing serialized data between process.
- [x] Provide a collection of alias by adding a special environment to the `PATH` of command launch by YeAST.
- [ ] Add a simple script in the path to update aliases by running a `git pull`.
- [ ] Re-write YeAST `Makefile` and other scripts using YeAST!
- [ ] Provide plugins for syntaxic coloration in commons editors.

## Contributing

Please read [CONTRIBUTING.md](https://github.com/yvan-sraka/YeAST/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* [Yvan Sraka](https://github.com/yvan-sraka)

See also the list of [contributors](https://github.com/yvan-sraka/YeAST/graphs/contributors) who participated in this project.

## License

This project is licensed under the 3rd version of GPL License - see the [LICENSE](https://github.com/yvan-sraka/YeAST/blob/master/LICENSE) file for details.
