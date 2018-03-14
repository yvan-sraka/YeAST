extern crate libc;
extern crate rand;
use rand::Rng;
use std::io::prelude::*;

fn exec(cmd: String, input: String) -> String {
    // Create a named pipe using libc (see man 2 mkfifo)
    std::fs::create_dir_all("/tmp/yeast/")
        .expect("Error: couldn't create the /tmp/yeast/ folder");
    let uuid: String = rand::thread_rng().gen_ascii_chars().take(32).collect();
    let path = format!("/tmp/yeast/{}", uuid);
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o644); }
    // Read and write on the same unique named pipe in different threads
    let cin = path.clone();
    let cout = path.clone();
    std::thread::spawn(move || { // Writer thread
        let mut file = std::fs::OpenOptions::new().write(true).open(cout)
            .expect("Error: couldn't open the named pipe");
        file.write_all(input.as_bytes())
            .expect("Error: couldn't write the named pipe");
    }); // This thread don't need to be join
    let sh = format!("export PATH=$PATH:~/.yeast/; {} {}", cmd, cin);
    let output = std::process::Command::new("sh").args(&["-c", &sh])
        .output().expect("Error: failed to execute process");
    if output.status.success() { // Retreive output of shell command
        format!("{}", String::from_utf8_lossy(&output.stdout))
    } else { // Panic on command error
        panic!("Error: {}", String::from_utf8_lossy(&output.stderr))
    }
}

fn readline(reader: &mut BufRead, line: &str) -> String {
    let p = line.trim_right().len(); // Position of '\n' in string
    if p == 0 { panic!("Error: you have to write a shell command next to #!"); }
    let cmd = String::from(&line[..p]);
    if &cmd[p - 1..] == "\\" { // if line is ended by '\' character
        let mut line = String::new();
        reader.read_line(&mut line)
            .expect("Error: couldn't read the input file");
        String::new() + &cmd[..p - 1] + &readline(reader, &line)
    } else { cmd } // Return inlined command
}

fn split2(s: &str, b: &str) -> (String, String) {
    if s.contains(b) { // works like s.splitn(2, b) ...
        let p = s.find(b).unwrap();
        (String::from(&s[..p]), String::from(&s[p + b.len()..]))
    } else { // ... but always return a (String, String) tuple
        (String::from(&s[..]), String::new())
    }
}

fn yeast(reader: &mut BufRead) -> String {
    let mut buffer = String::new();
    let mut vec = vec![];
    loop { // While there is something in the read buffer
        let len = reader.read_line(&mut buffer)
            .expect("Error: couldn't read the input file");
        if buffer.contains("#!") { // Start of a bloc
            let begin = split2(&buffer, "#!");
            let cmd = readline(reader, &begin.1);
            let end = split2(&yeast(reader), "!#");
            buffer = end.clone().1;
            vec.push(std::thread::spawn(move || -> String {
                begin.0 + &exec(cmd, end.0) // Launch command process
            }));
        } else if buffer.contains("!#") || len == 0 { // End of a bloc / file
            let mut outputs = String::new();
            for child in vec { // Join all child threads
                outputs += &child.join().unwrap();
            }
            return outputs + &buffer;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(std::path::Path::new(&args[1]))
        .expect("Error: couldn't open the input file");
    let mut reader = std::io::BufReader::new(file);
    print!("{}", split2(&yeast(&mut reader), "!#").0);
}
