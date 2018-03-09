extern crate libc;
extern crate rand;
use rand::Rng;
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;
use std::thread;

fn uuid() -> String {
    rand::thread_rng().gen_ascii_chars().take(32).collect()
}

fn pipe(uuid: &str) -> String {
    let path = format!("/tmp/yeast/pipes/{}", uuid);
    let filename = CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o644); }
    path
}

fn exec(cmd: &str, cin: &str) -> String {
    let sh = format!("export PATH=$PATH:~/.yeast/aliases/; {} {}", cmd, cin);
    let output = Command::new("sh").args(&["-c", &sh]).output()
        .expect("Error: failed to execute process");
    if output.status.success() {
        format!("{}", String::from_utf8_lossy(&output.stdout))
    } else {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        exec("cat", cin)
    }
}

fn thread(cmd: String, input: String) -> String {
    let cin = pipe(&uuid());
    let cout = cin.clone();
    let writer = thread::spawn(move || {
        let mut file = OpenOptions::new().write(true).open(cin)
            .expect("Error: couldn't open the file");
        file.write_all(input.as_bytes())
            .expect("Error: couldn't write the file");
    });
    let reader = thread::spawn(move || -> String {
        exec(&cmd, &cout)
    });
    writer.join().expect("Error: couldn't join thread");
    reader.join().unwrap()
}

fn split(s: &str, b: &str) -> (String, String) {
    if s.contains(b) {
        (String::from(&s[..s.find(b).unwrap()]),
         String::from(&s[s.find(b).unwrap() + b.len()..]))
    } else {
        (String::from(&s[..]), String::new())
    }
}

fn readline(reader: &mut BufRead, line: &str) -> String {
    let p = line.trim_right().len();
    let cmd = String::from(&line[..p]);
    if &cmd[p - 1..p] == "\\" {
        let mut line = String::new();
        reader.read_line(&mut line)
            .expect("Error: couldn't read the file");
        String::new() + &cmd[..p - 1] + &readline(reader, &line)
    } else {
        cmd
    }
}

fn yeast(reader: &mut BufRead) -> String {
    let mut buffer = String::new();
    loop {
        let len = reader.read_line(&mut buffer)
            .expect("Error: couldn't read the file");
        if buffer.contains("#!") {
            let begin = split(&buffer, "#!");
            let cmd = readline(reader, &begin.1);
            let end = split(&yeast(reader), "!#");
            buffer = begin.0 + &thread(cmd, end.0) + &end.1;
        } else if buffer.contains("!#") || len == 0 {
            return buffer;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])
        .expect("Error: couldn't open the file");
    let mut reader = BufReader::new(file);
    print!("{}", split(&yeast(&mut reader), "!#").0);
}
