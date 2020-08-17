use std::env::args;

fn main() {
    let env: Vec<String> = args().collect();
    println!("{:?}", env);
}