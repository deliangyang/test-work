fn main() {
    test_new();
}

// Boxes为智能指针，拥有额外的元数据和功能，引用计数
// 其值是一个指向被分配在堆上的值5的Box
// 允许创建递归类型
// https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html
fn test_new() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Rc<T> 引用计数智能指针

// RefCell<T> 和内部可变性模式  => mut?

