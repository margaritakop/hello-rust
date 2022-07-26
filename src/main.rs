use std::process::Command;
use std::io::{self, Write};

fn main() {

    let output =
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process");
    let hello = output.stdout;
    println!("{:?}", hello);
    let mut stdout = io::stdout().lock();
    stdout.write_all(&hello);


    let mut list_dir = Command::new("ls");
    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");
}
