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

fn rec(contents: &str, step: u32) -> (String, u32) {
    let mut output = String::from("");
    let mut current = step; let mut i = 0; // UGLY
    for line in contents.lines() { if i >= current { // UGLY
        if line.contains("#!") {
            let (input, next) = rec(contents, i + 1); // UGLY
            let cmd = String::from(&line[line.find("#!").unwrap() + 2..]);
            let cout = pipe(&uuid());
            let cin = cout.clone();
            let writer = thread::spawn(move || {
                let mut file = OpenOptions::new().write(true).open(cin)
                    .expect("Error: couldn't open the file");
                file.write_all(input.as_bytes())
                    .expect("Error: couldn't write the file");
            });
            let reader = thread::spawn(move || -> String {
                exec(&cmd, &cout)
            });
            writer.join().expect("ERROR");
            output += &format!("\n{}", &reader.join().unwrap());
            current = next; // UGLY
        } else if line.contains("!#") {
            return (output, i + 1);
        } else {
            output += line;
        }
    } i += 1; } // UGLY
    return (output, i);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)
        .expect("Error: couldn't open the file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)
        .expect("Error: couldn't read the file");
    print!("{}", rec(&contents, 0).0); // UGLY
}
