extern crate test_work;
use test_work::HelloMacro;
use test_work::show_streams;

fn main() {
    let b = vec![1, 2, 3];
    println!("{:?}", b);


    println!("answer(): {}", answer());

    test();
}


// 宏是一种为写其他代码而写代码的方式，即所谓的"元编程"，
#[macro_export]
macro_rules! vec {
    ( $( $x:expr),* ) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*
            tmp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Struct {}


#[show_streams{ test }]
fn test() {

}
