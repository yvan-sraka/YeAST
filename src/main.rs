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
    path
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

fn rec(contents: &str) -> String {
    let s = contents.find("#!").unwrap();
    let e = contents[s..].find("\n").unwrap();

    let cmd = String::from(&contents[s+2..e]);
    let input = String::from(&contents[e+1..]);

    let cin = pipe(&uuid());
    let cin2 = cin.clone(); // TODO

    let b = thread::spawn(move || {
        let mut f = OpenOptions::new().write(true).open(&cin2).expect("ERROR");
        f.write_all(input.as_bytes()).expect("ERROR");
    });

    let a = thread::spawn(move || -> String {
        exec(&cmd, &cin)
    });

    b.join().expect("ERROR");
    return a.join().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    print!("{}", rec(&contents));
}
