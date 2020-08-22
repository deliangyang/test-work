use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count: {}", Rc::strong_count(&a));
        println!("{:?}", c);
    }

    println!("count: {}", Rc::strong_count(&a));

    println!("{:?}", a);
    println!("{:?}", b);

}

// Rc<T> 引用计数智能指针
// 大部分的情况下所有权是非常的明确的，可以准确的是到哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有权。
// 在图数据结构中，多个边可能有相同的节点，而这个节点从概念是讲所指向它的边所有拥有。节点知道没有任何边指向它之前都
// 不应该被清理掉的，所以就引入了引用计数这个概念。Rc<T>

// Rc<T>用于当我们希望在堆上分配一些内存仅供程序的多个部分读取，而且无法在编译时确定程序的那个部分会最后结束使用它的时候。
// Rc<T> 只能用于单线程的场景，单线程共享数据
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}


