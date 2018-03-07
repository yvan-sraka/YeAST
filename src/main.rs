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
    return path;
}

fn exec(bin: &str, cin: &str) -> String {
    let cmd = format!("export PATH=$PATH:~/.yeast/aliases/; {} {}", bin, cin);
    let output = Command::new("sh").args(&["-c", &cmd]).output()
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

fn rec(reader: &mut BufRead) -> String {
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line)
            .expect("Error: couldn't read the file");
        if len == 0 { break }
        if line.contains("#!") {
            let r = rec(reader);
            let input = if r.contains("!#") {
                String::from(&r[..r.find("!#").unwrap()])
            } else {
                String::from(&r[..])
            };
            let cin = pipe(&uuid());
            let cout = cin.clone();
            let writer = thread::spawn(move || {
                let mut file = OpenOptions::new().write(true).open(cin)
                    .expect("Error: couldn't open the file");
                file.write_all(input.as_bytes())
                    .expect("Error: couldn't write the file");
            });
            let begin = line.find("#!").unwrap();
            let cmd = String::from(&line[begin + 2..line.trim_right().len()]);
            let reader = thread::spawn(move || -> String {
                exec(&cmd, &cout)
            });
            writer.join().expect("Error: couldn't join thread");
            line = String::from(&line[..begin]) + &reader.join().unwrap();
            if r.contains("!#") {
                line += &r[r.find("!#").unwrap() + 2..];
            }
        } else if line.contains("!#") {
            return line;
        }
    }
    return line;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)
        .expect("Error: couldn't open the file");
    let mut reader = BufReader::new(file);
    let r = rec(&mut reader);
    print!("{}", if r.contains("!#") {
        &r[..r.find("!#").unwrap()]
    } else {
        &r[..]
    });
}
