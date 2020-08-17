fn main() {
    println!("{}", String::from("hello world"));
    {
        let a = String::from("this is element a");
        let _b = a.clone();

        println!("b = {}", a);
    }

    println!("the longest is: {}", longest(
        String::from("test").as_str(),
        String::from("test2").as_str()
    ));

    println!("the longest2 is: {}", longest_2(
        String::from("test").as_str(),
        String::from("test2").as_str()
    ));

    println!("first word: {}", first_word(String::from("first word").as_str()));
}


// 所有权
// 三类
// 1、垃圾回收机制，在程序运行时不断地寻找不再使用的内存
// 2、程序员必须亲自分配和释放内存
// 3、Rust 通过所有权系统管理内存，编译器在编译的时候会根据一系列规则进行检查，在运行时，所有权的任何功能都不会减慢。
// 栈 Stack && 堆 Heap
// 栈 已知大小
// 堆 编译时大小未知或者大小可能变化的数据，缺乏组织性
// 入栈比在堆上分配内存要快。访问堆上的数据比访问栈上的数据慢


// 引用一个生命周期比自己短的变量，程序拒绝编译。

// 函数或者方法的参数的生命周期->输入生命周期（input lifetimes）
// 而返回值的生命周期被称为->输出生命周期（output lifetimes）
// 如下三条规则判断引用何时不需要明确的注解
// 第一条适用于输入生命周期，后两天适用于输出生命周期, 超越了这三个规则编译器会报错，适用规则的包含，fn定义，以及impl块
// 1. 每一个引用参数都有它自己的生命周期参数。
//      fn foo<'a>(x: &'a i32) or foo<'a, 'b>(x: &'a i32, y: &'b i32)

fn first_word(s: &str) -> &str {
    s
}

// 等价于
//fn first_word_2<'a>(s: &'a str) -> &'a str {
//    s
//}

// 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
//      fn foo<'a>(x: &'a i32) -> &'a i32

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 两个不同的生命周期
//fn longest_2<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}

// 3. 多个输入生命周期参数并且其中一个参数是&self或者&mut self, 说明是对象的方法，那么所有输出生命周期参数被赋予self的生命周期
//      第三条规则使得方法更容易读写，因为只需更少的符号