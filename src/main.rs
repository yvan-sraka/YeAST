//! YeAST [![Build Status](https://travis-ci.org/yvan-sraka/YeAST.svg?branch=master)](https://travis-ci.org/yvan-sraka/YeAST)
//! =========================================================================================================================
//!
//! *Yet Another Shell Trick*
//!
//! What's that?
//! ------------
//!
//! YeAST is an extension of the bourne shell [shebang
//! syntax](https://en.wikipedia.org/wiki/Shebang_(Unix)).
//!
//! YeAST aims to solve issues of interoperability, parallel programming,
//! quick prototyping and progressive refactoring by incitate users to use
//! in their scripts and software features of UNIX system and build things
//! more [KISS](https://en.wikipedia.org/wiki/KISS_principle).
//!
//! You can read more about it through:
//!
//! -   [a blog post](https://yvan-sraka.github.io/2018/06/13/yeast.html) on
//!     my personnal web page
//! -   [a lightning talk
//!     presentation](https://raw.githubusercontent.com/yvan-sraka/yvan-sraka.github.io/master/YeAST.pdf)
//!     of the core idea, implementation and real world applications
//!
//! Quick Running
//! -------------
//!
//! These instructions will get you the last stable binaries of yeast on
//! your local machine for standard usage purposes.
//!
//! ### Binaries installation
//!
//! #### ..on macOS using `brew`:
//!
//! You can install YeAST on macOS using our [Homebrew](https://brew.sh/)
//! custom Tap:
//!
//! ``` {.shell}
//! brew install yvan-sraka/YeAST/yeast
//! ```
//!
//! #### ..on others UNIX-like using the install script:
//!
//! Run the following line in your terminal, then follow the onscreen
//! instructions:
//!
//! ``` {.shell}
//! curl https://raw.githubusercontent.com/yvan-sraka/YeAST/master/install.sh -sSf | sh
//! ```
//!
//! > If you wonder what this previous command do, don't hesitate to inspect
//! > the script: it runs all instructions of the **Getting Started**
//! > section.
//!
//! ### Bootstraping
//!
//! To make scripts easily work out of the box, you can add this little hack
//! at the beggining of your files:
//!
//! ``` {.shell}
//! #! /bin/sh
//! #! @ignore
//! if ! [ -x "$(command -v yeast)" ]; then
//!   curl "https://raw.githubusercontent.com/yvan-sraka/YeAST/master/install.sh" -sSf | sh
//! fi
//! yeast "$0"
//! exit
//! ```
//!
//! ### Editors integration
//!
//! You now have YeAST installed on your machine! Cool next thing to do
//! could be to get a syntax support extension for your favorite code
//! editor:
//!
//! -   [VisualStudio
//!     Code](https://marketplace.visualstudio.com/items?itemName=yvan-sraka.yeast)
//!
//! Getting Started
//! ---------------
//!
//! These instructions will get you a copy of the project up and running on
//! your local machine for development and testing purposes.
//!
//! ### Prerequisites
//!
//! Get a stable [Rust](https://www.rust-lang.org/) toolchain:
//!
//! ``` {.shell}
//! curl https://sh.rustup.rs -sSf | sh
//! ```
//!
//! ### Building and running
//!
//! ``` {.shell}
//! git clone git@github.com:yvan-sraka/YeAST.git
//! cd YeAST
//! cargo run basic_example.yst
//! ```
//!
//! Ecosystem
//! ---------
//!
//! -   [Kombucha](https://github.com/yvan-sraka/Kombucha): a simple aliases
//!     manager for YeAST
//! -   [Palombe](https://github.com/yvan-sraka/Palombe): which lets you
//!     send and receive messages synchronously through different processes
//! -   [Cleopatra](https://github.com/yvan-sraka/Cleopatra): make YeAST be
//!     called in place of your standard interpreter in the current
//!     environment
//!
//! Real World Examples
//! -------------------
//!
//! The
//! [basic\_example](https://github.com/yvan-sraka/YeAST/blob/master/basic_example.yst)
//! available in this repository is fun to get the general idea behind
//! YeAST! But to really understand the purpose of this tool, we will go
//! through examples inspired from *REAL WORLD*, mainly by my scientific
//! domain: "Bioinformatics and Modeling".
//!
//! ### YeAST works like `cat` outside interpreters
//!
//! ``` {.yeast}
//! #! /usr/bin/env yeast
//!
//! This text will be printed on standard output!
//!
//! Beside we experiment with a Python code we expect to say "Hello World":
//!
//! #! python3
//! print("Hello, World!")
//! #!
//!
//! Enjoy this nice way of writing code in notebook style.
//! ```
//!
//! ### It provides multithreading by default
//!
//! ``` {.yeast}
//! #! /usr/bin/env yeast
//!
//! Here we try to fold 3 different sequences, so we do it in parallel by
//! allocating 3 threads, there is no additonal syntax needed:
//!
//! #! python3 folding.py
//! ;LCBO - Prolactin precursor - Bovine
//! ; a sample sequence in FASTA format
//! MDSKGSSQKGSRLLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVSLRDLFDRAVMVSHYIHDLSS
//! EMFNEFDKRYAQGKGFITMALNSCHTSSLPTPEDKEQAQQTHHEVLMSLILGLLRSWNDPLYHL
//! VTEVRGMKGAPDAILSRAIEIEEENKRLLEGMEMIFGQVIPGAKETEPYPVWSGLPSLQTKDED
//! ARYSAFYNLLHCLRRDSSKIDTYLKLLNCRIIYNNNC*
//! !#
//!
//! Shebang in YeAST could be any shell command that's accepting as last argument a
//! given file, here we use the syntax to call our external script `folding.py`.
//!
//! #! python3 folding.py
//! >MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
//! ADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTID
//! FPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIREA
//! DIDGDGQVNYEEFVQMMTAK*
//! !#
//!
//! Bonus: the output of this multhreading computation will be displayed the in right
//! order as soon that the information is available!
//!
//! #! python3 folding.py
//! >gi|5524211|gb|AAD44166.1| cytochrome b [Elephas maximus maximus]
//! LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV
//! EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG
//! LLILILLLLLLALLSPDMLGDPDNHMPADPLNTPLHIKPEWYFLFAYAILRSVPNKLGGVLALFLSIVIL
//! GLMPFLHTSKHRSMMLRPLSQALFWTLTMDLLTLTWIGSQPVEYPYTIIGQMASILYFSIILAFLPIAGX
//! IENY
//! !#
//! ```
//!
//! Roadmap
//! -------
//!
//! -   [x] Create a Makefile to handle `make`, `make test` & `make install`
//!     intructions.
//! -   [x] Validate builds using Travis CI.
//! -   [x] Read input file line by line using a Buffer.
//! -   [x] Use [clap.rs](https://github.com/kbknapp/clap-rs) to enhance
//!     command line interactions.
//! -   [x] Allow sub-scripts to read command line arguments, using `$0`,
//!     `$1`, etc ...
//! -   [x] ... and `$*` for all input args.
//! -   [x] Handle scripts errors correctly and display a `BACKTRACE` in
//!     `=1|full` mode
//! -   [x] Let the use of `#! /usr/bin/env yeast` not failed in stack
//!     overflow due of an infinite loop of self calls.
//! -   [x] Fix rules for inlining shell commands in scripts.
//! -   [x] Let user write multiline commands as he did in an interactive
//!     shell.
//! -   [x] Add bunch of input / output tests scripts.
//! -   [x] Provide true multithreading by get ride of the waiting lock of
//!     shell command output!
//! -   [ ] Write a beautiful and comprehensive `man` page.
//! -   [x] Create an install script for handling one-instruction
//!     installation.
//! -   [ ] Create packages for most of UNIX distributions.
//! -   [ ] Host a beautiful landing page on the `gh-page` branch of this
//!     repository.
//! -   [ ] Add Windows support (use `cmd` instead of `sh` and [Windows
//!     named
//!     pipes](https://msdn.microsoft.com/en-us/library/windows/desktop/aa365590(v=vs.85).aspx)
//!     instead of `mkfifo`).
//! -   [x] Create a small lib in each language to uniform the use a named
//!     pipes in sharing serialized data between process.
//! -   [x] Provide a collection of alias by adding a special environment to
//!     the `PATH` of command launch by YeAST.
//! -   [x] Add a simple script in the path to update aliases by running a
//!     `git pull`.
//! -   [x] Provide plugins for syntaxic coloration in VSCode.
//! -   [x] Let user define a top level interpreter by set `YEAST_CONTEXT`
//!     env variable
//!
//! Contributing
//! ------------
//!
//! Please read
//! [CONTRIBUTING.md](https://github.com/yvan-sraka/YeAST/blob/master/CONTRIBUTING.md)
//! for details on our code of conduct, and the process for submitting pull
//! requests to us.
//!
//! Authors
//! -------
//!
//! -   [Yvan Sraka](https://github.com/yvan-sraka) - for the original idea
//! -   [Mattia Primavera](https://github.com/MattiaPrimavera/) - who
//!     provide constant positive feedback and was the very first
//!     beta-tester
//!
//! See also the list of
//! [contributors](https://github.com/yvan-sraka/YeAST/graphs/contributors)
//! who participated in this project.
//!
//! License
//! -------
//!
//! This project is licensed under the 3rd version of GPL License - see the
//! [LICENSE](https://github.com/yvan-sraka/YeAST/blob/master/LICENSE) file
//! for details.

