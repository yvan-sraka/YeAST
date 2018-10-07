extern crate libc;
extern crate rand;
use rand::Rng;
use std::io::prelude::*;

fn err(msg: &str) -> String { format!("\n\n\x1b[31merror:\x1b[0m {}\n", msg) }

fn exec(cmd: String, input: String) -> String {
    // Create a named pipe using libc (see man 2 mkfifo)
    std::fs::create_dir_all("/tmp/yeast/")
        .expect(&err("couldn't create the /tmp/yeast/ folder"));
    let uuid: String = rand::thread_rng().gen_ascii_chars().take(64).collect();
    let path = format!("/tmp/yeast/{}", uuid);
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o644); }
    // Read and write on the same unique named pipe in different threads
    let cin = path.clone();
    let cout = path.clone();
    std::thread::spawn(move || { // Writer thread
        let mut file = std::fs::OpenOptions::new().write(true).open(cout)
            .expect(&err("couldn't open the yeast named pipe")); // PANIC
        file.write_all(input.as_bytes())
            .expect(&err("couldn't write the yeast named pipe")); // PANIC
    }); // This thread don't need to be join
    let cmd = if cmd.contains("$0") {
        str::replace(&cmd, "$0", &cin)
    } else { // Handle `$0` as the path of the named path
        format!("{} {}", cmd, cin)
    };
    let sh = format!("PATH=$HOME/.kombucha/:$PATH {}", cmd);
    let output = std::process::Command::new("sh").args(&["-c", &sh])
        .output().expect(&err(&format!("failed to execute process: {}", sh)));
    if output.status.success() { // Retreive output of shell command
        format!("{}", String::from_utf8_lossy(&output.stdout))
    } else { // Panic on command error
        panic!("\n#! {}\n!#", String::from_utf8_lossy(&output.stderr)
                                      .replace("\n", "\n.\t")) // BACKTRACE
    }
}

fn readline(reader: &mut BufRead, line: &str, args: &Vec<String>) -> String {
    let p = line.trim_right().len(); // Position of '\n' in string
    if p == 0 { panic!("{}", err("a shell command should follow #!")); }
    let cmd = String::from(&line[..p]);
    let mut c = if &cmd[p - 1..] == "\\" { // if line is ended by '\' character
        let mut line = String::new();
        reader.read_line(&mut line)
            .expect(&err("couldn't read the input file buffer")); // PANIC
        String::from(&cmd[..p - 1]) + &readline(reader, &line, &args)
    } else { cmd }; // Return inlined command
    for i in 2..args.len() {
        c = str::replace(&c, &format!("${}", i - 1), &args[i]);
    } c // Allow sub-scripts to read command line arguments
}

fn split2(s: &str, b: &str) -> (String, String) {
    if s.contains(b) { // works like s.splitn(2, b) ...
        let p = s.find(b).unwrap();
        (String::from(&s[..p]), String::from(&s[p + b.len()..]))
    } else { // ... but always return a (String, String) tuple
        (String::from(&s[..]), String::new())
    }
}

fn yeast(reader: &mut BufRead, args: &Vec<String>) -> String {
    let mut buffer = String::new();
    let mut vec = vec![];
    loop { // While there is something in the read buffer
        let len = reader.read_line(&mut buffer)
            .expect(&err("couldn't read the input file buffer")); // PANIC
        if buffer.contains("#!") { // Start of a bloc
            let begin = split2(&buffer, "#!");
            let cmd = readline(reader, &begin.1, &args);
            let end = split2(&yeast(reader, &args), "!#");
            buffer = end.clone().1;
            vec.push(std::thread::Builder::new().name(cmd.clone()).spawn(
                move || -> String {
                    begin.0 + &exec(cmd, end.0) // Launch command process
                }
            ));
        } else if buffer.contains("!#") || len == 0 { // End of a bloc / file
            let mut outputs = String::new();
            for child in vec { // Join all child threads
                outputs += &child.expect(&err("can't unwrap named thread"))
                                 .join().unwrap(); // PANIC
            }
            return outputs + &buffer;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(std::path::Path::new(&args[1]))
        .expect(&err(&format!("couldn't open: {}", &args[1])));
    let mut reader = std::io::BufReader::new(file);
    print!("{}", split2(&yeast(&mut reader, &args), "!#").0);
}
