use std::io::{Error, ErrorKind};

fn main() {
    println!("the result: {}", test_result(true).unwrap());
    println!("the result: {:?}", test_result(false).unwrap());
}

// result Ok(()) or Err(Error::from(ErrorKind))
fn test_result(is_ok: bool) -> Result<String, Error> {
    if is_ok {
        Ok(String::from("hello world"))
    } else {
        Err(Error::from(ErrorKind::NotFound {}))
    }
}