extern crate libc;
extern crate rand;
use rand::Rng;
extern crate clap;
use ansi_term::Colour::Red;
use clap::{App, AppSettings, Arg};
use std::io::prelude::*;
use std::io::Result;

fn err(msg: &str) -> String {
    format!("\n\n{} {}\n", Red.bold().paint("error:"), msg)
}

fn exec(cmd: &str, input: String) -> String {
    // Create a named pipe using libc (see man 2 mkfifo)
    std::fs::create_dir_all("/tmp/yeast/").unwrap_or_else(|_| {
        let msg = err("couldn't create the /tmp/yeast/ folder");
        panic!("{}", msg)
    });
    let uuid: String = rand::thread_rng().gen_ascii_chars().take(64).collect();
    let path = format!("/tmp/yeast/{}", uuid);
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe {
        libc::mkfifo(filename.as_ptr(), 0o600);
    }
    // Read and write on the same unique named pipe in different threads
    let cin = path.clone();
    let cout = path;
    std::thread::spawn(move || {
        // Writer thread
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(cout)
            .unwrap_or_else(|_| {
                let msg = err("couldn't open the yeast named pipe");
                panic!("{}", msg)
            });
        file.write_all(input.as_bytes()).unwrap_or_else(|_| {
            let msg = err("couldn't write the yeast named pipe");
            panic!("{}", msg)
        });
    }); // This thread don't need to be join
    let cmd = if cmd.contains("$0") {
        str::replace(&cmd, "$0", &cin)
    } else {
        // Handle `$0` as the path of the named path
        format!("{} {}", cmd, cin)
    };
    let sh = format!("PATH=$HOME/.kombucha/:$PATH {}", cmd);
    let output = std::process::Command::new("sh")
        .args(&["-c", &sh])
        .output()
        .unwrap_or_else(|_| {
            let msg = err(&format!("failed to execute process: {}", sh));
            panic!("{}", msg)
        });
    if output.status.success() {
        // Retreive output of shell command
        format!("{}", String::from_utf8_lossy(&output.stdout))
    } else {
        // Panic on command error
        panic!(
            "\n#! {}\n!#",
            String::from_utf8_lossy(&output.stderr).replace("\n", "\n.\t")
        ) // BACKTRACE
    }
}

