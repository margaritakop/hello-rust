use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::process::Command;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();


    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };

    let hello = output.stdout;
    println!("{:?}", hello);
    // TODO: format with
    // use std::io::Write;

    let mut list_dir = Command::new("ls");
    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");
}
