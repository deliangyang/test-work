fn main() {
    println!("2 * 2 + 2 * 2 = {}", do_twice(|x| x+x, 2));
    println!("2 * 2 + 2 * 2 = {}", do_twice(add, 2));

    println!("return closure = {}", return_closure()(3))
}

fn add(a: i32) -> i32 {
    a + a
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 闭包表现为trait，这意味着不能直接返回闭包。对于大部分需要返回trait的情况，可以使用实现了期望返回的
// trait的具体类型来代替函数的返回值，
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 2)
}