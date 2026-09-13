use std::process::Command;

pub fn run(path: &std::path::Path) -> Result<()> {
    Command::new("installer")
        .arg("-pkg")
        .arg(path)
        .arg("-target")
        .arg("/")
        .status();
    Ok(())
}

fn main() {
    // let mut echo_hello = Command::new("sh");
    // echo_hello.arg("-c").arg("echo hello");
    // let hello_1 = echo_hello.output().expect("failed to execute process");
    // let hello_2 = echo_hello.output().expect("failed to execute process");
}
