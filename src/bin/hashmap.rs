use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();
    map.insert(String::from("aaa"), String::from("hello2"));
    map.entry(String::from("aaa")).or_insert(String::from("cccc"));
    map.insert(String::from("aaa"), String::from("hello23"));


    for (k, v) in map {
        println!("{} => {}", k, v)
    }


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}