fn readline(reader: &mut dyn BufRead, line: &str, args: &[String]) -> String {
    let p = line.trim_end().len(); // Position of '\n' in string
    if p == 0 {
        panic!("{}", err("a shell command should follow #!"));
    }
    let cmd = String::from(&line[..p]);
    let mut c = if &cmd[p - 1..] == "\\" {
        // if line is ended by '\' character
        let mut line = String::new();
        reader.read_line(&mut line).unwrap_or_else(|_| {
            let msg = err("couldn't read the input file buffer");
            panic!("{}", msg)
        });
        String::from(&cmd[..p - 1]) + &readline(reader, &line, &args)
    } else {
        cmd
    }; // Return inlined command
    c = str::replace(&c, "$*", &args[2..].join(" "));
    for (i, arg) in args.iter().enumerate().skip(2) {
        c = str::replace(&c, &format!("${}", i - 1), arg);
    }
    c // Allow sub-scripts to read command line arguments
}

fn split2(s: &str, b: &str) -> (String, String) {
    if s.contains(b) {
        // works like s.splitn(2, b) ...
        let p = s.find(b).unwrap();
        (String::from(&s[..p]), String::from(&s[p + b.len()..]))
    } else {
        // ... but always return a (String, String) tuple
        (String::from(s), String::new())
    }
}

fn yeast(
    reader: &mut dyn BufRead,
    args: &[String],
    context: &std::option::Option<std::ffi::OsString>,
) -> String {
    let mut buffer = String::new();
    let mut vec = vec![];
    let mut len = 1; // Allow us to check context on first loop turn
    loop {
        // While there is something in the read buffer
        if (context.is_some() && len != 0) // YEAST_CONTEXT is defined ...
            || buffer.contains("#!")
        {
            // ... or a bloc start
            let begin = split2(&buffer, "#!");
            let cmd = match context.clone() {
                Some(x) => x.into_string().unwrap(),
                None => readline(reader, &begin.1, &args),
            };
            let end = split2(&yeast(reader, &args, &None), "!#");
            buffer = end.clone().1;
            vec.push(
                std::thread::Builder::new()
                    .name(cmd.clone())
                    .spawn(move || -> String {
                        begin.0 + &exec(&cmd, end.0) // Launch command process
                    }),
            );
        } else if buffer.contains("!#") || len == 0 {
            // End of a bloc / file
            let mut outputs = String::new();
            for child in vec {
                // Join all child threads
                outputs += &child
                    .unwrap_or_else(|_| {
                        let msg = err("can't unwrap named thread");
                        panic!("{}", msg)
                    })
                    .join()
                    .unwrap(); // PANIC
            }
            return outputs + &buffer;
        }
        len = reader.read_line(&mut buffer).unwrap_or_else(|_| {
            let msg = err("couldn't read the input file buffer");
            panic!("{}", msg)
        });
    }
}

fn main() -> Result<()> {
    App::new("YeAST")
        .version("0.20.0")
        .author("Yvan SRAKA <yvan@sraka.pw>")
        .about("Yet Another Shell Trick")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .multiple(true),
        )
        .get_matches();
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(std::path::Path::new(&args[1])).unwrap_or_else(|_| {
        let msg = err(&format!("couldn't open: {}", &args[1]));
        panic!("{}", msg)
    });
    let mut reader = std::io::BufReader::new(file);
    let context = std::env::var_os("YEAST_CONTEXT");
    print!("{}", split2(&yeast(&mut reader, &args, &context), "!#").0);
    Ok(())
}
