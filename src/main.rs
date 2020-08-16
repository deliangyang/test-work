use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::Add;

pub(crate) fn test() {
    println!("Hello world")
}

struct Test {
    name: String,
}

trait Info {
    fn println(&self);
}

impl Info for Test {
    fn println(&self) {
        println!("this is impl for {}", self.name)
    }
}

/// redis client
/// hash *num
/// string $num
/// incrby :num
fn main() {
    println!("Hello, world!");
    test();

    let test = Test {
        name: String::from("hello owrld")
    };
    println!("{}", test.name);

    test.println();
    println!("this is generic types result: {}", test.test2(3));
    println!("this is associated types result: {}", test.test3(3));

    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    stream.write(b"hgetall err\n").unwrap();
    let mut buffer = vec![0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("{}", String::from_utf8(buffer).unwrap())
        }
        _ => {}
    }
    println!("{}", (Millimeters(1000) + Meters(1)).0);
}

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    // 关联类型 associated types, 是一个将类型占位符与trait相关联的方式。
    // 这样的trait的方法签名中就可以使用这些占位符了
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait TestAssociatedTypes {
    // multi types, 那这里为什么不使用泛型呢？对于具体类型的我们实现我们的代码更加的方便
    // 如果使用泛型就显得更加的麻烦，比如整数相加和字符串拼接实现就完全不一样了。
    type Item;
    type Info;

    fn test3(&self, rhs: Self::Info) -> Self::Item;
}

impl TestAssociatedTypes for Test {
    type Item = u32;
    type Info = u32;

    fn test3(&self, rhs: Self::Info) -> Self::Item {
        rhs + 12
    }
}

// 泛型，参数多态，参数支持多种数据类型
// 泛型并不是运行时消耗，Rust在编译的时候就会将泛型代码编译成为单态化，使用具体的类型，将通用代码转化为特定代码
trait TestGeneric<T> {
    // 是&self, 不是self，不然会moved
    fn test2(&self, rhs: T) -> T;
}

impl TestGeneric<u32> for Test {
    fn test2(&self, rhs: u32) -> u32 {
        rhs + 3
    }
}
