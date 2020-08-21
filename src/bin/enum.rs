enum Test {
    T,
    B,
    C,
}

fn main() {
    test(Test::T);
    test(Test::B);
    test(Test::C);
}

fn test(t: Test) {
    match t {
        Test::T => {
            println!("this is T")
        },
        Test::B => {
            println!("this is B")
        },
        Test::C => {
            println!("this is C")
        },
    }
}