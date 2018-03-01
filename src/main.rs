extern crate libc;
extern crate rand;
use rand::Rng;
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::thread;

fn uuid() -> String {
    rand::thread_rng().gen_ascii_chars().take(16).collect() // TODO
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
                                   .expect("failed to execute process");
    // println!("status: {}", output.status); // DEBUG
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    // assert!(output.status.success()); // DEBUG
    format!("{}", String::from_utf8_lossy(&output.stdout))
}

fn rec(contents: &str, l: u32) -> (String, u32) {
    let mut output = String::from("");
    let mut i = 0;
    let mut j = l;
    for line in contents.lines() { if i >= j {
        if line.contains("#!") {
            let (x, y) = rec(contents, i + 1);
            let cmd = String::from(&line[line.find("#!").unwrap()+2..]);
            let input = x.clone();

            let cin = pipe(&uuid());
            let cin2 = cin.clone(); // TODO

            let b = thread::spawn(move || {
                let mut file = OpenOptions::new().write(true).open(cin2)
                        .expect("ERROR");
                file.write_all(input.as_bytes()).expect("ERROR");
            });

            let a = thread::spawn(move || -> String {
                exec(&cmd, &cin)
            });

            b.join().expect("ERROR");

            output += "\n";
            output += &a.join().unwrap();

            j = y;
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
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading the file");
    print!("{}", rec(&contents, 0).0);
}
