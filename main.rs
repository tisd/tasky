use std::process::Command;

fn main() {
    let output = Command::new("ps").arg("-e").output().expect("Error");

    println!("{:?}", output);
}
