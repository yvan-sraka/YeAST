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
    let path = format!("/tmp/yeast_{}", uuid);
    let filename = CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o644); }
    return path;
}

fn exec(bin: &str, cin: &str) -> String {
    let cmd = format!("{} {}", bin, cin);
    let output = Command::new("sh").args(&["-c", &cmd]).output()
        .expect("Error: failed to execute process");
    format!("{}", String::from_utf8_lossy(&output.stdout))
}

fn rec(reader: &mut BufRead) -> String {
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line)
            .expect("Error: couldn't read the file");
        if len == 0 { break }
        if line.contains("#!") {
            let cin = pipe(&uuid());
            let cout = cin.clone();
            let input = rec(reader);
            let writer = thread::spawn(move || {
                let mut file = OpenOptions::new().write(true).open(cin)
                    .expect("Error: couldn't open the file");
                file.write_all(input.as_bytes())
                    .expect("Error: couldn't write the file");
            });
            let cursor = line.find("#!").unwrap();
            let cmd = String::from(&line[cursor + 2..line.trim_right().len()]);
            let reader = thread::spawn(move || -> String {
                exec(&cmd, &cout)
            });
            writer.join().expect("Error: couldn't join thread");
            line = String::from(&line[..cursor]) + &reader.join().unwrap();
        } else if line.contains("!#") {
            return String::from(&line[..line.find("!#").unwrap()]);
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
    print!("{}", rec(&mut reader));
}
