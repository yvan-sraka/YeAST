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
        panic!(msg)
    });
    let uuid: String = rand::thread_rng().gen_ascii_chars().take(64).collect();
    let path = format!("/tmp/yeast/{}", uuid);
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe {
        libc::mkfifo(filename.as_ptr(), 0o600);
    }
    // Read and write on the same unique named pipe in different threads
    let cin = path.clone();
    let cout = path.clone();
    std::thread::spawn(move || {
        // Writer thread
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(cout)
            .unwrap_or_else(|_| {
                let msg = err("couldn't open the yeast named pipe");
                panic!(msg)
            });
        file.write_all(input.as_bytes()).unwrap_or_else(|_| {
            let msg = err("couldn't write the yeast named pipe");
            panic!(msg)
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
            panic!(msg)
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

fn readline(reader: &mut BufRead, line: &str, args: &[String]) -> String {
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
            panic!(msg)
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
        (String::from(&s[..]), String::new())
    }
}

fn yeast(
    reader: &mut BufRead,
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
                        panic!(msg)
                    })
                    .join()
                    .unwrap(); // PANIC
            }
            return outputs + &buffer;
        }
        len = reader.read_line(&mut buffer).unwrap_or_else(|_| {
            let msg = err("couldn't read the input file buffer");
            panic!(msg)
        });
    }
}

fn main() -> Result<()> {
    App::new("YeAST")
        .version("0.15.0")
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
        panic!(msg)
    });
    let mut reader = std::io::BufReader::new(file);
    let context = std::env::var_os("YEAST_CONTEXT");
    print!("{}", split2(&yeast(&mut reader, &args, &context), "!#").0);
    Ok(())
}
