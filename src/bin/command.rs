use std::process::Command;

// http://llever.com/rust-cookbook-zh/os/external.zh.html
fn main() {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .unwrap();
    if !output.status.success() {
        println!("error");
    }
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|cap| {
            println!("{:?}", cap)
        });

}